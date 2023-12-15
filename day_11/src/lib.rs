use std::arch::x86_64;

struct Galaxy {
    name: u32,
    x: usize,
    y: usize,
}

pub fn part_one(input: &str) -> usize {
    let mut image: Vec<(usize, usize, char)> = Vec::new();
    let mut expanded_image: Vec<(usize, usize, char)> = Vec::new();
    let mut galaxy_name = 0;
    //First expand universe
    for (y, line) in input.trim().split('\n').enumerate() {
        for (x, c) in line.trim().chars().enumerate() {
            //println!("{}", c);
            if c == '#' {
                image.push((x, y, char::from_digit(galaxy_name, 10).unwrap()));
                galaxy_name += 1;
            } else {
                image.push((x, y, '.'));
            }
        }
    }
    //println!("{:?}", image);

    let mut num_of_rows = image.iter().map(|p| p.1).max().unwrap();
    let mut num_of_columns = image.iter().map(|p| p.0).max().unwrap();
    // println!("Number of rows: {}", num_of_rows + 1);
    // println!("Number of columns: {}", num_of_columns + 1);

    for row in 0..num_of_rows {
        //let mut is_empty = false;
        let spaces = image
            .iter()
            .filter(|&s| s.1 == row)
            .collect::<Vec<&(usize, usize, char)>>();

        if spaces.iter().all(|&s| s.2 == '.') {
            //is_empty = true;
            
            let mut new_row: Vec<(usize, usize, char)> = Vec::new();
            for x in 0..num_of_columns {
                new_row.push((x, row, '.'));
            }
        }
    }

    image.sort_by(|a, b| {
        if a.0 == b.0 {
            a.1.partial_cmp(&b.1).unwrap()
        } else {
            a.0.partial_cmp(&b.0).unwrap()
        }
    });
    //println!("IMAGE: {:?}", image);
    print_image(image);

    // println!("{:?}", image);
    // num_of_rows = image.iter().map(|p| p.1).max().unwrap();
    // num_of_columns = image.iter().map(|p| p.0).max().unwrap();
    // println!("Number of rows: {}", num_of_rows + 1);
    // println!("Number of columns: {}", num_of_columns + 1);

    //Second name galaxies
    //for p in image {}

    //Third collect all pairs and then get shortest path between them

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
