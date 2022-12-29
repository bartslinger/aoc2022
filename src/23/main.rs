use std::collections::{HashMap, HashSet};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let elves = parse_input("./src/23/test.txt");
        let elves = spread_out(&elves, 10);

        assert_eq!(count_ground_tiles(&elves), 110);
    }
}

fn parse_input(path: &str) -> HashSet<(i32, i32)> {
    let input = std::fs::read_to_string(path).unwrap();
    let mut elves = HashSet::new();

    for (row, line) in input.lines().enumerate() {
        for (col, item) in line.chars().enumerate() {
            if item == '#' {
                elves.insert((row as i32, col as i32));
            }
        }
    }

    elves
}

#[derive(Debug)]
struct Elf {
    position: (i32, i32),
    proposal: (i32, i32),
}

fn direction_possible(positions: &HashSet<(i32, i32)>, elf: &Elf, direction: i32) -> bool {
    // 0 = North
    // 1 = South
    // 2 = West
    // 3 = East
    let options = match direction {
        0 => vec![
            // North
            (elf.position.0 - 1, elf.position.1 - 1),
            (elf.position.0 - 1, elf.position.1),
            (elf.position.0 - 1, elf.position.1 + 1),
        ],
        1 => vec![
            // South
            (elf.position.0 + 1, elf.position.1 - 1),
            (elf.position.0 + 1, elf.position.1),
            (elf.position.0 + 1, elf.position.1 + 1),
        ],
        2 => vec![
            // West
            (elf.position.0 - 1, elf.position.1 - 1),
            (elf.position.0, elf.position.1 - 1),
            (elf.position.0 + 1, elf.position.1 - 1),
        ],
        3 => vec![
            // East
            (elf.position.0 - 1, elf.position.1 + 1),
            (elf.position.0, elf.position.1 + 1),
            (elf.position.0 + 1, elf.position.1 + 1),
        ],
        _ => panic!("invalid direction"),
    };
    options
        .iter()
        .filter(|p| positions.get(*p).is_some())
        .count()
        == 0
}

fn propose_direction(elf: &mut Elf, direction: i32) {
    // 0 = North
    // 1 = South
    // 2 = West
    // 3 = East
    match direction {
        0 => {
            // North
            elf.proposal = (elf.position.0 - 1, elf.position.1);
        }
        1 => {
            // South
            elf.proposal = (elf.position.0 + 1, elf.position.1);
        }
        2 => {
            // West
            elf.proposal = (elf.position.0, elf.position.1 - 1);
        }
        3 => {
            // East
            elf.proposal = (elf.position.0, elf.position.1 + 1);
        }
        _ => panic!("invalid direction"),
    }
}

fn spread_out(input: &HashSet<(i32, i32)>, rounds: usize) -> HashSet<(i32, i32)> {
    let mut positions = input.clone();
    let mut direction = 0;
    let directions = [[0, 1, 2, 3], [1, 2, 3, 0], [2, 3, 0, 1], [3, 0, 1, 2]];
    for _ in 0..rounds {
        let mut elves: Vec<Elf> = positions
            .iter()
            .map(|p| Elf {
                position: *p,
                proposal: *p, // calculate actual proposal below
            })
            .collect();

        let mut proposals = HashMap::<(i32, i32), usize>::new();
        for elf in &mut elves {
            let a = direction_possible(&positions, elf, directions[direction][0]);
            let b = direction_possible(&positions, elf, directions[direction][1]);
            let c = direction_possible(&positions, elf, directions[direction][2]);
            let d = direction_possible(&positions, elf, directions[direction][3]);
            if !a || !b || !c || !d {
                if a {
                    propose_direction(elf, directions[direction][0]);
                } else if b {
                    propose_direction(elf, directions[direction][1]);
                } else if c {
                    propose_direction(elf, directions[direction][2]);
                } else if d {
                    propose_direction(elf, directions[direction][3]);
                }
            }
            if let Some(count) = proposals.get_mut(&elf.proposal) {
                *count += 1;
            } else {
                proposals.insert(elf.proposal, 1);
            }
        }

        // Move elf if proposal count equals 1
        for mut elf in &mut elves {
            let count = proposals.get(&elf.proposal).unwrap();
            if *count == 1 {
                // move
                elf.position = elf.proposal;
            }
        }

        // Update direction
        direction += 1;
        direction %= 4;

        // Update positions with elf locations
        let new_positions: HashSet<(i32, i32)> = elves.iter().map(|elf| elf.position).collect();
        positions = new_positions;
    }
    positions
}

#[allow(unused)]
fn draw(positions: &HashSet<(i32, i32)>) {
    for row in -2..10 {
        for col in -3..11 {
            if positions.get(&(row, col)).is_some() {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

fn count_ground_tiles(elves: &HashSet<(i32, i32)>) -> usize {
    let north = elves.iter().map(|p| p.0).min().unwrap();
    let south = elves.iter().map(|p| p.0).max().unwrap();
    let west = elves.iter().map(|p| p.1).min().unwrap();
    let east = elves.iter().map(|p| p.1).max().unwrap();

    let mut count = 0;
    for row in north..=south {
        for col in west..=east {
            if elves.get(&(row, col)).is_none() {
                count += 1;
            }
        }
    }
    count
}

fn main() {
    println!("Hello, day 23!");

    let elves = parse_input("./input/23/input.txt");
    let elves = spread_out(&elves, 10);
    let ground_tiles = count_ground_tiles(&elves);
    println!("Part 1: {}", ground_tiles);
}
