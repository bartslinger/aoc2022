extern crate core;

use itertools::Itertools;
use std::cmp::Ordering;
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
    fn test_parse_empty_list_first() {
        let list = parse_item(&mut "[[],1]".chars().peekable());
        assert_eq!(list, Item::List(vec![Item::List(vec![]), Item::Int(1)]));
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
    fn test_equal_list_first() {
        let left = Item::List(vec![
            Item::List(vec![Item::Int(42), Item::Int(42)]),
            Item::Int(5),
        ]);
        let right = Item::List(vec![
            Item::List(vec![Item::Int(42), Item::Int(42)]),
            Item::Int(3),
        ]);
        assert!(!correct_order(&(left, right)).unwrap());
    }

    #[test]
    fn test_comparing_equal_inputs() {
        let left = Item::List(vec![Item::List(vec![])]);
        let right = Item::List(vec![Item::List(vec![])]);
        assert!(correct_order(&(left, right)).is_none());
    }

    #[test]
    fn test_count_correct_pairs() {
        let pairs = parse_input("./src/13/test.txt");
        assert_eq!(count_correct_pairs(&pairs), 13);
    }

    #[test]
    fn sort_example() {
        let pairs = parse_input("./src/13/test.txt");
        let decoder_key = find_decoder_key(&pairs);
        assert_eq!(decoder_key, 140);
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
enum Item {
    Int(i32),
    List(Vec<Item>),
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        match correct_order(&(self.clone(), other.clone())) {
            Some(true) => Ordering::Less,
            Some(false) => Ordering::Greater,
            None => Ordering::Equal,
        }
    }
}

fn parse_item(line: &mut Peekable<std::str::Chars>) -> Item {
    match line.peek() {
        Some('[') => {
            line.next();
            let mut items = Vec::<Item>::new();
            loop {
                if *line.peek().unwrap() == ']' {
                    line.next();
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

fn flatten_pairs(pairs: &Vec<(Item, Item)>) -> Vec<Item> {
    let mut items = vec![];
    for pair in pairs {
        items.push(pair.0.clone());
        items.push(pair.1.clone());
    }
    items
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
            if b.len() == a.len() {
                return None;
            }
            Some(a.len() < b.len())
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

fn find_decoder_key(pairs: &Vec<(Item, Item)>) -> usize {
    let mut items = flatten_pairs(pairs);
    let first = parse_item(&mut "[[2]]".chars().peekable());
    let second = parse_item(&mut "[[6]]".chars().peekable());
    items.push(first.clone());
    items.push(second.clone());
    items.sort();

    let mut first_index = 0;
    let mut second_index = 0;
    for (index, item) in items.iter().enumerate() {
        if *item == first {
            first_index = index + 1;
        }
        if *item == second {
            second_index = index + 1;
        }
    }
    first_index * second_index
}

fn main() {
    println!("Hello, day 13!");

    let pairs = parse_input("./input/13/input.txt");
    let correct_pairs = count_correct_pairs(&pairs);
    println!("Part 1: {}", correct_pairs);

    let decoder_key = find_decoder_key(&pairs);
    println!("Part 2: {}", decoder_key);
}
