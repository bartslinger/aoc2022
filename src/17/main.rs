use std::collections::HashSet;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

        assert_eq!(drop_rocks(input, 2022), 3068);
    }

    #[test]
    fn example_part_two() {
        let input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

        assert_eq!(drop_rocks(input, 1000000000000), 1514285714288);
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
    // count blocks per input cycle
    let commands_len = input.len();
    let mut commands_executed = 0;

    let mut command_index_memory: Vec<usize> = vec![];

    let mut height = 0;
    let mut prev_height = 0;
    let mut diffs = vec![];
    for i in 0..amount {
        // discover a pattern
        if i % 5 == 0 {
            let command_index = commands_executed % commands_len;
            command_index_memory.push(command_index);
            // find another occurance of last value
            let prev_occurance = command_index_memory
                .iter()
                .enumerate()
                .filter(|(_, item)| **item == command_index)
                .rev()
                .nth(1);
            if let Some((x, _)) = prev_occurance {
                let slice_length = command_index_memory.len() - x - 1;
                if command_index_memory.len() >= 2 * slice_length {
                    let mut pattern_detected = true;
                    for n in 0..slice_length {
                        let right = *command_index_memory
                            .get(command_index_memory.len() - 1 - n)
                            .unwrap();
                        let left = *command_index_memory
                            .get(command_index_memory.len() - 1 - n - slice_length)
                            .unwrap();
                        if left != right {
                            pattern_detected = false
                        }
                    }
                    if pattern_detected {
                        let pattern_diffs =
                            diffs.as_slice()[diffs.len() - slice_length * 5..].to_vec();

                        let pattern_height: u64 = pattern_diffs.iter().sum();

                        // println!("Pattern detected after dropping {} blocks", i);
                        // println!("The pattern contains {} blocks", slice_length * 5);
                        // println!("Every pattern increases the height by: {}", pattern_height);
                        // println!("Blocks remaining to be dropped: {}", amount - i);

                        let remaining_patterns = (amount - i) / (slice_length * 5);
                        let remaining_blocks = (amount - i) % (slice_length * 5);
                        height += remaining_patterns as u64 * pattern_height;
                        // println!("Height after dropping remaining patterns: {}", height);
                        // println!("Remaining blocks to be dropped: {}", remaining_blocks);

                        let remaining_height: u64 =
                            pattern_diffs.as_slice()[0..remaining_blocks].iter().sum();
                        height += remaining_height;
                        break;
                    }
                }
            }
        }

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
            commands_executed += 1;

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
        let diff = height - prev_height;
        diffs.push(diff);
        prev_height = height;
    }
    height
}

fn main() {
    println!("Hello, day 17!");

    let input = std::fs::read_to_string("./input/17/input.txt").unwrap();
    let height = drop_rocks(input.trim(), 2022);
    println!("Part 1: {}", height);

    let height = drop_rocks(input.trim(), 1000000000000);
    println!("Part 2: {}", height);
}
