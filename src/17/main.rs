use std::collections::HashSet;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

        assert_eq!(drop_rocks(input, 2022), 3068);
    }
}

fn drop_rocks(input: &str, amount: usize) -> u64 {
    let rocks: Vec<Vec<(u8, u64)>> = vec![
        vec![(0, 0), (1, 0), (2, 0), (3, 0)],
        vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
        vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
        vec![(0, 0), (0, 1), (0, 2), (0, 3)],
        vec![(0, 0), (1, 0), (0, 1), (1, 1)],
    ];
    let mut occupied = HashSet::<(u8, u64)>::new();
    // create floor
    for i in 0..9 {
        occupied.insert((i, 0));
    }

    let mut commands = input.chars().cycle();
    let mut height = 0;
    for i in 0..amount {
        // spawn
        let rock_type = i % 5;
        let mut rock = rocks.get(rock_type).unwrap().clone();

        // move to start position
        rock.iter_mut().for_each(|p| {
            p.0 += 3;
            p.1 += height + 4;
        });
        loop {
            // left/right
            match commands.next().unwrap() {
                '>' => {
                    let mut moved_rock = rock.clone();
                    moved_rock.iter_mut().for_each(|p| p.0 += 1);
                    let collissions = moved_rock
                        .iter()
                        .filter(|p| p.0 > 7 || occupied.get(*p).is_some())
                        .count();
                    if collissions == 0 {
                        rock = moved_rock;
                    }
                }
                '<' => {
                    let mut moved_rock = rock.clone();
                    moved_rock.iter_mut().for_each(|p| p.0 -= 1);
                    let collissions = moved_rock
                        .iter()
                        .filter(|p| p.0 < 1 || occupied.get(*p).is_some())
                        .count();
                    if collissions == 0 {
                        rock = moved_rock;
                    }
                }
                _ => panic!("Invalid command"),
            }
            // move down
            let mut descended_rock = rock.clone();
            descended_rock.iter_mut().for_each(|p| p.1 -= 1);
            let collissions = descended_rock
                .iter()
                .filter(|p| occupied.get(*p).is_some())
                .count();
            if collissions == 0 {
                rock = descended_rock;
            } else {
                break;
            }
        }
        rock.iter().for_each(|p| {
            occupied.insert(*p);
            height = height.max(p.1);
        });
    }
    height
}

fn main() {
    println!("Hello, day 17!");

    let input = std::fs::read_to_string("./input/17/input.txt").unwrap();
    let height = drop_rocks(input.trim(), 2022);
    println!("Part 1: {}", height);
}
