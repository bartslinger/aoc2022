use ndarray::Dim;
use std::collections::{HashMap, HashSet};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let (blizzards, dimensions) = parse_input("./src/24/test.txt");

        let extrapolated = extrapolate_blizzards(&blizzards, &dimensions, 20);
        let options = calculate_options(&extrapolated, &dimensions);
        let mut state = State {
            current_best: usize::MAX,
            seen: HashSet::new(),
        };
        find_fastest_path(&options, 1, &(0, 1), &dimensions, &mut state);

        assert_eq!(state.current_best, 18);
    }
}
#[derive(Debug)]
struct Dimensions {
    rows: i32,
    cols: i32,
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
struct Blizzard {
    dir: char,
    row: i32,
    col: i32,
}

fn parse_input(path: &str) -> (Vec<Blizzard>, Dimensions) {
    let input = std::fs::read_to_string(path).unwrap();
    let rows = input.lines().count() as i32;
    let cols = input.lines().next().unwrap().chars().count() as i32;

    let mut blizzards = Vec::<Blizzard>::new();
    for (row, line) in input.lines().enumerate() {
        for (col, item) in line.chars().enumerate() {
            if item == '^' || item == 'v' || item == '>' || item == '<' {
                blizzards.push(Blizzard {
                    dir: item,
                    row: row as i32,
                    col: col as i32,
                });
            }
        }
    }

    (blizzards, Dimensions { rows, cols })
}

#[allow(unused)]
fn draw(blizzards: &Vec<Blizzard>, dimensions: &Dimensions) {
    for row in 0..dimensions.rows {
        for col in 0..dimensions.cols {
            if row == 0 && col == 1 {
                print!(".");
                continue;
            }
            if row == dimensions.rows - 1 && col == dimensions.cols - 2 {
                print!(".");
                continue;
            }
            if row == 0 || row == dimensions.rows - 1 || col == 0 || col == dimensions.cols - 1 {
                print!("#");
            } else {
                let items: Vec<char> = blizzards
                    .iter()
                    .filter(|blizzard| blizzard.row == row && blizzard.col == col)
                    .map(|blizzard| blizzard.dir)
                    .collect();
                match items.len() {
                    2..=usize::MAX => print!("{}", items.len()),
                    1 => print!("{}", items[0]),
                    _ => print!("."),
                }
            }
        }
        println!();
    }
}

#[allow(unused)]
fn draw_occupation(occupation_map: &HashSet<(i32, i32)>, dimensions: &Dimensions) {
    for row in 0..dimensions.rows {
        for col in 0..dimensions.cols {
            if occupation_map.get(&(row, col)).is_some() {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn get_occupation_map(input: &Vec<Blizzard>, dimensions: &Dimensions) -> HashSet<(i32, i32)> {
    let mut occupation_map = HashSet::new();
    // walls are occupied except entry/exit points
    (0..dimensions.rows).for_each(|row| {
        occupation_map.insert((row, 0));
        occupation_map.insert((row, dimensions.cols - 1));
    });
    (0..dimensions.cols)
        .filter(|col| *col != 1)
        .for_each(|col| {
            occupation_map.insert((0, col));
        });
    (0..dimensions.cols)
        .filter(|col| *col != dimensions.cols - 2)
        .for_each(|col| {
            occupation_map.insert((dimensions.rows - 1, col));
        });
    occupation_map.insert((-1, 1));
    occupation_map.insert((dimensions.rows, dimensions.cols - 2));

    for blizzard in input {
        occupation_map.insert((blizzard.row, blizzard.col));
    }

    occupation_map
}

fn extrapolate_blizzards(
    start_blizzards: &Vec<Blizzard>,
    dimensions: &Dimensions,
    depth: usize,
) -> Vec<HashSet<(i32, i32)>> {
    let mut output = Vec::new();
    output.push(get_occupation_map(start_blizzards, dimensions));

    let mut blizzards = start_blizzards.clone();
    for _ in 0..depth {
        let mut new_blizzards = Vec::<Blizzard>::new();
        for blizzard in blizzards {
            let mut updated_blizzard = match blizzard.dir {
                '^' => Blizzard {
                    dir: blizzard.dir,
                    row: blizzard.row - 1,
                    col: blizzard.col,
                },
                'v' => Blizzard {
                    dir: blizzard.dir,
                    row: blizzard.row + 1,
                    col: blizzard.col,
                },
                '>' => Blizzard {
                    dir: blizzard.dir,
                    row: blizzard.row,
                    col: blizzard.col + 1,
                },
                '<' => Blizzard {
                    dir: blizzard.dir,
                    row: blizzard.row,
                    col: blizzard.col - 1,
                },
                _ => panic!("invalid direction"),
            };
            if updated_blizzard.row == 0 {
                updated_blizzard.row = dimensions.rows - 2
            } else if updated_blizzard.row == dimensions.rows - 1 {
                updated_blizzard.row = 1
            } else if updated_blizzard.col == 0 {
                updated_blizzard.col = dimensions.cols - 2
            } else if updated_blizzard.col == dimensions.cols - 1 {
                updated_blizzard.col = 1
            }
            new_blizzards.push(updated_blizzard);
        }
        let occupation_map = get_occupation_map(&new_blizzards, dimensions);
        blizzards = new_blizzards;

        // draw(&blizzards, dimensions);
        // println!();
        // draw_occupation(&occupation_map, dimensions);
        // println!();
        // println!();
        output.push(occupation_map);
    }

    output
}

type Options = HashMap<(i32, i32), Vec<(i32, i32)>>;

fn calculate_options(input: &Vec<HashSet<(i32, i32)>>, dimensions: &Dimensions) -> Vec<Options> {
    let mut output = Vec::new();

    for i in 0..input.len() - 1 {
        let current = input.get(i).unwrap();
        let next = input.get(i + 1).unwrap();
        let mut options = HashMap::<(i32, i32), Vec<(i32, i32)>>::new();

        for row in 0..dimensions.rows {
            for col in 0..dimensions.cols {
                if current.get(&(row, col)).is_none() {
                    // Thise tile is free, find options from here to next
                    let mut tile_options = vec![];
                    let coordinates = [
                        (row, col),
                        (row + 1, col),
                        (row - 1, col),
                        (row, col + 1),
                        (row, col - 1),
                    ];
                    for coordinate in coordinates {
                        if next.get(&coordinate).is_none() {
                            tile_options.push(coordinate);
                        }
                    }
                    options.insert((row, col), tile_options);
                }
            }
        }
        output.push(options);
    }

    output
}

struct State {
    current_best: usize,
    seen: HashSet<(i32, i32, usize)>,
}

fn find_fastest_path(
    input: &Vec<Options>,
    time: usize,
    start_position: &(i32, i32),
    dimensions: &Dimensions,
    mut state: &mut State,
) {
    let options = input.get(time - 1).unwrap().get(start_position).unwrap();

    for position in options {
        if state.seen.get(&(position.0, position.1, time)).is_some() {
            continue;
        }
        state.seen.insert((position.0, position.1, time));
        if position.0 == dimensions.rows - 1 {
            state.current_best = state.current_best.min(time);
        }
        if time < input.len() - 1 && time < state.current_best {
            find_fastest_path(input, time + 1, position, dimensions, state);
        }
    }
}

fn main() {
    println!("Hello, day 24!");

    let (blizzards, dimensions) = parse_input("./input/24/input.txt");

    let extrapolated = extrapolate_blizzards(&blizzards, &dimensions, 500);
    let options = calculate_options(&extrapolated, &dimensions);

    let mut state = State {
        current_best: usize::MAX,
        seen: HashSet::new(),
    };
    find_fastest_path(&options, 1, &(0, 1), &dimensions, &mut state);
    println!("Part 1: {}", state.current_best);
}
