use std::collections::{BTreeMap, HashMap};

pub fn part_one(input: &str) -> usize {
    let mut all_card_points: Vec<usize> = Vec::new();

    for line in input.trim().split('\n') {
        let mut card_points: usize = 0;
        let card = line.split(":").collect::<Vec<&str>>();
        let winning_numbers = check_card(card[1]);

        for num in 0..winning_numbers {
            if num == 0 {
                card_points = 1;
            } else {
                card_points *= 2;
            }
        }

        all_card_points.push(card_points);
    }

    all_card_points.iter().sum()
}

pub fn part_two(input: &str) -> u32 {
    let mut cards: BTreeMap<u32, u32> = BTreeMap::new();
    let mut cards_left: HashMap<u32, u32> = HashMap::new();

    for line in input.trim().split('\n') {
        let card = line.split(":").collect::<Vec<&str>>();
        let card_id_str = card[0];

        let card_id = card_id_str
            .split(" ")
            .collect::<Vec<&str>>()
            .last()
            .unwrap()
            .parse()
            .unwrap();

        let card_numbers = card[1];
        let winning_numbers = check_card(card_numbers);
        cards.insert(card_id, winning_numbers);
        cards_left.insert(card_id, 1);
    }

    for (card, values) in cards.iter() {
        let mut count: u32 = *cards_left.get(card).unwrap();

        while count > 0 {
            let start = card + 1;
            let end = start + values;
            for i in start..end {
                cards_left.entry(i).and_modify(|e| *e += 1);
            }

            count -= 1;
        }
    }

    cards_left.values().sum()
}

fn check_card(card_numbers: &str) -> u32 {
    let mut card_points: u32 = 0;

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
    #[test]
    fn example() {
        assert_eq!(super::part_one(include_str!("example.txt")), 13);
    }

    #[test]
    fn part_one() {
        assert_eq!(super::part_one(include_str!("input.txt")), 23235);
    }

    #[test]
    fn example2() {
        assert_eq!(super::part_two(include_str!("example.txt")), 30);
    }

    #[test]
    fn part_two() {
        assert_eq!(super::part_two(include_str!("input.txt")), 5920640);
    }
}
