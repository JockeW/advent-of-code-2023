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

    for (y, row) in grid.iter().enumerate() {
        let mut number_string: String = String::new();
        for (x, c) in row.iter().enumerate() {
            //let mut number_found = false;
            
            if c.is_digit(10) {
                number_string.push(*c);

                if x == row.len() - 1 {
                    //End of line. We have a full number

                    //Do stuff

                    //number_found = false;
                }
                //number_found = true;
            } else if number_string.len() > 0 {
                //We have a full number that ends on previous position
                
                //Check for symbol adjecent to any digit in number_string

                //number_found = false;
                number_string = String::new();
            }
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
