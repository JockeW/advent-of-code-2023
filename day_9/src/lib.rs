pub fn part_one(input: &str) -> usize {
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

    //println!("Histories: {:?}", histories);

    for history in histories {
        //let test = history.iter().map(|n| n - n.)
        //let iterator = history.iter();

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

        println!("Sequences: {:?}", sequences);

        //TODO: extrapolate!
    }

    0
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

    // #[test]
    // fn part_one() {
    //     assert_eq!(super::part_one(include_str!("input.txt")), 0);
    // }

    // #[test]
    // fn part_two() {
    //     assert_eq!(super::part_two(include_str!("input.txt")), 0);
    // }
}
