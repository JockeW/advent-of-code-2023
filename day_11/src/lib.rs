struct Galaxy {
    name: u32,
    x: usize,
    y: usize,
}

pub fn part_one(input: &str) -> usize {
    let mut image: Vec<(usize, usize)> = Vec::new();
    let mut columns: Vec<u32>;
    //First expand universe
    for (y, line) in input.trim().split('\n').enumerate() {
        for (x, c) in line.trim().chars().enumerate() {
            println!("{}", c);
            image.push((x, y));
        }
    }
    println!("{:?}", image);

    let num_of_rows = image.iter().map(|p| p.1).max().unwrap();
    let num_of_columns = image.iter().map(|p| p.0).max().unwrap();
    println!("Number of rows: {}", num_of_rows + 1);
    println!("Number of columns: {}", num_of_columns + 1);

    //Second name galaxies
    let mut galaxies: Vec<Galaxy> = Vec::new();
    let mut galaxy_name = 1;
    for p in image {}

    //Third collect all pairs and then get shortest path between them

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
