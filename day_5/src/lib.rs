pub fn part_one(input: &str) -> usize {
    let seeds: Vec<u32> = Vec::new();
    let seed_to_soil: Vec<[u32; 3]> = Vec::new();
    let soil_to_fertilizer: Vec<[u32; 3]> = Vec::new();
    let fertilizer_to_water: Vec<[u32; 3]> = Vec::new();
    let water_to_light: Vec<[u32; 3]> = Vec::new();
    let light_to_temperature: Vec<[u32; 3]> = Vec::new();
    let temperature_to_humidity: Vec<[u32; 3]> = Vec::new();
    let humidity_to_location: Vec<[u32; 3]> = Vec::new();

    for line in input.trim().split('\n') {
        let seed_line = line.split(":").collect::<Vec<&str>>()[1].trim();
        seeds.append(seed_line.split(" ").collect::<Vec<&str>>().iter().map(|&s| s.parse()).collect());
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
        assert_eq!(super::part_one(include_str!("example.txt")), 35);
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