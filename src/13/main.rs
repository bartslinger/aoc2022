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
    fn test_count_correct_pairs() {
        let input = parse_input("./src/13/test.txt");

        assert!(false);
    }
}

#[derive(PartialEq, Debug)]
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

fn main() {
    println!("Hello, day 13!");
}
