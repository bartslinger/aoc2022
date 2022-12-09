use std::collections::HashSet;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = parse_input("./src/09/test.txt");
        assert_eq!(count_tail_locations(&input, 2), 13);
    }

    #[test]
    fn test_larger_example() {
        let input = parse_input("./src/09/test2.txt");
        assert_eq!(count_tail_locations(&input, 10), 36);
    }
}

fn parse_input(path: &str) -> Vec<(i32, i32)> {
    let input = std::fs::read_to_string(path).unwrap();
    let lines = input.lines();
    lines
        .flat_map(|line| {
            let mut split = line.split(' ');
            let dir = split.next().unwrap();
            let amount: usize = split.next().unwrap().parse().unwrap();
            let delta = match dir {
                "U" => (0, 1),
                "D" => (0, -1),
                "R" => (1, 0),
                "L" => (-1, 0),
                _ => panic!("invalid direction"),
            };
            std::iter::repeat(delta)
                .take(amount)
                .collect::<Vec<(i32, i32)>>()
        })
        .collect()
}

fn add(a: (i32, i32), b: (i32, i32)) -> (i32, i32) {
    (a.0 + b.0, a.1 + b.1)
}

fn sub(a: (i32, i32), b: (i32, i32)) -> (i32, i32) {
    (a.0 - b.0, a.1 - b.1)
}

fn follow(head: (i32, i32), tail: (i32, i32)) -> (i32, i32) {
    let relative_position = sub(tail, head);
    let updated_relative_position = match relative_position {
        (-2, 0) => (-1, 0),
        (2, 0) => (1, 0),
        (0, -2) => (0, -1),
        (0, 2) => (0, 1),
        (-2, 1) | (-2, -1) => (-1, 0),
        (2, 1) | (2, -1) => (1, 0),
        (-1, 2) | (1, 2) => (0, 1),
        (-1, -2) | (1, -2) => (0, -1),
        // Add diagonal moves of the head for part 2
        (-2, -2) => (-1, -1),
        (-2, 2) => (-1, 1),
        (2, 2) => (1, 1),
        (2, -2) => (1, -1),
        _ => relative_position,
    };
    add(head, updated_relative_position)
}

fn count_tail_locations(steps: &Vec<(i32, i32)>, rope_length: usize) -> usize {
    // Create a rope of rope_length with all knots at (0, 0)
    let mut rope: Vec<(i32, i32)> = std::iter::repeat((0, 0)).take(rope_length).collect();

    let mut unique_positions = HashSet::<(i32, i32)>::new();
    for step in steps {
        rope[0] = add(rope[0], *step);
        for n in 1..rope_length {
            rope[n] = follow(rope[n - 1], rope[n]);
        }
        let tail = *rope.last().unwrap();
        unique_positions.insert(tail);
    }

    unique_positions.len()
}

fn main() {
    println!("Hello, day 9!");
    let input = parse_input("./input/09/input.txt");

    let unique_positions = count_tail_locations(&input, 2);
    println!("Part 1: {}", unique_positions);

    let unique_positions = count_tail_locations(&input, 10);
    println!("Part 2: {}", unique_positions);
}
