use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = parse_input("./src/21/test.txt");

        assert_eq!(shout(&input, "root".to_string()), 152);
    }
}

#[derive(Debug)]
enum Shout {
    Number(i64),
    Multiply(String, String),
    Divide(String, String),
    Add(String, String),
    Sub(String, String),
}

type Monkeys = HashMap<String, Shout>;

fn parse_input(path: &str) -> Monkeys {
    let input = std::fs::read_to_string(path).unwrap();
    let mut monkeys = HashMap::<String, Shout>::new();
    for line in input.lines() {
        let name = line[0..4].to_string();
        let mut operation = line[6..].split(' ');
        if operation.clone().count() == 1 {
            let number: i64 = operation.next().unwrap().parse().unwrap();
            monkeys.insert(name, Shout::Number(number));
        } else {
            let a = operation.next().unwrap().to_string();
            let op: &str = operation.next().unwrap();
            let b = operation.next().unwrap().to_string();
            let shout = match op {
                "*" => Shout::Multiply(a, b),
                "/" => Shout::Divide(a, b),
                "+" => Shout::Add(a, b),
                "-" => Shout::Sub(a, b),
                _ => panic!("invalid operation"),
            };
            monkeys.insert(name, shout);
        }
    }
    monkeys
}

fn shout(monkeys: &Monkeys, name: String) -> i64 {
    let monkey = monkeys.get(&name).unwrap();
    match monkey {
        Shout::Number(x) => *x,
        Shout::Multiply(a, b) => shout(monkeys, a.clone()) * shout(monkeys, b.clone()),
        Shout::Divide(a, b) => shout(monkeys, a.clone()) / shout(monkeys, b.clone()),
        Shout::Add(a, b) => shout(monkeys, a.clone()) + shout(monkeys, b.clone()),
        Shout::Sub(a, b) => shout(monkeys, a.clone()) - shout(monkeys, b.clone()),
    }
}

fn main() {
    println!("Hello, day 21!");

    let monkeys = parse_input("./input/21/input.txt");
    let root_shout = shout(&monkeys, "root".to_string());
    println!("Part 1: {}", root_shout);
}
