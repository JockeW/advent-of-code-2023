use std::collections::{BTreeMap, HashMap};

pub fn part_one(input: &str) -> usize {
    let mut all_card_points: Vec<usize> = Vec::new();

    for line in input.trim().split('\n') {
        let mut card_points: usize = 0;
        let card = line.split(":").collect::<Vec<&str>>()[1];
        let numbers = card.split("|").collect::<Vec<&str>>();
        let winning_numbers: Vec<&str> = numbers[0]
            .trim()
            .split(" ")
            .collect::<Vec<&str>>()
            .iter()
            .filter(|&&n| n != "")
            .map(|n| *n)
            .collect();

        let my_numbers: Vec<&str> = numbers[1]
            .trim()
            .split(" ")
            .collect::<Vec<&str>>()
            .iter()
            .filter(|&&n| n != "")
            .map(|n| *n)
            .collect();

        for num in winning_numbers {
            if my_numbers.contains(&num) {
                if card_points == 0 {
                    card_points = 1
                } else {
                    card_points *= 2;
                }
            }
        }

        all_card_points.push(card_points);
    }

    all_card_points.iter().sum()
}

pub fn part_two(input: &str) -> usize {
    let mut cards: BTreeMap<u32, &str> = BTreeMap::new();
    let mut cards_left: Vec<u32> = Vec::new();

    for line in input.trim().split('\n') {
        let card = line.split(":").collect::<Vec<&str>>();
        let card_id_str = card[0];
        //let card_id_parts = card_id_str.split(" ").collect::<Vec<&str>>();
        let card_id = card_id_str
            .split(" ")
            .collect::<Vec<&str>>()
            .last()
            .unwrap()
            .parse()
            .unwrap();
        let card_numbers = card[1];
        cards.insert(card_id, card_numbers);
        cards_left.push(card_id);
    }
    //println!("Hash: {:?}", cards);
    //println!("Cards left start: {:?}", cards_left);

    for (card, values) in cards.iter() {
        let mut count: usize = cards_left
            .iter()
            .filter(|&c| c == card)
            .collect::<Vec<&u32>>()
            .len();
        println!("card: {}. count: {}", card, count);

        while count > 0 {
            let matching_numbers = check_card(values); //TODO: Don't need to do this everytime. One time per card should be enough, than we know the value here
            //println!("Matches: {}", matching_numbers);
            let start = card + 1;
            let end = start + matching_numbers;
            for i in start..end {
                cards_left.push(i);
            }

            cards_left.sort();
            //println!("Cards left: {:?}", cards_left);
            count -= 1;
        }
    }

    cards_left.sort();
    //println!("Cards left: {:?}", cards_left);

    cards_left.len()
}

fn check_card(card_numbers: &str) -> u32 {
    let mut card_points: u32 = 0;
    //println!("Numbers: {:?}", card_numbers);

    let numbers = card_numbers.split("|").collect::<Vec<&str>>();
    let winning_numbers: Vec<&str> = numbers[0]
        .trim()
        .split(" ")
        .collect::<Vec<&str>>()
        .iter()
        .filter(|&&n| n != "")
        .map(|n| *n)
        .collect();

    let my_numbers: Vec<&str> = numbers[1]
        .trim()
        .split(" ")
        .collect::<Vec<&str>>()
        .iter()
        .filter(|&&n| n != "")
        .map(|n| *n)
        .collect();

    for num in winning_numbers {
        if my_numbers.contains(&num) {
            card_points += 1;
        }
    }

    card_points
}

#[cfg(test)]
mod tests {
    // #[test]
    // fn example() {
    //     assert_eq!(super::part_one(include_str!("example.txt")), 13);
    // }

    // #[test]
    // fn part_one() {
    //     assert_eq!(super::part_one(include_str!("input.txt")), 23235);
    // }

    // #[test]
    // fn example2() {
    //     assert_eq!(super::part_two(include_str!("example.txt")), 30);
    // }

    #[test]
    fn part_two() {
        assert_eq!(super::part_two(include_str!("input.txt")), 0);
    }
}
