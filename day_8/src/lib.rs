use num_integer::lcm;
use std::collections::HashMap;

fn get_nodes(lines: Vec<&str>) -> HashMap<String, (String, String)> {
    let mut nodes: HashMap<String, (String, String)> = HashMap::new();

    for &line in lines[2..].iter() {
        let line_parts = line.trim().split("=").collect::<Vec<&str>>();
        let node_id = line_parts[0].trim().to_string();
        let node_edges: Vec<String> = line_parts[1]
            .trim()
            .split(",")
            .collect::<Vec<&str>>()
            .iter()
            .map(|&edge| edge.trim().replace("(", "").replace(")", ""))
            .collect();

        nodes.insert(node_id, (node_edges[0].clone(), node_edges[1].clone()));
    }

    nodes
}

pub fn part_one(input: &str) -> usize {
    let lines = input.trim().split('\n').collect::<Vec<&str>>();
    let instructions: &str = lines[0].trim();
    let nodes = get_nodes(lines);

    let mut current_node = nodes.get("AAA").unwrap();
    let mut steps = 1;

    for ins in instructions.chars().cycle() {
        let next_node;
        if ins == 'R' {
            next_node = current_node.1.as_str();
        } else {
            next_node = current_node.0.as_str();
        }

        if next_node == "ZZZ" {
            break;
        }

        current_node = nodes.get(next_node).unwrap();
        steps += 1;
    }

    steps
}

pub fn part_two(input: &str) -> u64 {
    let lines = input.trim().split('\n').collect::<Vec<&str>>();
    let instructions: &str = lines[0].trim();
    let nodes = get_nodes(lines);

    let current_nodes = nodes
        .iter()
        .filter(|&n| n.0.ends_with("A"))
        .map(|(k, v)| ((k.as_str(), v.0.as_str(), v.1.as_str())))
        .collect::<Vec<(&str, &str, &str)>>();

    let mut all_steps: Vec<u64> = Vec::new();

    for node in current_nodes {
        let mut current_node = node;
        let mut steps = 1;
        for ins in instructions.chars().cycle() {
            let next_node;
            if ins == 'R' {
                next_node = current_node.2;
            } else {
                next_node = current_node.1;
            }

            if next_node.ends_with("Z") {
                break;
            }

            let n = nodes.get(next_node).unwrap();
            current_node = (next_node, n.0.as_str(), n.1.as_str());
            steps += 1;
        }

        all_steps.push(steps);
    }

    calculate_lcm(all_steps)
}

fn calculate_lcm(numbers: Vec<u64>) -> u64 {
    let mut result = 1;

    for num in numbers {
        result = lcm(result, num);
    }

    result
}

#[cfg(test)]
mod tests {
    #[test]
    fn example() {
        assert_eq!(super::part_one(include_str!("example.txt")), 2);
    }

    #[test]
    fn example2_part_one() {
        assert_eq!(super::part_one(include_str!("example2.txt")), 6);
    }

    #[test]
    fn part_one() {
        assert_eq!(super::part_one(include_str!("input.txt")), 24253);
    }

    #[test]
    fn example_part_two() {
        assert_eq!(super::part_two(include_str!("example_part_two.txt")), 6);
    }

    #[test]
    fn part_two() {
        assert_eq!(super::part_two(include_str!("input.txt")), 12357789728873);
    }
}
