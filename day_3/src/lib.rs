struct Number {
    value: usize,
    x_start: usize,
    x_end: usize,
    y: usize,
}

pub fn part_one(input: &str) -> usize {
    let numbers: Vec<Number> = Vec::new();
    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in input.trim().split('\n') {
        let mut grid_line: Vec<char> = Vec::new();
        let mut number: String = String::new();
        for c in line.chars() {
            grid_line.push(c);
            // if c.is_digit(10) {
            //     number.push(c);
            // } else if number.len() > 0 {

            // }
        }
        grid.push(grid_line);
    }

    for row in grid {
        for c in row {
            
        }
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
