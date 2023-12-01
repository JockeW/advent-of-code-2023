pub fn part_one(input: &str) -> usize {
    let mut values: Vec<usize> = Vec::new();

    for line in input.trim().split('\n') {
        let mut first_digit = '-';
        let mut last_digit = '-';

        for c in line.chars() {
            if c.is_digit(10) {
                if first_digit == '-' {
                    first_digit = c;
                    last_digit = c;
                } else {
                    last_digit = c;
                }
            }
        }

        let number: usize = format!("{}{}", first_digit, last_digit).parse().unwrap();
        values.push(number);
    }

    values.iter().sum()
}

pub fn part_two(input: &str) -> usize {
    let mut values: Vec<usize> = Vec::new();

    for line in input.trim().split('\n') {
        let mut first_digit = '-';
        let mut last_digit = '-';

        for (i, c) in line.chars().enumerate() {
            if c.is_digit(10) {
                if first_digit == '-' {
                    first_digit = c;
                    last_digit = c;
                } else {
                    last_digit = c;
                }
            } else if let Some(digit_char) = check_for_letter_digit(line, i) {
                if first_digit == '-' {
                    first_digit = digit_char;
                    last_digit = digit_char;
                } else {
                    last_digit = digit_char;
                }
            }
        }

        let number: usize = format!("{}{}", first_digit, last_digit).parse().unwrap();
        values.push(number);
    }

    values.iter().sum()
}

fn check_for_letter_digit(line: &str, index: usize) -> Option<char> {
    if line.len() >= index + 3 && &line[index..index + 3] == "one" {
        return Some('1');
    }
    if line.len() >= index + 3 && &line[index..index + 3] == "two" {
        return Some('2');
    }
    if line.len() >= index + 5 && &line[index..index + 5] == "three" {
        return Some('3');
    }
    if line.len() >= index + 4 && &line[index..index + 4] == "four" {
        return Some('4');
    }
    if line.len() >= index + 4 && &line[index..index + 4] == "five" {
        return Some('5');
    }
    if line.len() >= index + 3 && &line[index..index + 3] == "six" {
        return Some('6');
    }
    if line.len() >= index + 5 && &line[index..index + 5] == "seven" {
        return Some('7');
    }
    if line.len() >= index + 5 && &line[index..index + 5] == "eight" {
        return Some('8');
    }
    if line.len() >= index + 4 && &line[index..index + 4] == "nine" {
        return Some('9');
    }

    None
}

#[cfg(test)]
mod tests {
    #[test]
    fn example() {
        assert_eq!(super::part_one(include_str!("example.txt")), 142);
    }

    #[test]
    fn example2() {
        assert_eq!(super::part_two(include_str!("example2.txt")), 281);
    }

    #[test]
    fn part_one() {
        assert_eq!(super::part_one(include_str!("input.txt")), 55172);
    }

    #[test]
    fn part_two() {
        assert_eq!(super::part_two(include_str!("input.txt")), 54925);
    }
}
