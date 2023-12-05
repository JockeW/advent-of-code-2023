pub fn part_one(input: &str) -> u64 {
    let lines = input
        .trim()
        .split('\n')
        .collect::<Vec<&str>>()
        .iter()
        .map(|&l| l.trim())
        .collect::<Vec<&str>>();
    let seeds: Vec<u64> = lines[0].split(":").collect::<Vec<&str>>()[1]
        .trim()
        .split(" ")
        .collect::<Vec<&str>>()
        .iter()
        .map(|&s| s.parse::<u64>().unwrap())
        .collect();

    let mut locations: Vec<u64> = Vec::new();

    for seed in seeds {
        let mut mapping_found = false;
        let mut source = seed;
        let mut destination = seed;
        for (index, &line) in lines[2..lines.len()].iter().enumerate() {
            if line.is_empty() || index == lines[2..lines.len()].len() - 1 {
                if mapping_found == false {
                    destination = source;
                    println!("Destination (same as source): {}", destination);
                }

                continue;
            }

            if line
                .chars()
                .collect::<Vec<char>>()
                .first()
                .unwrap()
                .is_digit(10)
                == false
            {
                println!("New mapping: {}", line);
                source = destination;
                destination = 0;
                mapping_found = false;
            } else if mapping_found {
                continue;
            } else {
                let map_numbers: Vec<u64> = line
                    .split(" ")
                    .collect::<Vec<&str>>()
                    .iter()
                    .map(|&m| m.parse::<u64>().unwrap())
                    .collect();

                let source_map = map_numbers[1];
                let destination_map = map_numbers[0];
                let map_range = map_numbers[2];

                let source_in_map = source >= source_map && source <= source_map + map_range;
                if !source_in_map {
                    continue;
                }

                let difference = destination_map.abs_diff(source_map);

                destination = if source_map < destination_map {
                    source + difference
                } else if source_map > destination_map {
                    source - difference
                } else {
                    source
                };

                mapping_found = true;
            }
        }

        locations.push(destination);
    }

    *locations.iter().min().unwrap()
}

pub fn part_two(input: &str) -> u64 {
    let lines = input
        .trim()
        .split('\n')
        .collect::<Vec<&str>>()
        .iter()
        .map(|&l| l.trim())
        .collect::<Vec<&str>>();
    let seed_ranges: Vec<u64> = lines[0].split(":").collect::<Vec<&str>>()[1]
        .trim()
        .split(" ")
        .collect::<Vec<&str>>()
        .iter()
        .map(|&s| s.parse::<u64>().unwrap())
        .collect();

    let mut seeds: Vec<u64> = Vec::new();
    let mut ranges: Vec<u64> = Vec::new();
    for (i, seed_data) in seed_ranges.iter().enumerate() {
        if i % 2 == 0 {
            seeds.push(*seed_data);
        } else {
            ranges.push(*seed_data);
        }
    }

    let mut locations: Vec<u64> = Vec::new();

    for (seed_index, seed_start) in seeds.into_iter().enumerate() {
        for seed in seed_start..seed_start + ranges[seed_index] {
            let mut mapping_found = false;
            let mut source = seed;
            let mut destination = seed;
            for (index, &line) in lines[2..lines.len()].iter().enumerate() {
                if line.is_empty() || index == lines[2..lines.len()].len() - 1 {
                    if mapping_found == false {
                        destination = source;
                    }

                    continue;
                }

                if line
                    .chars()
                    .collect::<Vec<char>>()
                    .first()
                    .unwrap()
                    .is_digit(10)
                    == false
                {
                    source = destination;
                    destination = 0;
                    mapping_found = false;
                } else if mapping_found {
                    continue;
                } else {
                    let map_numbers: Vec<u64> = line
                        .split(" ")
                        .collect::<Vec<&str>>()
                        .iter()
                        .map(|&m| m.parse::<u64>().unwrap())
                        .collect();

                    let source_map = map_numbers[1];
                    let destination_map = map_numbers[0];
                    let map_range = map_numbers[2];

                    let source_in_map = source >= source_map && source <= source_map + map_range;
                    if !source_in_map {
                        continue;
                    }

                    let difference = destination_map.abs_diff(source_map);

                    destination = if source_map < destination_map {
                        source + difference
                    } else if source_map > destination_map {
                        source - difference
                    } else {
                        source
                    };

                    mapping_found = true;
                }
            }

            locations.push(destination);
        }
    }

    *locations.iter().min().unwrap()
}

#[cfg(test)]
mod tests {
    // #[test]
    // fn example() {
    //     assert_eq!(super::part_one(include_str!("example.txt")), 35);
    // }

    // #[test]
    // fn part_one() {
    //     assert_eq!(super::part_one(include_str!("input.txt")), 26273516);
    // }

    #[test]
    fn example2() {
        assert_eq!(super::part_two(include_str!("example.txt")), 46);
    }

    #[test]
    fn part_two() {
        assert_eq!(super::part_two(include_str!("input.txt")), 0);
    }
}
