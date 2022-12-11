use itertools::Itertools;
use std::cell::RefCell;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_round() {
        let mut monkeys = parse_input("./src/11/test.txt");
        play_round(&mut monkeys);
        assert_eq!(monkeys[0].borrow().items, vec![20, 23, 27, 26]);
        assert_eq!(
            monkeys[1].borrow().items,
            vec![2080, 25, 167, 207, 401, 1046]
        );
        assert_eq!(monkeys[2].borrow().items, vec![]);
        assert_eq!(monkeys[3].borrow().items, vec![]);
    }

    #[test]
    fn test_20_rounds() {
        let mut monkeys = parse_input("./src/11/test.txt");
        play_rounds(&mut monkeys, 20);
        assert_eq!(monkeys[0].borrow().items, vec![10, 12, 14, 26, 34]);
        assert_eq!(monkeys[1].borrow().items, vec![245, 93, 53, 199, 115]);
        assert_eq!(monkeys[2].borrow().items, vec![]);
        assert_eq!(monkeys[3].borrow().items, vec![]);
    }

    #[test]
    fn calculate_monkey_business() {
        let mut monkeys = parse_input("./src/11/test.txt");
        play_rounds(&mut monkeys, 20);
        assert_eq!(monkey_business(&monkeys), 10605);
    }
}

struct Monkey {
    items: Vec<i64>,
    operation: Box<dyn Fn(i64) -> i64>,
    test_divisible_by: i64,
    test_true_target_index: usize,
    test_false_target_index: usize,
    inspection_count: usize,
}

fn parse_operation(input: &str) -> Box<dyn Fn(i64) -> i64> {
    let args: Vec<&str> = input.split(' ').collect();
    match args.as_slice() {
        ["old", "+", "old"] => Box::new(move |x| x + x),
        ["old", "*", "old"] => Box::new(move |x| x * x),
        ["old", "*", b] => {
            let b = b.parse::<i64>().unwrap();
            Box::new(move |x| x * b)
        }
        ["old", "+", b] => {
            let b = b.parse::<i64>().unwrap();
            Box::new(move |x| x + b)
        }
        _ => panic!("could not parse"),
    }
}

fn parse_input(path: &str) -> Vec<RefCell<Monkey>> {
    let mut monkeys = vec![];
    let input = std::fs::read_to_string(path).unwrap();
    for input in input.split("\n\n") {
        let lines: Vec<&str> = input.lines().collect();
        let items: Vec<i64> = lines[1]
            .split(": ")
            .nth(1)
            .unwrap()
            .split(", ")
            .map(|x| x.parse().unwrap())
            .collect();
        let operation = lines[2].split("= ").nth(1).unwrap();
        let operation = parse_operation(operation);
        let test_divisible_by = lines[3].split(' ').last().unwrap().parse().unwrap();
        let test_true_target_index: usize = lines[4].split(' ').last().unwrap().parse().unwrap();
        let test_false_target_index: usize = lines[5].split(' ').last().unwrap().parse().unwrap();

        let monkey = Monkey {
            items,
            operation,
            test_divisible_by,
            test_true_target_index,
            test_false_target_index,
            inspection_count: 0,
        };
        monkeys.push(RefCell::new(monkey));
    }
    monkeys
}

fn play_round(monkeys: &mut [RefCell<Monkey>]) {
    for m in monkeys.iter() {
        let mut monkey = m.borrow_mut();
        let op = &monkey.operation;
        for item in monkey.items.iter() {
            let mut item = *item;
            item = op(item);
            item /= 3;
            let divisible = item % monkey.test_divisible_by == 0;
            let target_monkey_index = if divisible {
                monkey.test_true_target_index
            } else {
                monkey.test_false_target_index
            };
            let mut target_monkey = monkeys.get(target_monkey_index).unwrap().borrow_mut();
            target_monkey.items.push(item);
        }
        monkey.inspection_count += monkey.items.len();
        monkey.items = vec![];
    }
}

fn play_rounds(monkeys: &mut [RefCell<Monkey>], rounds: usize) {
    for _ in 0..rounds {
        play_round(monkeys);
    }
}

fn monkey_business(monkeys: &[RefCell<Monkey>]) -> usize {
    monkeys
        .iter()
        .map(|monkey| monkey.borrow().inspection_count)
        .sorted()
        .rev()
        .take(2)
        .product()
}

fn main() {
    println!("Hello, day 11!");

    let mut monkeys = parse_input("./input/11/input.txt");
    play_rounds(&mut monkeys, 20);
    let monkey_business = monkey_business(&monkeys);
    println!("Part 1: {}", monkey_business);
}
