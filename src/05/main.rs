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

    #[test]
    fn part_one() {
        let (stacks, moves) = parse_input("./src/05/test.txt");
        let arranged = arrange_with_silly_crate_mover_9000(stacks, moves);
        assert_eq!(get_top_crates(arranged), "CMZ");
    }

    #[test]
    fn part_two() {
        let (stacks, moves) = parse_input("./src/05/test.txt");
        let arranged = arrange_with_crate_mover_9001(stacks, moves);
        assert_eq!(get_top_crates(arranged), "MCD");
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
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

fn get_top_crates(stacks: Vec<Vec<char>>) -> String {
    stacks.into_iter().map(|s| *s.last().unwrap()).collect()
}

fn arrange_with_silly_crate_mover_9000(
    mut stacks: Vec<Vec<char>>,
    moves: Vec<Move>,
) -> Vec<Vec<char>> {
    for mov in moves {
        for _ in 0..mov.number {
            let item = stacks.get_mut(mov.from - 1).unwrap().pop().unwrap();
            stacks.get_mut(mov.to - 1).unwrap().push(item);
        }
    }
    stacks
}

fn arrange_with_crate_mover_9001(mut stacks: Vec<Vec<char>>, moves: Vec<Move>) -> Vec<Vec<char>> {
    let mut buffer = vec![];
    for mov in moves {
        for _ in 0..mov.number {
            let item = stacks.get_mut(mov.from - 1).unwrap().pop().unwrap();
            buffer.push(item);
        }
        for _ in 0..mov.number {
            let item = buffer.pop().unwrap();
            stacks.get_mut(mov.to - 1).unwrap().push(item);
        }
    }
    stacks
}

fn main() {
    println!("Hello, day 5!");

    let (stacks, moves) = parse_input("./input/05/input.txt");
    let arranged = arrange_with_silly_crate_mover_9000(stacks.clone(), moves.clone());
    let top_crates = get_top_crates(arranged);
    println!("Part 1: {}", top_crates);

    let arranged = arrange_with_crate_mover_9001(stacks, moves);
    let top_crates = get_top_crates(arranged);
    println!("Part 2: {}", top_crates);
}
