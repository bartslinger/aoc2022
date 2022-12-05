use std::fs;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_parsing() {
        let (stacks, moves) = parse_input("./src/05/test.txt");
        let expected_stacks = vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']];
        assert_eq!(stacks, expected_stacks);

        assert_eq!(
            moves[0],
            Move {
                number: 1,
                from: 2,
                to: 1
            }
        );
    }
}

#[derive(Eq, PartialEq, Debug)]
struct Move {
    number: usize,
    from: usize,
    to: usize,
}

fn parse_input(path: &str) -> (Vec<Vec<char>>, Vec<Move>) {
    let input_string = std::fs::read_to_string(path).unwrap();
    let mut parts = input_string.split("\n\n");
    let stacks = parse_stacks(parts.next().unwrap());
    let moves = parse_moves(parts.next().unwrap());
    (stacks, moves)
}

fn parse_stacks(input: &str) -> Vec<Vec<char>> {
    let mut stacks = vec![];
    let width = input.find('\n').unwrap() + 1;
    let height = ((input.len() + 1) / width) - 1; // exclude index row
    let number_of_stacks = width / 4;
    for i in 0..number_of_stacks {
        let mut stack = vec![];
        for j in 0..height {
            let index = ((height - 1 - j) * width + 1) + 4 * i;
            let item = input.chars().nth(index).unwrap();
            if item != ' ' {
                stack.push(item);
            }
        }
        stacks.push(stack);
    }
    stacks
}

fn parse_moves(input: &str) -> Vec<Move> {
    input
        .lines()
        .into_iter()
        .map(|line| Move {
            number: line.split(' ').nth(1).unwrap().parse::<usize>().unwrap(),
            from: line.split(' ').nth(3).unwrap().parse::<usize>().unwrap(),
            to: line.split(' ').nth(5).unwrap().parse::<usize>().unwrap(),
        })
        .collect()
}

fn main() {
    println!("Hello, day 5!");

    let (stacks, moves) = parse_input("./input/05/input.txt");
}
