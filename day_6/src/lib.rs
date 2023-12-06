pub fn part_one(input: &str) -> usize {
    let lines: Vec<&str> = input.trim().split('\n').collect();

    let time_values = lines[0].split(":").collect::<Vec<&str>>()[1]
        .trim()
        .split(" ")
        .filter_map(|t| t.trim().parse::<u32>().ok())
        .collect::<Vec<u32>>();

    let distance_values = lines[1].split(":").collect::<Vec<&str>>()[1]
        .trim()
        .split(" ")
        .filter_map(|t| t.trim().parse::<u32>().ok())
        .collect::<Vec<u32>>();

    let races: Vec<(u32, u32)> = time_values.into_iter().zip(distance_values).collect();
    let mut beat_record: Vec<usize> = Vec::new();

    for race in races {
        let mut ways_to_win: Vec<u32> = Vec::new();
        let mut button_press_time = 1;

        while button_press_time <= race.0 {
            let distance_traveled = (race.0 - button_press_time) * button_press_time;

            if distance_traveled > race.1 {
                ways_to_win.push(button_press_time);
            }

            button_press_time += 1;
        }

        beat_record.push(ways_to_win.len())
    }

    beat_record.iter().product()
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
        assert_eq!(super::part_one(include_str!("example.txt")), 288);
    }

    #[test]
    fn part_one() {
        assert_eq!(super::part_one(include_str!("input.txt")), 449550);
    }

    // #[test]
    // fn part_two() {
    //     assert_eq!(super::part_two(include_str!("input.txt")), 0);
    // }
}
