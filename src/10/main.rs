use itertools::Itertools;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let instructions = parse_input("./src/10/test.txt");
        let history = register_value_history(&instructions);
        assert_eq!(signal_strength(&history, 20), 420);
        assert_eq!(signal_strength(&history, 60), 1140);
        assert_eq!(signal_strength(&history, 100), 1800);
        assert_eq!(signal_strength(&history, 140), 2940);
        assert_eq!(signal_strength(&history, 180), 2880);
        assert_eq!(signal_strength(&history, 220), 3960);
    }

    #[test]
    fn test_part_one() {
        let instructions = parse_input("./src/10/test.txt");
        let history = register_value_history(&instructions);
        assert_eq!(signal_strength_sum(&history), 13140);
    }

    #[test]
    fn crt_rows() {
        let instructions = parse_input("./src/10/test.txt");
        let history = register_value_history(&instructions);
        assert_eq!(
            crt_row(&history, 0),
            "##..##..##..##..##..##..##..##..##..##.."
        );
        assert_eq!(
            crt_row(&history, 5),
            "#######.......#######.......#######....."
        );
    }
}

#[derive(Debug)]
enum Instruction {
    Noop,
    Addx(i32),
}

fn parse_input(path: &str) -> Vec<Instruction> {
    let input = std::fs::read_to_string(path).unwrap();
    input
        .lines()
        .map(|line| {
            let instruction = line.split(' ').next().unwrap();

            match instruction {
                "noop" => Instruction::Noop,
                "addx" => {
                    let value: i32 = line.split(' ').nth(1).unwrap().parse().unwrap();
                    Instruction::Addx(value)
                }
                _ => panic!("invalid instruction"),
            }
        })
        .collect()
}

fn register_value_history(instructions: &Vec<Instruction>) -> Vec<i32> {
    let mut history = Vec::new();
    let mut value = 1;
    history.push(value);
    for instruction in instructions {
        match instruction {
            Instruction::Noop => history.push(value),
            Instruction::Addx(v) => {
                history.push(value);
                value += v;
                history.push(value);
            }
        }
    }
    history
}

fn signal_strength(history: &[i32], cycle: usize) -> i32 {
    let value = history.get(cycle - 1).unwrap();
    value * (cycle as i32)
}

fn signal_strength_sum(history: &[i32]) -> i32 {
    (20..=220)
        .step_by(40)
        .map(|cycle| signal_strength(history, cycle))
        .sum()
}

fn crt_row(history: &[i32], row: usize) -> String {
    (0..40)
        .map(|i| {
            let diff = history[i + row * 40] - (i as i32);
            diff.abs() <= 1
        })
        .map(|visible| if visible { '#' } else { '.' })
        .collect()
}

fn crt_screen(history: &[i32]) -> String {
    (0..6).map(|row| crt_row(history, row)).join("\n")
}

fn main() {
    println!("Hello, day 10!");

    let instructions = parse_input("./input/10/input.txt");
    let history = register_value_history(&instructions);
    let sum = signal_strength_sum(&history);

    println!("Part 1: {}", sum);
    println!("Part 2:");
    println!("{}", crt_screen(&history));
}
