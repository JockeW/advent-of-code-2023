use std::collections::HashMap;

pub fn part_one(input: &str) -> usize {
    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in input.trim().split('\n') {
        let mut grid_line: Vec<char> = Vec::new();
        for c in line.trim().chars() {
            grid_line.push(c);
        }
        grid.push(grid_line);
    }

    let mut part_numbers: Vec<usize> = Vec::new();

    for (y, row) in grid.iter().enumerate() {
        let mut number_string: String = String::new();

        for (x, c) in row.iter().enumerate() {
            if c.is_digit(10) {
                number_string.push(*c);

                if x == row.len() - 1 {
                    let adjecents =
                        get_adjecents_for_number(x + 1, y, grid.len(), number_string.len(), true);

                    for adj in adjecents {
                        let c = grid[adj.1][adj.0];
                        if !c.is_digit(10) && c != '.' {
                            part_numbers.push(number_string.parse::<usize>().unwrap());
                            break;
                        }
                    }
                }
            } else if number_string.len() > 0 {
                let adjecents =
                    get_adjecents_for_number(x, y, grid.len(), number_string.len(), false);

                for adj in adjecents {
                    let c = grid[adj.1][adj.0];
                    if !c.is_digit(10) && c != '.' {
                        part_numbers.push(number_string.parse::<usize>().unwrap());
                        break;
                    }
                }

                number_string = String::new();
            }
        }
    }

    part_numbers.iter().sum()
}

fn get_adjecents_for_number(
    x: usize,
    y: usize,
    num_of_rows: usize,
    number_lenght: usize,
    end_of_line: bool,
) -> Vec<(usize, usize)> {
    let mut adjecents: Vec<(usize, usize)> = Vec::new();
    if x - number_lenght != 0 {
        adjecents.push((x - number_lenght - 1, y));
        if y != 0 {
            adjecents.push((x - number_lenght - 1, y - 1));
        }
        if y != num_of_rows - 1 {
            adjecents.push((x - number_lenght - 1, y + 1));
        }
    }

    if !end_of_line {
        adjecents.push((x, y));
        if y != num_of_rows - 1 {
            adjecents.push((x, y + 1));
        }
        if y != 0 {
            adjecents.push((x, y - 1));
        }
    }

    let x_start = x - number_lenght;
    let x_end = if end_of_line { x } else { x + 1 };

    for a in x_start..x_end {
        if y != 0 {
            adjecents.push((a, y - 1));
        }
        if y != num_of_rows - 1 {
            adjecents.push((a, y + 1));
        }
    }

    adjecents
}

pub fn part_two(input: &str) -> usize {
    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in input.trim().split('\n') {
        let mut grid_line: Vec<char> = Vec::new();
        for c in line.trim().chars() {
            grid_line.push(c);
        }
        grid.push(grid_line);
    }

    let mut gears_positions: HashMap<(usize, usize), Vec<usize>> = HashMap::new();

    for (y, row) in grid.iter().enumerate() {
        let mut number_string: String = String::new();

        for (x, c) in row.iter().enumerate() {
            if c.is_digit(10) {
                number_string.push(*c);

                if x == row.len() - 1 {
                    let adjecents =
                        get_adjecents_for_number(x + 1, y, grid.len(), number_string.len(), true);

                    for adj in adjecents {
                        let c = grid[adj.1][adj.0];
                        if c == '*' {
                            gears_positions
                                .entry((adj.0, adj.1))
                                .and_modify(|e| e.push(number_string.parse::<usize>().unwrap()))
                                .or_insert(vec![number_string.parse::<usize>().unwrap()]);

                            break;
                        }
                    }
                }
            } else if number_string.len() > 0 {
                let adjecents =
                    get_adjecents_for_number(x, y, grid.len(), number_string.len(), false);

                for adj in adjecents {
                    let c = grid[adj.1][adj.0];
                    if c == '*' {
                        gears_positions
                            .entry((adj.0, adj.1))
                            .and_modify(|e| e.push(number_string.parse::<usize>().unwrap()))
                            .or_insert(vec![number_string.parse::<usize>().unwrap()]);

                        break;
                    }
                }

                number_string = String::new();
            }
        }
    }

    let number_gears = gears_positions
        .iter()
        .filter(|g| g.1.len() == 2)
        .map(|g| g.1)
        .collect::<Vec<&Vec<usize>>>();

    let mut sum_of_ratios = 0;

    for adjecent_numbers in number_gears {
        let gear_reatio: usize = adjecent_numbers.iter().product();
        sum_of_ratios += gear_reatio;
    }

    sum_of_ratios
}

#[cfg(test)]
mod tests {
    #[test]
    fn example() {
        assert_eq!(super::part_one(include_str!("example.txt")), 4361);
    }

    #[test]
    fn part_one() {
        assert_eq!(super::part_one(include_str!("input.txt")), 536576);
    }

    #[test]
    fn example2() {
        assert_eq!(super::part_two(include_str!("example.txt")), 467835);
    }

    #[test]
    fn part_two() {
        assert_eq!(super::part_two(include_str!("input.txt")), 75741499);
    }
}
