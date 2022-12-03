use std::fs;

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
}

fn parse_input(input: &str) -> Vec<&str> {
    input.lines().collect()
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
}
