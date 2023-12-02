use std::collections::HashMap;

pub fn part_one(input: &str) -> i32 {
    let mut sum: i32 = 0;

    'game: for line in input.trim().split('\n') {
        let line_parts = line.split(":").collect::<Vec<&str>>();

        let game_id = line_parts[0]
            .split(" ")
            .map(String::from)
            .collect::<Vec<String>>()[1]
            .clone()
            .parse::<i32>()
            .unwrap();

        let subsets_string = line_parts[1];
        let subsets = subsets_string
            .split(";")
            .map(|s| s.trim())
            .collect::<Vec<&str>>();

        for subset in subsets {
            let mut subset_possible = true;

            let cubes = subset.split(",").map(|s| s.trim()).collect::<Vec<&str>>();
            for cube in cubes {
                let cube_parts = cube.split(" ").collect::<Vec<&str>>();
                let cube_color = cube_parts[1];
                let cube_value = cube_parts[0].parse::<usize>().unwrap();

                match cube_color {
                    "red" => {
                        if cube_value > 12 {
                            subset_possible = false
                        }
                    }
                    "green" => {
                        if cube_value > 13 {
                            subset_possible = false
                        }
                    }
                    "blue" => {
                        if cube_value > 14 {
                            subset_possible = false
                        }
                    }
                    _ => panic!("Invalid color"),
                }
            }

            if !subset_possible {
                continue 'game;
            }
        }

        sum += game_id;
    }

    sum
}

pub fn part_two(input: &str) -> usize {
    let mut powers: Vec<usize> = Vec::new();

    for line in input.trim().split('\n') {
        let line_parts = line.split(":").collect::<Vec<&str>>();

        let subsets_string = line_parts[1];
        let subsets = subsets_string
            .split(";")
            .map(|s| s.trim())
            .collect::<Vec<&str>>();

        let mut colors: HashMap<&str, usize> = HashMap::new();
        colors.insert("red", 0);
        colors.insert("green", 0);
        colors.insert("blue", 0);

        for subset in subsets {
            let cubes = subset.split(",").map(|s| s.trim()).collect::<Vec<&str>>();
            for cube in cubes {
                let cube_parts = cube.split(" ").collect::<Vec<&str>>();
                let cube_color = cube_parts[1];
                let cube_value = cube_parts[0].parse::<usize>().unwrap();

                let saved_cube_value = colors.get(cube_color).unwrap();
                if cube_value > *saved_cube_value {
                    colors.insert(cube_color, cube_value);
                }
            }
        }

        let power: usize = colors
            .values()
            .cloned()
            .collect::<Vec<usize>>()
            .iter()
            .product();
        powers.push(power);
    }

    powers.iter().sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn example() {
        assert_eq!(super::part_one(include_str!("example.txt")), 8);
    }

    #[test]
    fn part_one() {
        assert_eq!(super::part_one(include_str!("input.txt")), 2486);
    }

    #[test]
    fn example2() {
        assert_eq!(super::part_two(include_str!("example.txt")), 2286);
    }

    #[test]
    fn part_two() {
        assert_eq!(super::part_two(include_str!("input.txt")), 87984);
    }
}
