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

    #[test]
    fn example_cube() {
        let (monkey_map, instructions, start_position) = parse_input("./src/22/test.txt");

        let cube_map = get_cube_map(&monkey_map, 4);

        let password = get_password(&cube_map, &instructions, &start_position);
        assert_eq!(password, 5031);
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

fn parse_input(path: &str) -> (MonkeyMap, Instructions, (i32, i32, i32)) {
    let input = std::fs::read_to_string(path).unwrap();
    let mut parts = input.split("\n\n");
    let mut start_col = -1;

    let mut map = MonkeyMap::new();
    for (row, line) in parts.next().unwrap().lines().enumerate() {
        for (col, item) in line.chars().enumerate() {
            match item {
                '#' | '.' => {
                    let tile_type = if item == '.' {
                        TileType::Open
                    } else {
                        TileType::Wall
                    };
                    if start_col == -1 {
                        start_col = col as i32;
                    }
                    let tile = Tile {
                        tile_type,
                        east: (row as i32, col as i32 + 1, 0),
                        south: (row as i32 + 1, col as i32, 1),
                        west: (row as i32, col as i32 - 1, 2),
                        north: (row as i32 - 1, col as i32, 3),
                    };
                    map.insert((row as i32, col as i32), tile);
                }
                _ => {}
            }
        }
    }

    let mut wrapped_map = map.clone();
    for (coordinate, tile) in &map {
        if map.get(&(tile.east.0, tile.east.1)).is_none() {
            let mut next = tile.west;
            loop {
                if let Some(next_tile) = map.get(&(next.0, next.1)) {
                    if map.get(&(next_tile.west.0, next_tile.west.1)).is_none() {
                        let updated_tile = wrapped_map.get_mut(coordinate).unwrap();
                        updated_tile.east.0 = next.0;
                        updated_tile.east.1 = next.1;
                        break;
                    } else {
                        next = next_tile.west;
                    }
                }
            }
        }

        if map.get(&(tile.south.0, tile.south.1)).is_none() {
            let mut next = tile.north;
            loop {
                if let Some(next_tile) = map.get(&(next.0, next.1)) {
                    if map.get(&(next_tile.north.0, next_tile.north.1)).is_none() {
                        let updated_tile = wrapped_map.get_mut(coordinate).unwrap();
                        updated_tile.south.0 = next.0;
                        updated_tile.south.1 = next.1;
                        break;
                    } else {
                        next = next_tile.north;
                    }
                }
            }
        }

        if map.get(&(tile.west.0, tile.west.1)).is_none() {
            let mut next = tile.east;
            loop {
                if let Some(next_tile) = map.get(&(next.0, next.1)) {
                    if map.get(&(next_tile.east.0, next_tile.east.1)).is_none() {
                        let updated_tile = wrapped_map.get_mut(coordinate).unwrap();
                        updated_tile.west.0 = next.0;
                        updated_tile.west.1 = next.1;
                        break;
                    } else {
                        next = next_tile.east;
                    }
                }
            }
        }

        if map.get(&(tile.north.0, tile.north.1)).is_none() {
            let mut next = tile.south;
            loop {
                if let Some(next_tile) = map.get(&(next.0, next.1)) {
                    if map.get(&(next_tile.south.0, next_tile.south.1)).is_none() {
                        let updated_tile = wrapped_map.get_mut(coordinate).unwrap();
                        updated_tile.north.0 = next.0;
                        updated_tile.north.1 = next.1;
                        break;
                    } else {
                        next = next_tile.south;
                    }
                }
            }
        }
    }
    map = wrapped_map;

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

    let start_position = (0, start_col, 0);
    (map, instructions, start_position)
}

// first is the cell id
// second is the side
type Edge = (i32, i32);

fn get_cube_map(monkey_map: &MonkeyMap, edge_length: usize) -> MonkeyMap {
    let mut cube_map = monkey_map.clone();

    // Read the map into these cells:
    //  0  1  2  3
    //  4  5  6  7
    //  8  9 10 11
    // 12 13 14 15

    // Connect sides
    let mut connected_edges = HashMap::<Edge, Edge>::new();
    if edge_length == 4 {
        // example
        connected_edges.insert((2, 2), (5, 3));
        connected_edges.insert((2, 3), (4, 3));
        connected_edges.insert((2, 0), (11, 0));
        connected_edges.insert((6, 0), (11, 3));
        connected_edges.insert((11, 3), (6, 0));
        connected_edges.insert((11, 0), (2, 0));
        connected_edges.insert((11, 1), (4, 2));
        connected_edges.insert((10, 1), (4, 1));
        connected_edges.insert((10, 2), (5, 1));
        connected_edges.insert((5, 1), (10, 2));
        connected_edges.insert((4, 1), (10, 1));
        connected_edges.insert((4, 2), (11, 1));
        connected_edges.insert((4, 3), (2, 3));
        connected_edges.insert((5, 3), (2, 2));
    } else if edge_length == 50 {
        // This is for part 2
        connected_edges.insert((1, 3), (12, 2));
        connected_edges.insert((2, 3), (12, 1));
        connected_edges.insert((2, 0), (9, 0));
        connected_edges.insert((2, 1), (5, 0));
        connected_edges.insert((5, 0), (2, 1));
        connected_edges.insert((9, 0), (2, 0));
        connected_edges.insert((9, 1), (12, 0));
        connected_edges.insert((12, 0), (9, 1));
        connected_edges.insert((12, 1), (2, 3));
        connected_edges.insert((12, 2), (1, 3));
        connected_edges.insert((8, 2), (1, 2));
        connected_edges.insert((8, 3), (5, 2));
        connected_edges.insert((5, 2), (8, 3));
        connected_edges.insert((1, 2), (8, 2));
    }

    for (from, to) in connected_edges {
        let from_col = from.0 % edge_length as i32;
        let from_row = from.0 / edge_length as i32;
        let to_col = to.0 % edge_length as i32;
        let to_row = to.0 / edge_length as i32;

        let to_vec: Vec<(i32, i32, i32)> = match to.1 {
            0 => {
                let col = to_col * edge_length as i32 + edge_length as i32 - 1;
                let row = to_row * edge_length as i32;
                (0..edge_length)
                    .rev()
                    .map(|i| (row + i as i32, col, 2))
                    .collect()
            }
            1 => {
                let col = to_col * edge_length as i32;
                let row = to_row * edge_length as i32 + edge_length as i32 - 1;
                (0..edge_length).map(|i| (row, col + i as i32, 3)).collect()
            }
            2 => {
                let col = to_col * edge_length as i32;
                let row = to_row * edge_length as i32;
                (0..edge_length).map(|i| (row + i as i32, col, 0)).collect()
            }
            3 => {
                let col = to_col * edge_length as i32;
                let row = to_row * edge_length as i32;
                (0..edge_length)
                    .rev()
                    .map(|i| (row, col + i as i32, 1))
                    .collect()
            }
            _ => panic!("invalid"),
        };

        match from.1 {
            0 => {
                let col = from_col * edge_length as i32 + edge_length as i32 - 1;
                let row = from_row * edge_length as i32;
                (0..edge_length).zip(to_vec.iter()).for_each(|(i, to)| {
                    let from_coordinate = (row + i as i32, col);
                    let tile = cube_map.get_mut(&from_coordinate).unwrap();
                    tile.east = *to;
                });
            }
            1 => {
                let col = from_col * edge_length as i32;
                let row = from_row * edge_length as i32 + edge_length as i32 - 1;
                (0..edge_length)
                    .rev()
                    .zip(to_vec.iter())
                    .for_each(|(i, to)| {
                        let from_coordinate = (row, col + i as i32);
                        let tile = cube_map.get_mut(&from_coordinate).unwrap();
                        tile.south = *to;
                    });
            }
            2 => {
                let col = from_col * edge_length as i32;
                let row = from_row * edge_length as i32;
                (0..edge_length)
                    .rev()
                    .zip(to_vec.iter())
                    .for_each(|(i, to)| {
                        let from_coordinate = (row + i as i32, col);
                        let tile = cube_map.get_mut(&from_coordinate).unwrap();
                        tile.west = *to;
                    });
            }
            3 => {
                let col = from_col * edge_length as i32;
                let row = from_row * edge_length as i32;
                (0..edge_length).zip(to_vec.iter()).for_each(|(i, to)| {
                    let from_coordinate = (row, col + i as i32);
                    let tile = cube_map.get_mut(&from_coordinate).unwrap();
                    tile.north = *to;
                });
            }
            _ => panic!("invalid"),
        }
    }

    cube_map
}

fn get_password(
    monkey_map: &MonkeyMap,
    instructions: &Instructions,
    start_position: &(i32, i32, i32),
) -> i32 {
    let mut position = *start_position;
    let mut current_tile = monkey_map.get(&(position.0, position.1)).unwrap();

    for instruction in instructions {
        match instruction {
            Instruction::Move(distance) => {
                for _ in 0..*distance {
                    let next_coordinate = match position.2 {
                        0 => current_tile.east,
                        1 => current_tile.south,
                        2 => current_tile.west,
                        3 => current_tile.north,
                        _ => panic!("invalid"),
                    };
                    let next_tile = monkey_map
                        .get(&(next_coordinate.0, next_coordinate.1))
                        .unwrap();
                    if next_tile.tile_type == TileType::Open {
                        position = next_coordinate;
                        current_tile = next_tile;
                    } else {
                        // else, next is wall and we don't move
                        break;
                    }
                }
            }
            Instruction::TurnLeft => {
                position.2 += 3;
                position.2 %= 4;
            }
            Instruction::TurnRight => {
                position.2 += 1;
                position.2 %= 4;
            }
        }
    }
    let final_row = position.0 + 1;
    let final_col = position.1 + 1;
    let final_facing = position.2;
    1000 * final_row + 4 * final_col + final_facing
}

fn main() {
    println!("Hello, day 22!");

    let (monkey_map, instructions, start_position) = parse_input("./input/22/input.txt");
    let password = get_password(&monkey_map, &instructions, &start_position);
    println!("Part 1: {}", password);

    let cube_map = get_cube_map(&monkey_map, 50);
    let password = get_password(&cube_map, &instructions, &start_position);
    println!("Part 2: {}", password);
}
