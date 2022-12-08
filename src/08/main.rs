extern crate core;

use ndarray::{ArrayView, Axis};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visible_sides() {
        let forest = parse_input("./src/08/test.txt");
        assert_eq!(visible_sides_and_score(&forest, [1, 1]).0, 2);
        assert_eq!(visible_sides_and_score(&forest, [1, 2]).0, 2);
        assert_eq!(visible_sides_and_score(&forest, [1, 3]).0, 0);
        assert_eq!(visible_sides_and_score(&forest, [2, 1]).0, 1);
        assert_eq!(visible_sides_and_score(&forest, [2, 2]).0, 0);
        assert_eq!(visible_sides_and_score(&forest, [2, 3]).0, 1);
        assert_eq!(visible_sides_and_score(&forest, [3, 1]).0, 0);
        assert_eq!(visible_sides_and_score(&forest, [3, 2]).0, 2);
        assert_eq!(visible_sides_and_score(&forest, [3, 3]).0, 0);

        assert_eq!(visible_sides_and_score(&forest, [0, 0]).0, 2);
        assert_eq!(visible_sides_and_score(&forest, [4, 3]).0, 4);
    }

    #[test]
    fn test_part_one() {
        let forest = parse_input("./src/08/test.txt");
        assert_eq!(visible(&forest), 21);
    }

    #[test]
    fn test_scenic_score() {
        let forest = parse_input("./src/08/test.txt");
        assert_eq!(visible_sides_and_score(&forest, [1, 2]).1, 4);
        assert_eq!(visible_sides_and_score(&forest, [3, 2]).1, 8);
    }

    #[test]
    fn find_highest_score() {
        let forest = parse_input("./src/08/test.txt");
        assert_eq!(highest_score(&forest), 8);
    }
}

fn parse_input(path: &str) -> ndarray::Array2<u8> {
    let input = std::fs::read_to_string(path).unwrap();
    let cols = input.lines().next().unwrap().chars().count();

    let mut forest = ndarray::Array2::zeros((0, cols));
    for line in input.lines() {
        let chars: Vec<u8> = line
            .chars()
            .map(|x| x.to_digit(10).unwrap() as u8)
            .collect();
        let row = ArrayView::from(chars.as_slice())
            .into_shape((1, cols))
            .unwrap();
        forest.append(Axis(0), row).unwrap();
    }
    forest
}

fn visible_sides_and_score(forest: &ndarray::Array2<u8>, coordinate: [usize; 2]) -> (usize, usize) {
    let dim = forest.dim();
    let value = forest[coordinate];
    let mut visible_sides = 4;
    // same row to the west
    let mut west = 0;
    for x in (0..coordinate[1]).rev() {
        west += 1;
        if forest[[coordinate[0], x]] >= value {
            visible_sides -= 1;
            break;
        }
    }
    // same row to the east
    let mut east = 0;
    for x in (coordinate[1] + 1)..dim.1 {
        east += 1;
        if forest[[coordinate[0], x]] >= value {
            visible_sides -= 1;
            break;
        }
    }
    // same column to the north
    let mut north = 0;
    for y in (0..coordinate[0]).rev() {
        north += 1;
        if forest[[y, coordinate[1]]] >= value {
            visible_sides -= 1;
            break;
        }
    }
    // same column to the south
    let mut south = 0;
    for y in (coordinate[0] + 1)..dim.0 {
        south += 1;
        if forest[[y, coordinate[1]]] >= value {
            visible_sides -= 1;
            break;
        }
    }
    (visible_sides, west * east * north * south)
}

fn visible(forest: &ndarray::Array2<u8>) -> usize {
    let (rows, cols) = forest.dim();
    let mut visible_trees = 0;
    for i in 0..cols {
        for j in 0..rows {
            if visible_sides_and_score(&forest, [i, j]).0 > 0 {
                visible_trees += 1;
            }
        }
    }
    visible_trees
}

fn highest_score(forest: &ndarray::Array2<u8>) -> usize {
    let mut highest = 0;
    let (rows, cols) = forest.dim();
    for i in 0..cols {
        for j in 0..rows {
            let score = visible_sides_and_score(&forest, [i, j]).1;
            if score > highest {
                highest = score;
            }
        }
    }
    highest
}

fn main() {
    println!("Hello, day 8!");

    let input = parse_input("./input/08/input.txt");
    let visible_trees = visible(&input);
    println!("Part 1: {}", visible_trees);

    let highest_score = highest_score(&input);
    println!("Part 2: {}", highest_score);
}
