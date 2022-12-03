use std::fs;

use itertools::Itertools;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_duplicate_item() {
        let rucksack = "vJrwpWtwJgWrhcsFMMfFFhFp";
        assert_eq!(find_duplicate_item(rucksack), 'p');
    }

    #[test]
    fn test_priorities() {
        assert_eq!(priority('a'), 1);
        assert_eq!(priority('z'), 26);
        assert_eq!(priority('A'), 27);
        assert_eq!(priority('Z'), 52);
    }

    #[test]
    fn part_one() {
        let input = fs::read_to_string("./src/03/test.txt").unwrap();
        let rucksacks = parse_input(&input);

        assert_eq!(sum_of_priorities(&rucksacks), 157);
    }

    #[test]
    fn part_two() {
        let input = fs::read_to_string("./src/03/test.txt").unwrap();
        let groups = parse_input_as_groups(&input);

        assert_eq!(sum_of_group_badges(groups), 70);
    }
}

fn parse_input(input: &str) -> Vec<&str> {
    input.lines().collect()
}

fn parse_input_as_groups(input: &str) -> Vec<(&str, &str, &str)> {
    let groups = input
        .lines()
        .chunks(3)
        .into_iter()
        .map(|mut x| (x.next().unwrap(), x.next().unwrap(), x.next().unwrap()))
        .collect();
    groups
}

fn find_duplicate_item(rucksack: &str) -> char {
    let (one, two) = rucksack.split_at(rucksack.len() / 2);

    for item in one.chars() {
        if two.contains(item) {
            return item;
        }
    }
    panic!("no duplicate found")
}

fn priority(item: char) -> u8 {
    let ascii = item as u8; // ASCII value
    match ascii {
        97..=122 => ascii - 97 + 1, // a-z
        65..=90 => ascii - 65 + 27, // A-Z
        _ => panic!("invalid item"),
    }
}

fn find_badge(group: (&str, &str, &str)) -> char {
    // Loop through the items in the first rucksack and see if they exist in the other two
    for item in group.0.chars() {
        if group.1.contains(item) && group.2.contains(item) {
            return item;
        }
    }
    panic!("could not find badge")
}

fn sum_of_group_badges(groups: Vec<(&str, &str, &str)>) -> u64 {
    groups
        .iter()
        .map(|group| find_badge(*group))
        .map(|badge| priority(badge) as u64)
        .sum()
}

fn sum_of_priorities(rucksacks: &Vec<&str>) -> u64 {
    rucksacks
        .iter()
        .map(|x| find_duplicate_item(*x))
        .map(|x| priority(x) as u64)
        .sum()
}

fn main() {
    println!("Hello, day 3!");

    let input = fs::read_to_string("./input/03/input.txt").unwrap();
    let rucksacks = parse_input(&input);
    let sum = sum_of_priorities(&rucksacks);
    println!("Part 1: {}", sum);

    let groups = parse_input_as_groups(&input);
    let sum = sum_of_group_badges(groups);
    println!("Part 2: {}", sum);
}
