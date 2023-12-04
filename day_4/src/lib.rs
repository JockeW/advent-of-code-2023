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
    for line in input.trim().split('\n') {
        //
    }

    1
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

    // #[test]
    // fn part_two() {
    //     assert_eq!(super::part_two(include_str!("input.txt")), 0);
    // }
}
