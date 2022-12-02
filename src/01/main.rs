use itertools::Itertools;
use std::fs;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_parsing() {
        let input = parse_input("./src/01/test.txt");
        let expected = vec![
            vec![1000, 2000, 3000],
            vec![4000],
            vec![5000, 6000],
            vec![7000, 8000, 9000],
            vec![10000],
        ];
        println!("{:?}", input);
        assert_eq!(input, expected);
    }

    #[test]
    fn part_one() {
        let input = parse_input("./src/01/test.txt");
        assert_eq!(most_calories(input), 24000);
    }

    #[test]
    fn part_two() {
        let input = parse_input("./src/01/test.txt");
        assert_eq!(top_three_calories(input), 45000);
    }
}

fn parse_input(path: &str) -> Vec<Vec<u64>> {
    let input_string = fs::read_to_string(path).unwrap();
    let mut output: Vec<Vec<u64>> = vec![vec![]];
    for line in input_string.lines() {
        match line.parse::<u64>() {
            Ok(v) => output.last_mut().unwrap().push(v),
            Err(_) => output.push(vec![]),
        }
    }
    output
}

fn most_calories(input: Vec<Vec<u64>>) -> u64 {
    input
        .into_iter()
        .map(|x| x.into_iter().sum::<u64>())
        .max()
        .unwrap()
}

fn top_three_calories(input: Vec<Vec<u64>>) -> u64 {
    input
        .into_iter()
        .map(|x| x.into_iter().sum::<u64>())
        .sorted()
        .rev()
        .take(3)
        .sum()
}

fn main() {
    println!("Hello, day 1!");

    let input = parse_input("./input/01/input.txt");
    let most_calories = most_calories(input.clone());
    println!("Part 1: {}", most_calories);

    let top_three = top_three_calories(input);
    println!("Part 2: {}", top_three);
}
