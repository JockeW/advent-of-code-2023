use std::collections::HashMap;

struct Pipe {
    c: char,
    x: usize,
    y: usize,
}

pub fn part_one(input: &str) -> usize {
    let mut grid: HashMap<(i32, i32), char> = HashMap::new();

    // let grid: Vec<Vec<char>> = input
    //     .trim()
    //     .split('\n')
    //     .map(|line| line.trim().chars().collect::<Vec<char>>())
    //     .collect();

    for (y, line) in input.trim().split('\n').enumerate() {
        for (x, char) in line.trim().chars().enumerate() {
            grid.insert((x.try_into().unwrap(), y.try_into().unwrap()), char);
        }
    }

    //println!("Grid: {:?}", grid);

    let start = grid.iter().find(|p| *p.1 == 'S').unwrap();
    let west_neighbor = grid.get(&(start.0 .0 - 1, start.0 .1));
    let east_neighbor = grid.get(&(start.0 .0 + 1, start.0 .1));
    let north_neighbor = grid.get(&(start.0 .0, start.0 .1 - 1));
    let south_neighbor = grid.get(&(start.0 .0, start.0 .1 + 1));

    // println!("West neighbor: {:?}", west_neighbor);
    // println!("East neighbor: {:?}", east_neighbor);
    // println!("North neighbor: {:?}", north_neighbor);
    // println!("South neighbor: {:?}", south_neighbor);

    let mut current_pipe: ((i32, i32), char) = ((0, 0), '.');

    if let Some(n) = west_neighbor {
        if n == &'-' || n == &'F' || n == &'L' {
            current_pipe = ((start.0 .0 - 1, start.0 .1), *n);
        }
    } else if let Some(n) = east_neighbor {
        if n == &'-' || n == &'7' || n == &'J' {
            current_pipe = ((start.0 .0 + 1, start.0 .1), *n);
        }
    } else if let Some(n) = north_neighbor {
        if n == &'|' || n == &'F' || n == &'7' {
            current_pipe = ((start.0 .0, start.0 .1 - 1), *n);
        }
    } else if let Some(n) = south_neighbor {
        if n == &'|' || n == &'L' || n == &'J' {
            current_pipe = ((start.0 .0, start.0 .1 + 1), *n);
        } else {
            panic!("NO VALID NEIGHBOR");
        }
    } else {
        panic!("NO VALID NEIGHBOR");
    }

    //println!("current_pipe: {:?}", current_pipe);
    let mut previous_pipe = ((start.0.0, start.0.1), *start.1);

    let mut steps = 2;

    //Go full loop, divide by two
    loop {
        println!("Steps: {}", steps);
        println!("current_pipe: {:?}", current_pipe);
        // println!("previous_pipe: {:?}", previous_pipe);
        let next_pipe = get_connected_pipe(current_pipe, previous_pipe, &grid);

        if next_pipe.0 .0 == start.0 .0 && next_pipe.0 .1 == start.0 .1 {
            break;
        }

        steps += 1;
        previous_pipe = current_pipe;
        current_pipe = next_pipe;
    }

    steps / 2
}

fn get_connected_pipe(
    current_pipe: ((i32, i32), char),
    previous_pipe: ((i32, i32), char),
    grid: &HashMap<(i32, i32), char>,
) -> ((i32, i32), char) {
    //TODO: Get pipe going out from current_pipe. We came from previous_pipe
    let west_neighbor = if previous_pipe.0 != (current_pipe.0 .0 - 1, current_pipe.0 .1) {
        grid.get(&(current_pipe.0 .0 - 1, current_pipe.0 .1))
    } else {
        None
    };
    let east_neighbor = if previous_pipe.0 != (current_pipe.0 .0 + 1, current_pipe.0 .1) {
        grid.get(&(current_pipe.0 .0 + 1, current_pipe.0 .1))
    } else {
        None
    };
    let north_neighbor = if previous_pipe.0 != (current_pipe.0 .0, current_pipe.0 .1 - 1) {
        grid.get(&(current_pipe.0 .0, current_pipe.0 .1 - 1))
    } else {
        None
    };
    let south_neighbor = if previous_pipe.0 != (current_pipe.0 .0, current_pipe.0 .1 + 1) {
        grid.get(&(current_pipe.0 .0, current_pipe.0 .1 + 1))
    } else {
        None
    };

    if let Some(n) = west_neighbor {
        if n == &'-' || n == &'F' || n == &'L' || n == &'S' {
            return ((current_pipe.0 .0 - 1, current_pipe.0 .1), *n);
        }
    }

    if let Some(n) = east_neighbor {
        if n == &'-' || n == &'7' || n == &'J' || n == &'S' {
            return ((current_pipe.0 .0 + 1, current_pipe.0 .1), *n);
        }
    }

    if let Some(n) = north_neighbor {
        if n == &'|' || n == &'F' || n == &'7' || n == &'S' {
            return ((current_pipe.0 .0, current_pipe.0 .1 - 1), *n);
        }
    }

    if let Some(n) = south_neighbor {
        if n == &'|' || n == &'L' || n == &'J' || n == &'S' {
            return ((current_pipe.0 .0, current_pipe.0 .1 + 1), *n);
        } else {
            println!("Current: {:?}", current_pipe);
            panic!("NO VALID NEIGHBOR");
        }
    } else {
        println!("Current: {:?}", current_pipe);
        panic!("NO VALID NEIGHBOR");
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
    // #[test]
    // fn example() {
    //     assert_eq!(super::part_one(include_str!("example.txt")), 8);
    // }

    #[test]
    fn part_one() {
        assert_eq!(super::part_one(include_str!("input.txt")), 0);
    }

    // #[test]
    // fn part_two() {
    //     assert_eq!(super::part_two(include_str!("input.txt")), 0);
    // }
}
