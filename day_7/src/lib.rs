use std::{cmp::Ordering, collections::HashMap};

const CARD_VALUES: [char; 13] = [
    'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
];

const CARD_VALUES_WITH_JOKER: [char; 13] = [
    'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
];

pub fn part_one(input: &str) -> u32 {
    let mut hands: HashMap<&str, Vec<(&str, u32)>> = HashMap::new();
    hands.insert("Five of a kind", Vec::new());
    hands.insert("Four of a kind", Vec::new());
    hands.insert("Full house", Vec::new());
    hands.insert("Three of a kind", Vec::new());
    hands.insert("Two pair", Vec::new());
    hands.insert("One pair", Vec::new());
    hands.insert("High card", Vec::new());

    let rank: u32 = input.trim().split('\n').count().try_into().unwrap();

    for line in input.trim().split('\n') {
        let cards_with_bid = line.trim().split(" ").collect::<Vec<&str>>();
        let card_type = get_type_for_hand(cards_with_bid[0], false);

        hands
            .entry(card_type)
            .and_modify(|e| e.push((cards_with_bid[0], cards_with_bid[1].parse::<u32>().unwrap())));
    }

    for value in hands.values_mut() {
        value.sort_by(|a, b| hands_compare(a.0, b.0, false));
    }

    get_total_winnings(hands, rank)
}

fn get_total_winnings(hands: HashMap<&str, Vec<(&str, u32)>>, mut rank: u32) -> u32 {
    let mut total_winnings = 0;

    for hand in hands.get("Five of a kind").unwrap() {
        total_winnings = total_winnings + (hand.1 * rank);
        rank -= 1;
    }
    for hand in hands.get("Four of a kind").unwrap() {
        total_winnings = total_winnings + (hand.1 * rank);
        rank -= 1;
    }
    for hand in hands.get("Full house").unwrap() {
        total_winnings = total_winnings + (hand.1 * rank);
        rank -= 1;
    }
    for hand in hands.get("Three of a kind").unwrap() {
        total_winnings = total_winnings + (hand.1 * rank);
        rank -= 1;
    }
    for hand in hands.get("Two pair").unwrap() {
        total_winnings = total_winnings + (hand.1 * rank);
        rank -= 1;
    }
    for hand in hands.get("One pair").unwrap() {
        total_winnings = total_winnings + (hand.1 * rank);
        rank -= 1;
    }
    for hand in hands.get("High card").unwrap() {
        total_winnings = total_winnings + (hand.1 * rank);
        rank -= 1;
    }

    total_winnings
}

fn hands_compare(a: &str, b: &str, include_jokers: bool) -> Ordering {
    let mut b_iterator = b.chars();
    for a_char in a.chars() {
        let b_char = b_iterator.next().unwrap();

        let a_char_pos;
        let b_char_pos;

        if include_jokers {
            a_char_pos = CARD_VALUES_WITH_JOKER
                .iter()
                .position(|v| *v == a_char)
                .unwrap();
            b_char_pos = CARD_VALUES_WITH_JOKER
                .iter()
                .position(|v| *v == b_char)
                .unwrap();
        } else {
            a_char_pos = CARD_VALUES.iter().position(|v| *v == a_char).unwrap();
            b_char_pos = CARD_VALUES.iter().position(|v| *v == b_char).unwrap();
        }

        if a_char_pos > b_char_pos {
            return Ordering::Greater;
        } else if a_char_pos < b_char_pos {
            return Ordering::Less;
        }
    }

    Ordering::Equal
}

fn get_type_for_hand(hand: &str, include_jokers: bool) -> &str {
    let mut same_card_map: HashMap<char, u8> = HashMap::new();
    for card in hand.chars() {
        same_card_map
            .entry(card)
            .and_modify(|c| *c += 1)
            .or_insert(1);
    }

    let hand_type: &str;

    if same_card_map.len() == 1 {
        hand_type = "Five of a kind";
    } else if same_card_map.len() == 2 {
        if same_card_map.iter().any(|c| *c.1 == 3) {
            hand_type = "Full house";
        } else {
            hand_type = "Four of a kind";
        }
    } else if same_card_map.len() == 3 {
        if same_card_map.iter().any(|c| *c.1 == 3) {
            hand_type = "Three of a kind";
        } else {
            hand_type = "Two pair";
        }
    } else if same_card_map.len() == 4 {
        hand_type = "One pair";
    } else {
        hand_type = "High card";
    }

    if include_jokers {
        let jokers = same_card_map.get(&'J');
        let num_of_jokers = match jokers {
            Some(j) => *j,
            None => 0,
        };
        get_type_with_jokers(&hand_type, num_of_jokers)
    } else {
        &hand_type
    }
}

fn get_type_with_jokers(type_without_jokers: &str, num_of_jokers: u8) -> &str {
    match type_without_jokers {
        "Five of a kind" => "Five of a kind",
        "Four of a kind" => {
            if num_of_jokers >= 1 {
                "Five of a kind"
            } else {
                "Four of a kind"
            }
        }
        "Full house" => {
            if num_of_jokers >= 2 {
                "Five of a kind"
            } else {
                "Full house"
            }
        }
        "Three of a kind" => {
            if num_of_jokers == 3 {
                "Four of a kind"
            } else if num_of_jokers == 2 {
                "Five of a kind"
            } else if num_of_jokers == 1 {
                "Four of a kind"
            } else {
                "Three of a kind"
            }
        }
        "Two pair" => {
            if num_of_jokers == 2 {
                "Four of a kind"
            } else if num_of_jokers == 1 {
                "Full house"
            } else {
                "Two pair"
            }
        }
        "One pair" => {
            if num_of_jokers == 1 {
                "Three of a kind"
            } else if num_of_jokers == 2 {
                "Three of a kind"
            } else {
                "One pair"
            }
        }
        "High card" => {
            if num_of_jokers == 1 {
                "One pair"
            } else {
                "High card"
            }
        }
        _ => panic!("Not a valid type!"),
    }
}

pub fn part_two(input: &str) -> u32 {
    let mut hands: HashMap<&str, Vec<(&str, u32)>> = HashMap::new();
    hands.insert("Five of a kind", Vec::new());
    hands.insert("Four of a kind", Vec::new());
    hands.insert("Full house", Vec::new());
    hands.insert("Three of a kind", Vec::new());
    hands.insert("Two pair", Vec::new());
    hands.insert("One pair", Vec::new());
    hands.insert("High card", Vec::new());

    let rank: u32 = input.trim().split('\n').count().try_into().unwrap();

    for line in input.trim().split('\n') {
        let cards_with_bid = line.trim().split(" ").collect::<Vec<&str>>();
        let card_type = get_type_for_hand(cards_with_bid[0], true);

        hands
            .entry(card_type)
            .and_modify(|e| e.push((cards_with_bid[0], cards_with_bid[1].parse::<u32>().unwrap())));
    }

    for value in hands.values_mut() {
        value.sort_by(|a, b| hands_compare(a.0, b.0, true));
    }

    get_total_winnings(hands, rank)
}

#[cfg(test)]
mod tests {
    #[test]
    fn example() {
        assert_eq!(super::part_one(include_str!("example.txt")), 6440);
    }

    #[test]
    fn part_one() {
        assert_eq!(super::part_one(include_str!("input.txt")), 251058093);
    }

    #[test]
    fn example2() {
        assert_eq!(super::part_two(include_str!("example.txt")), 5905);
    }

    #[test]
    fn part_two() {
        assert_eq!(super::part_two(include_str!("input.txt")), 249781879);
    }
}
