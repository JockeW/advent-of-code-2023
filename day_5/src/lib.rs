use std::{
    collections::{BTreeMap, HashMap},
    ops::Range,
};

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

    *locations.iter().min().unwrap()
}

pub fn part_two(input: &str) -> u64 {
    let lines = input
        .trim()
        .split('\n')
        .collect::<Vec<&str>>()
        .iter()
        .map(|&l| l.trim())
        .filter(|&x| !x.is_empty())
        .collect::<Vec<&str>>();

    //println!("Lines: {:?}", lines);

    let seed_values: Vec<u64> = lines[0].split(":").collect::<Vec<&str>>()[1]
        .trim()
        .split(" ")
        .collect::<Vec<&str>>()
        .iter()
        .map(|&s| s.parse::<u64>().unwrap())
        .collect();

    let mut ranges: Vec<(u64, u64)> = Vec::new();

    for (i, seed) in seed_values.iter().enumerate().step_by(2) {
        ranges.push((*seed, seed + seed_values[i + 1] - 1));
    }

    let mut maps: Vec<Vec<Vec<u64>>> = lines[2..]
        .split(|line| line.chars().next().unwrap().is_ascii_alphabetic())
        //.filter(|group| !group.is_empty())
        .map(|group| {
            group
                .iter()
                .map(|line| {
                    line.split_whitespace()
                        .map(|x| x.parse::<u64>().unwrap())
                        .collect()
                })
                .collect()
        })
        .collect();

    //println!("Maps: {:?}", maps);

    maps.reverse();
    //println!("Maps reversed: {:?}", maps);

    //let mut lowest_found = false;
    let mut location_value = 0;

    loop {
        // if location_value > 10 {
        //     break;
        // }
        println!("Location value: {}", location_value);
        let mut destination_value = location_value;
        for map in &maps {
            //println!("Checking map: {:?}", map);
            destination_value = check_map_for_value(map, destination_value);
            //println!("Destination value: {}", destination_value);
        }
        // println!("Seed value: {}", destination_value);
        // println!();

        //Check if destination_value now is any of my seeds. If so, lowest location was found. Else start over with one higher location_value
        let seed_found = ranges
            .iter()
            .map(|r| r.0..r.1)
            .any(|s| s.contains(&destination_value));

        if seed_found {
            break;
        }

        location_value += 1;
    }

    location_value
}

fn check_map_for_value(map: &Vec<Vec<u64>>, value: u64) -> u64 {
    let map_range = map
        .iter()
        .filter(|&v| (v[0]..(v[0] + v[2])).contains(&value))
        .map(|v| {
            //println!("Found map: {:?}", v);
            // let difference = v[0].abs_diff(v[1]);
            // println!("Diff: {}", difference);
            let diff_from_start = value - v[0];
            let source_value = v[1] + diff_from_start;
            //println!("Source value: {}", source_value);

            return source_value;
        })
        .collect::<Vec<u64>>();
    //println!("Map range: {:?}", map_range);

    let mapping = map_range.first();

    match mapping {
        Some(a) => return *a,
        None => value,
    }
}

// fn get_maps(lines: Vec<&str>) -> Vec<Vec<u64>> {
//     let mut maps: Vec<Vec<(u64, u64, u64)>> = Vec::new();
//     // maps.insert("seed-to-soil", Vec::new());
//     // maps.insert("soil-to-fertilizer", Vec::new());
//     // maps.insert("fertilizer-to-water", Vec::new());
//     // maps.insert("water-to-light", Vec::new());
//     // maps.insert("light-to-temperature", Vec::new());
//     // maps.insert("temperature-to-humidity", Vec::new());
//     // maps.insert("humidity-to-location", Vec::new());
//     let mut map_index = 0;
//     //let mut current_map = &maps[map_index];

//     for &line in lines[3..].iter() {
//         if line
//             .chars()
//             .collect::<Vec<char>>()
//             .first()
//             .unwrap()
//             .is_digit(10)
//             == false
//         {
//             continue;
//         }

//         if line.is_empty() {
//             let ma
//             //map_index += 1;
//             //current_map = &maps[map_index];
//             continue;
//         }

//         let map_numbers: Vec<u64> = line
//             .split(" ")
//             .collect::<Vec<&str>>()
//             .iter()
//             .map(|&m| m.parse::<u64>().unwrap())
//             .collect();

//         let source_map = map_numbers[1];
//         let destination_map = map_numbers[0];
//         let map_range = map_numbers[2];

//     }

//     maps
// }

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

    // #[test]
    // fn example2() {
    //     assert_eq!(super::part_two(include_str!("example.txt")), 46);
    // }

    #[test]
    fn part_two() {
        assert_eq!(super::part_two(include_str!("input.txt")), 0);
    }
}
