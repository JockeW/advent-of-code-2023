use std::collections::HashMap;

struct Node {}

pub fn part_one(input: &str) -> usize {
    let lines = input.trim().split('\n').collect::<&str>();
    let instructions: &str;
    let nodes: HashMap<&str, (&str, &str)> = HashMap::new();
    
    for line in input.trim().split('\n') {
        
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

    #[test]
    fn part_one() {
        assert_eq!(super::part_one(include_str!("input.txt")), 0);
    }

    #[test]
    fn part_two() {
        assert_eq!(super::part_two(include_str!("input.txt")), 0);
    }
}