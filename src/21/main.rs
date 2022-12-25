use std::borrow::Borrow;
use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = parse_input("./src/21/test.txt");

        assert_eq!(shout(&input, "root".to_string()), 152);
    }

    #[test]
    fn test_part_two() {
        let input = parse_input("./src/21/test.txt");

        assert_eq!(equality_shout(&input), 301);
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

#[derive(Clone, Debug, Eq, PartialEq)]
enum Var {
    Humn,
    Number(i64),
    Multiply(Box<Var>, Box<Var>),
    Divide(Box<Var>, Box<Var>),
    Add(Box<Var>, Box<Var>),
    Sub(Box<Var>, Box<Var>),
}

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

fn shout_vars(monkeys: &Monkeys, name: String) -> Box<Var> {
    let monkey = monkeys.get(&name).unwrap();
    if name == "humn" {
        return Box::new(Var::Humn);
    }
    let var = match monkey {
        Shout::Number(x) => Var::Number(*x),
        Shout::Multiply(a, b) => Var::Multiply(
            shout_vars(monkeys, a.clone()),
            shout_vars(monkeys, b.clone()),
        ),
        Shout::Divide(a, b) => Var::Divide(
            shout_vars(monkeys, a.clone()),
            shout_vars(monkeys, b.clone()),
        ),
        Shout::Add(a, b) => Var::Add(
            shout_vars(monkeys, a.clone()),
            shout_vars(monkeys, b.clone()),
        ),
        Shout::Sub(a, b) => Var::Sub(
            shout_vars(monkeys, a.clone()),
            shout_vars(monkeys, b.clone()),
        ),
        _ => panic!("invalid"),
    };
    Box::new(var)
}

fn simplify(input: Box<Var>) -> Box<Var> {
    match input.borrow() {
        Var::Humn | Var::Number(_) => input,
        Var::Multiply(a_var, b_var) => {
            let a = a_var.borrow();
            let b = b_var.borrow();
            match (a, b) {
                (Var::Number(x), Var::Number(y)) => Box::new(Var::Number(x * y)),
                _ => {
                    let a_simplified = simplify(a_var.clone());
                    let b_simplified = simplify(b_var.clone());
                    match (a_simplified.borrow(), b_simplified.borrow()) {
                        (&Var::Number(_), &Var::Number(_)) => {
                            simplify(Box::new(Var::Multiply(a_simplified, b_simplified)))
                        }
                        _ => Box::new(Var::Multiply(a_simplified, b_simplified)),
                    }
                }
            }
        }
        Var::Divide(a, b) => {
            let a = a.borrow();
            let b = b.borrow();
            match (a, b) {
                (Var::Number(x), Var::Number(y)) => Box::new(Var::Number(x / y)),
                _ => {
                    let a_simplified = simplify(Box::new(a.clone()));
                    let b_simplified = simplify(Box::new(b.clone()));
                    match (a_simplified.borrow(), b_simplified.borrow()) {
                        (&Var::Number(_), &Var::Number(_)) => {
                            simplify(Box::new(Var::Divide(a_simplified, b_simplified)))
                        }
                        _ => Box::new(Var::Divide(a_simplified, b_simplified)),
                    }
                }
            }
        }
        Var::Add(a, b) => {
            let a = a.borrow();
            let b = b.borrow();
            match (a, b) {
                (Var::Number(x), Var::Number(y)) => Box::new(Var::Number(x + y)),
                _ => {
                    let a_simplified = simplify(Box::new(a.clone()));
                    let b_simplified = simplify(Box::new(b.clone()));
                    match (a_simplified.borrow(), b_simplified.borrow()) {
                        (&Var::Number(_), &Var::Number(_)) => {
                            simplify(Box::new(Var::Add(a_simplified, b_simplified)))
                        }
                        _ => Box::new(Var::Add(a_simplified, b_simplified)),
                    }
                }
            }
        }
        Var::Sub(a, b) => {
            let a = a.borrow();
            let b = b.borrow();
            match (a, b) {
                (Var::Number(x), Var::Number(y)) => Box::new(Var::Number(x - y)),
                _ => {
                    let a_simplified = simplify(Box::new(a.clone()));
                    let b_simplified = simplify(Box::new(b.clone()));
                    match (a_simplified.borrow(), b_simplified.borrow()) {
                        (&Var::Number(_), &Var::Number(_)) => {
                            simplify(Box::new(Var::Sub(a_simplified, b_simplified)))
                        }
                        _ => Box::new(Var::Sub(a_simplified, b_simplified)),
                    }
                }
            }
        }
    }
}

fn inverse(input: Box<Var>, number: i64) -> i64 {
    let input = input.borrow();
    match input {
        Var::Divide(a_box, b_box) => {
            let a_var = a_box.borrow();
            let b_var = b_box.borrow();
            match (a_var, b_var) {
                (Var::Number(x), _) => {}
                (_, Var::Number(x)) => {
                    // let new_var =
                    //     Box::new(Var::Add(a_box.clone(), Box::new(Var::Number(number * x))));
                }
                _ => panic!("invalid"),
            }
            0
        }
        _ => 0,
    }
}

fn figure_out_humn(formula: Box<Var>, compare_to: i64) -> i64 {
    println!("{:#?}", formula);
    inverse(formula, compare_to)
}

fn equality_shout(monkeys: &Monkeys) -> i64 {
    let root = monkeys.get("root").unwrap();
    let (a, b) = if let Shout::Add(a, b) = root {
        (a, b)
    } else {
        panic!();
    };
    let a_var = simplify(shout_vars(monkeys, a.clone()));
    let b_var = simplify(shout_vars(monkeys, b.clone()));

    let mut formula = a_var.clone();
    let mut compare_to = 0;
    if let Var::Number(x) = a_var.borrow() {
        compare_to = *x;
        formula = b_var.clone();
    }
    if let Var::Number(x) = b_var.borrow() {
        compare_to = *x;
    }

    figure_out_humn(formula, compare_to)
}

fn main() {
    println!("Hello, day 21!");

    let monkeys = parse_input("./input/21/input.txt");
    let root_shout = shout(&monkeys, "root".to_string());
    println!("Part 1: {}", root_shout);

    let humn_shout = equality_shout(&monkeys);
}
