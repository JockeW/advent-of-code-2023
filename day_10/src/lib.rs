use std::collections::HashMap;

struct Pipe {
    c: char,
    x: usize,
    y: usize,
}

pub fn part_one(input: &str) -> usize {
    let mut grid: Vec<Pipe> = Vec::new();

    // let grid: Vec<Vec<char>> = input
    //     .trim()
    //     .split('\n')
    //     .map(|line| line.trim().chars().collect::<Vec<char>>())
    //     .collect();

    for (y, line) in input.trim().split('\n').enumerate() {
        for (x, char) in line.trim().chars().enumerate() {
            grid.push(Pipe {
                c: char,
                x: x,
                y: y,
            });
        }
    }

    //println!("Grid: {:?}", grid);

    let start = grid.iter().find(|p| p.c == 'S').unwrap();
    let start_neighbours = grid
        .iter()
        .filter(|&pipe| {
            (pipe.x == start.x + 1 && pipe.y == start.y)
                || (pipe.x == start.x - 1 && pipe.y == start.y)
                || (pipe.x == start.x && pipe.y == start.y + 1)
                || (pipe.x == start.x && pipe.y == start.y - 1)
        })
        //.map(|(k, v)| (*k, *v))
        .collect::<Vec<&Pipe>>();

    let mut dir_1: (usize, usize) = (0, 0);
    let mut dir_2: (usize, usize) = (0, 0);

    for neighbour in start_neighbours {
        if (neighbour.x == start.x + 1)
            && (neighbour.c == '-' || neighbour.c == 'J' || neighbour.c == '7')
        {
            if dir_1 != (0, 0) {
                dir_1 = (neighbour.x, neighbour.y);
            } else {
                dir_2 = (neighbour.x, neighbour.y);
            }
        }
        if (neighbour.x == start.x - 1)
            && (neighbour.c == '-' || neighbour.c == 'F' || neighbour.c == 'L')
        {
            if dir_1 != (0, 0) {
                dir_1 = (neighbour.x, neighbour.y);
            } else {
                dir_2 = (neighbour.x, neighbour.y);
            }
        }
        if (neighbour.y == start.y + 1)
            && (neighbour.c == '|' || neighbour.c == 'L' || neighbour.c == 'J')
        {
            if dir_1 != (0, 0) {
                dir_1 = (neighbour.x, neighbour.y);
            } else {
                dir_2 = (neighbour.x, neighbour.y);
            }
        }
        if (neighbour.y == start.y - 1)
            && (neighbour.c == '|' || neighbour.c == 'F' || neighbour.c == '7')
        {
            if dir_1 != (0, 0) {
                dir_1 = (neighbour.x, neighbour.y);
            } else {
                dir_2 = (neighbour.x, neighbour.y);
            }
        }
    }

    println!("Dir 1: {:?}, Dir 2: {:?}", dir_1, dir_2);

    let mut steps = 0;

    //Go in both directions from start. Where they end up on the same is fathest away
    loop {}

    0
}

fn get_connected_pipe(current_pipe: Pipe, previous_pipe: Pipe, all_pipes: Vec<Pipe>) {
    //TODO: Get pipe going out from current_pipe. We came from previous_pipe
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
        assert_eq!(super::part_one(include_str!("example.txt")), 8);
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
