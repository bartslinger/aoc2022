use std::fs;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_parsing() {
        let input = parse_input("./src/02/test.txt");
        assert_eq!(input, vec![('A', 'Y'), ('B', 'X'), ('C', 'Z')]);
    }

    #[test]
    fn day_one() {
        let input = parse_input("./src/02/test.txt");
        assert_eq!(play_with_strategy_guide(input), 15);
    }

    #[test]
    fn day_two() {
        let input = parse_input("./src/02/test.txt");
        assert_eq!(play_with_ultra_top_secret_strategy_guide(input), 12);
    }
}

fn parse_input(path: &str) -> Vec<(char, char)> {
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|x| (x.chars().nth(0).unwrap(), x.chars().nth(2).unwrap()))
        .collect()
}

fn round_score(input: (char, char)) -> u64 {
    // X = rock; Y = paper; Z = scissors
    let winning_score = match input {
        ('A', 'X') => 3,
        ('A', 'Y') => 6,
        ('A', 'Z') => 0,

        ('B', 'X') => 0,
        ('B', 'Y') => 3,
        ('B', 'Z') => 6,

        ('C', 'X') => 6,
        ('C', 'Y') => 0,
        ('C', 'Z') => 3,

        _ => panic!(""),
    };

    let play_score = match input.1 {
        'X' => 1,
        'Y' => 2,
        'Z' => 3,
        _ => panic!(""),
    };

    winning_score + play_score
}

fn play_with_strategy_guide(input: Vec<(char, char)>) -> u64 {
    input.iter().map(|x| round_score(*x)).sum()
}

fn get_what_to_play(input: (char, char)) -> char {
    // X = lose; Y = draw; Z = win
    match input {
        ('A', 'X') => 'Z',
        ('A', 'Y') => 'X',
        ('A', 'Z') => 'Y',

        ('B', 'X') => 'X',
        ('B', 'Y') => 'Y',
        ('B', 'Z') => 'Z',

        ('C', 'X') => 'Y',
        ('C', 'Y') => 'Z',
        ('C', 'Z') => 'X',

        _ => panic!(""),
    }
}

fn play_with_ultra_top_secret_strategy_guide(input: Vec<(char, char)>) -> u64 {
    input
        .into_iter()
        .map(|x| (x.0, get_what_to_play(x)))
        .map(|x| round_score(x))
        .sum()
}

fn main() {
    println!("Hello, day 2!");

    let input = parse_input("./input/02/input.txt");
    let score = play_with_strategy_guide(input.clone());
    println!("Part 1: {}", score);

    let score = play_with_ultra_top_secret_strategy_guide(input.clone());
    println!("Part 2: {}", score);
}
