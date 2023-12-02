use std::collections::HashMap;

pub struct Game {
    Id: String,
    Colors: HashMap<String, usize>,
}

pub fn part_one(input: &str) -> usize {
    let mut games: Vec<Game> = Vec::new();

    for line in input.trim().split('\n') {
        let line_parts = line.split(":").collect::<Vec<&str>>();

        let mut game = Game {
            Id: line_parts[0]
                .split(" ")
                .map(String::from)
                .collect::<Vec<String>>()[1]
                .clone(),
            Colors: HashMap::new(),
        };
        let subsets_string = line_parts[1];
        let subsets = subsets_string
            .split(";")
            .map(|s| s.trim())
            .collect::<Vec<&str>>();
        //println!("{:?}", subsets);

        for subset in subsets {
            let cubes = subset.split(",").map(|s| s.trim()).collect::<Vec<&str>>();
            for cube in cubes {
                let cube_parts = cube.split(" ").collect::<Vec<&str>>();
                let cube_color = cube_parts[1];
                let cube_value = cube_parts[0].parse::<usize>().unwrap();

                game.Colors
                    .entry(cube_color.to_owned())
                    .and_modify(|e| *e += cube_value)
                    .or_insert(cube_value);
            }
        }
    }
    1
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
        assert_eq!(super::part_one(include_str!("example.txt")), 0);
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
