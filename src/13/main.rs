extern crate core;

use itertools::Itertools;
use std::iter::Peekable;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_parsing() {
        let list = parse_item(&mut "[[42],[2,101,4]]".chars().peekable());
        assert_eq!(
            list,
            Item::List(vec![
                Item::List(vec![Item::Int(42),]),
                Item::List(vec![Item::Int(2), Item::Int(101), Item::Int(4)]),
            ])
        );
    }

    #[test]
    fn test_correct_pairs() {
        let pairs = parse_input("./src/13/test.txt");
        assert!(correct_order(&pairs[0]).unwrap());
        assert!(correct_order(&pairs[1]).unwrap());
        assert!(!correct_order(&pairs[2]).unwrap());
        assert!(correct_order(&pairs[3]).unwrap());
        assert!(!correct_order(&pairs[4]).unwrap());
        assert!(correct_order(&pairs[5]).unwrap());
        assert!(!correct_order(&pairs[6]).unwrap());
        assert!(!correct_order(&pairs[7]).unwrap());
    }

    #[test]
    fn test_count_correct_pairs() {
        let pairs = parse_input("./src/13/test.txt");
        assert_eq!(count_correct_pairs(&pairs), 13);
    }
}

#[derive(Clone, PartialEq, Debug)]
enum Item {
    Int(i32),
    List(Vec<Item>),
}

fn parse_item(line: &mut Peekable<std::str::Chars>) -> Item {
    match line.peek() {
        Some('[') => {
            line.next();
            let mut items = Vec::<Item>::new();
            loop {
                if *line.peek().unwrap() == ']' {
                    break;
                }
                let item = parse_item(line);
                items.push(item);
                match line.next() {
                    Some(',') => continue,
                    Some(']') => break,
                    v => panic!("unexpected value: {:?}", v),
                }
            }
            Item::List(items)
        }
        Some(_) => {
            let v: String = line.peeking_take_while(|c| c.is_ascii_digit()).collect();
            Item::Int(v.parse().unwrap())
        }
        None => panic!("end of input"),
    }
}

fn parse_input(path: &str) -> Vec<(Item, Item)> {
    let input = std::fs::read_to_string(path).unwrap();

    let mut pairs = vec![];
    for input in input.split("\n\n") {
        let mut lines = input.lines();
        let first = parse_item(&mut lines.next().unwrap().chars().peekable());
        let second = parse_item(&mut lines.next().unwrap().chars().peekable());
        let pair = (first, second);
        pairs.push(pair);
    }
    pairs
}

fn correct_order(pair: &(Item, Item)) -> Option<bool> {
    let pair = pair.clone();
    match pair {
        (Item::Int(a), Item::Int(b)) => {
            if a == b {
                None
            } else {
                Some(a <= b)
            }
        }
        (Item::Int(a), Item::List(b)) => {
            correct_order(&(Item::List(vec![Item::Int(a)]), Item::List(b)))
        }
        (Item::List(a), Item::Int(b)) => {
            correct_order(&(Item::List(a), Item::List(vec![Item::Int(b)])))
        }
        (Item::List(a), Item::List(b)) => {
            for i in 0..a.len() {
                let left = a.get(i);
                let right = b.get(i);
                if right.is_none() {
                    return Some(false);
                };
                if let Some(correct) =
                    correct_order(&(left.unwrap().clone(), right.unwrap().clone()))
                {
                    return Some(correct);
                }
            }
            Some(true)
        }
    }
}

fn count_correct_pairs(pairs: &[(Item, Item)]) -> usize {
    pairs
        .iter()
        .enumerate()
        .filter(|(_, pair)| correct_order(pair).unwrap())
        .map(|(index, _)| index + 1)
        .sum()
}

fn main() {
    println!("Hello, day 13!");

    let pairs = parse_input("./input/13/input.txt");
    let correct_pairs = count_correct_pairs(&pairs);
    println!("Part 1: {}", correct_pairs); // 5509 is wrong (too high)
}
