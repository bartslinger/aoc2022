use std::fs;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_parsing() {
        let pairs = parse_input("./src/04/test.txt");
        assert_eq!(pairs[0].0 .0, 2);
        assert_eq!(pairs[0].0 .1, 4);
        assert_eq!(pairs[0].1 .0, 6);
        assert_eq!(pairs[0].1 .1, 8);
    }

    #[test]
    fn part_one() {
        let input = parse_input("./src/04/test.txt");
        assert_eq!(count_fully_enclosed(input), 2);
    }
}

type Pair = ((i64, i64), (i64, i64));

fn parse_range(input: &str) -> (i64, i64) {
    let mut bounds = input.split('-');
    (
        bounds.next().unwrap().parse().unwrap(),
        bounds.next().unwrap().parse().unwrap(),
    )
}

fn parse_input(path: &str) -> Vec<Pair> {
    std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|line| {
            let mut pairs = line.split(',');
            (
                parse_range(pairs.next().unwrap()),
                parse_range(pairs.next().unwrap()),
            )
        })
        .collect()
}

fn is_enclosed(pair: Pair) -> bool {
    let a = pair.0 .0 >= pair.1 .0 && pair.0 .1 <= pair.1 .1;
    let b = pair.0 .0 <= pair.1 .0 && pair.0 .1 >= pair.1 .1;
    a || b
}

fn count_fully_enclosed(input: Vec<Pair>) -> usize {
    input.iter().filter(|pair| is_enclosed(**pair)).count()
}

fn main() {
    println!("Hello, day 4!");

    let input = parse_input("./input/04/input.txt");
    let count = count_fully_enclosed(input);
    println!("Part 1: {}", count);
}
