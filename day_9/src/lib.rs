pub fn part_one(input: &str) -> i32 {
    let histories: Vec<Vec<i32>> = input
        .trim()
        .split('\n')
        .map(|s| {
            s.trim()
                .split(' ')
                .map(|n| n.parse().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect();

    let mut next_values: Vec<i32> = Vec::new();

    for history in histories {
        let mut sequences: Vec<Vec<i32>> = Vec::new();
        sequences.push(history);

        loop {
            let mut sequence: Vec<i32> = Vec::new();

            for (i, n) in sequences.last().unwrap().iter().enumerate() {
                if i == sequences.last().unwrap().len() - 1 {
                    break;
                }

                sequence.push(sequences.last().unwrap()[i + 1] - n);
            }

            sequences.push(sequence);

            if sequences.last().unwrap().iter().all(|&s| s == 0) {
                break;
            }
        }

        sequences.reverse();
        let mut increase_by = 0;
        for mut sequence in sequences {
            let new_sequence_number = sequence.last().unwrap() + increase_by;
            sequence.push(new_sequence_number);

            increase_by = new_sequence_number;
        }

        next_values.push(increase_by);
    }

    next_values.iter().sum()
}

pub fn part_two(input: &str) -> usize {
    for line in input.trim().split('\n') {
        //
    }
    0
}

#[cfg(test)]
mod tests {
    #[test]
    fn example() {
        assert_eq!(super::part_one(include_str!("example.txt")), 114);
    }

    #[test]
    fn part_one() {
        assert_eq!(super::part_one(include_str!("input.txt")), 1953784198);
    }

    // #[test]
    // fn part_two() {
    //     assert_eq!(super::part_two(include_str!("input.txt")), 0);
    // }
}
