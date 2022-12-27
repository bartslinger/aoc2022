use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let (monkey_map, instructions, start_position) = parse_input("./src/22/test.txt");

        let password = get_password(&monkey_map, &instructions, &start_position);
        assert_eq!(password, 6032);
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum TileType {
    Open,
    Wall,
}

#[derive(Debug, Copy, Clone)]
struct Tile {
    tile_type: TileType,
    west: (i32, i32, i32),
    east: (i32, i32, i32),
    north: (i32, i32, i32),
    south: (i32, i32, i32),
}

type MonkeyMap = HashMap<(i32, i32), Tile>;

#[derive(Debug)]
enum Instruction {
    Move(usize),
    TurnLeft,
    TurnRight,
}
type Instructions = Vec<Instruction>;

fn parse_input(path: &str) -> (MonkeyMap, Instructions, (i32, i32)) {
    let input = std::fs::read_to_string(path).unwrap();
    let mut parts = input.split("\n\n");
    let mut start_col = 0;

    let mut map = MonkeyMap::new();
    for (row, line) in parts.next().unwrap().lines().enumerate() {
        for (col, item) in line.chars().enumerate() {
            match item {
                '#' | '.' => {
                    // is there an item to the left?
                    let west = if let Some(left) = map.get(&(row as i32, col as i32 - 1)) {
                        let left = *left;
                        for j in left.west.1..(col as i32) {
                            if let Some(item_left) = map.get_mut(&(row as i32, j)) {
                                item_left.east = (row as i32, col as i32, 0);
                            }
                        }
                        (row as i32, left.west.1, 0)
                    } else {
                        if row == 0 {
                            start_col = col as i32;
                        }
                        (row as i32, col as i32, 0)
                    };
                    let north = if let Some(above) = map.get(&(row as i32 - 1, col as i32)) {
                        let above = *above;
                        for i in above.north.0..(row as i32) {
                            if let Some(above_item) = map.get_mut(&(i, col as i32)) {
                                above_item.south = (row as i32, col as i32, 0);
                            }
                        }
                        above.north
                    } else {
                        (row as i32, col as i32, 0)
                    };
                    let tile_type = if item == '.' {
                        TileType::Open
                    } else {
                        TileType::Wall
                    };
                    let tile = Tile {
                        tile_type,
                        west,
                        east: (row as i32, col as i32, 0),
                        north,
                        south: (row as i32, col as i32, 0),
                    };
                    map.insert((row as i32, col as i32), tile);
                }
                _ => {}
            }
        }
    }

    let mut instructions = vec![];
    for i in parts.next().unwrap().trim().split_inclusive(&['L', 'R']) {
        if i.contains('L') {
            let number = i[0..i.len() - 1].parse().unwrap();
            instructions.push(Instruction::Move(number));
            instructions.push(Instruction::TurnLeft);
        } else if i.contains('R') {
            let number = i[0..i.len() - 1].parse().unwrap();
            instructions.push(Instruction::Move(number));
            instructions.push(Instruction::TurnRight);
        } else {
            let number = i.parse().unwrap();
            instructions.push(Instruction::Move(number));
        }
    }

    (map, instructions, (0, start_col))
}

fn get_password(
    monkey_map: &MonkeyMap,
    instructions: &Instructions,
    start_position: &(i32, i32),
) -> i32 {
    let mut position = *start_position;
    let mut direction = 0;
    let mut current_tile = monkey_map.get(&position).unwrap();

    for instruction in instructions {
        match instruction {
            Instruction::Move(distance) => {
                for _ in 0..*distance {
                    let next_position = match direction {
                        0 => (position.0, position.1 + 1),
                        1 => (position.0 + 1, position.1),
                        2 => (position.0, position.1 - 1),
                        3 => (position.0 - 1, position.1),
                        _ => panic!("invalid direction"),
                    };
                    let next_tile = monkey_map.get(&next_position);
                    if let Some(tile) = next_tile {
                        if tile.tile_type == TileType::Open {
                            position = next_position;
                            current_tile = tile;
                        } else {
                            // else, next is wall and we don't move
                            break;
                        }
                    } else {
                        // Wrap around, next tile depends on direction
                        let next_position = match direction {
                            0 => current_tile.west,
                            1 => current_tile.north,
                            2 => current_tile.east,
                            3 => current_tile.south,
                            _ => panic!("invalid direction"),
                        };
                        let next_coordinate = (next_position.0, next_position.1);
                        let next_tile = monkey_map.get(&next_coordinate).unwrap();
                        if next_tile.tile_type == TileType::Open {
                            position = next_coordinate;
                            current_tile = next_tile;
                        }
                    }
                }
            }
            Instruction::TurnLeft => {
                direction += 3;
                direction %= 4;
            }
            Instruction::TurnRight => {
                direction += 1;
                direction %= 4;
            }
        }
    }
    let final_row = position.0 + 1;
    let final_col = position.1 + 1;
    let final_facing = direction;
    1000 * final_row + 4 * final_col + final_facing
}

fn main() {
    println!("Hello, day 22!");

    let (monkey_map, instructions, start_position) = parse_input("./input/22/input.txt");
    let password = get_password(&monkey_map, &instructions, &start_position);
    println!("Part 1: {}", password);
}
