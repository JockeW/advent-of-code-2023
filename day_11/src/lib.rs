use std::arch::x86_64;

struct Galaxy {
    name: u32,
    x: usize,
    y: usize,
}

pub fn part_one(input: &str) -> usize {
    let lines: Vec<&str> = input.trim().split('\n').collect();
    let mut galaxies: Vec<Galaxy> = Vec::new();
    let mut image: Vec<(usize, usize, char)> = Vec::new();
    //let mut expanded_image: Vec<Vec<(usize, usize, char)>> = Vec::new();
    let mut galaxy_name = 0;

    let mut expanded_rows: Vec<usize> = Vec::new();
    let mut expanded_cols: Vec<usize> = Vec::new();
    //First expand universe
    for (y, &line) in lines.iter().enumerate() {
        let mut row_empty = true;
        for (x, c) in line.trim().chars().enumerate() {
            image.push((x, y, c));

            if y == 0 {
                //Check all cols (x)
                let any_galaxy_in_col = lines
                    .iter()
                    .map(|&l| l.chars().nth(x).unwrap())
                    .any(|c| c == '#');

                if !any_galaxy_in_col {
                    expanded_cols.push(x);
                }
            }

            if c == '#' {
                galaxies.push(Galaxy {
                    name: galaxy_name,
                    x,
                    y,
                });
                galaxy_name += 1;

                row_empty = false;
            }
        }

        if row_empty {
            expanded_rows.push(y);
        }
    }

    println!("Expanded rows: {:?}", expanded_rows);
    println!("Expanded cols: {:?}", expanded_cols);

    //Collect all pairs and then get shortest path between them

    0
}

fn print_image(image: Vec<(usize, usize, char)>) {
    let num_of_rows = image.iter().map(|p| p.1).max().unwrap();
    let num_of_columns = image.iter().map(|p| p.0).max().unwrap();
    for y in 0..num_of_rows {
        for x in 0..num_of_columns {
            let space = image.iter().find(|&s| s.0 == x && s.1 == y).unwrap();
            print!("{}", space.2);
        }
        println!();
    }
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
        assert_eq!(super::part_one(include_str!("example.txt")), 374);
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
