extern crate core;

use ndarray::{ArrayView, Axis};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visible_sides() {
        let forest = parse_input("./src/08/test.txt");
        assert_eq!(visible_sides(&forest, [1, 1]), 2);
        assert_eq!(visible_sides(&forest, [1, 2]), 2);
        assert_eq!(visible_sides(&forest, [1, 3]), 0);
        assert_eq!(visible_sides(&forest, [2, 1]), 1);
        assert_eq!(visible_sides(&forest, [2, 2]), 0);
        assert_eq!(visible_sides(&forest, [2, 3]), 1);
        assert_eq!(visible_sides(&forest, [3, 1]), 0);
        assert_eq!(visible_sides(&forest, [3, 2]), 2);
        assert_eq!(visible_sides(&forest, [3, 3]), 0);

        assert_eq!(visible_sides(&forest, [0, 0]), 2);
        assert_eq!(visible_sides(&forest, [4, 3]), 4);
    }

    #[test]
    fn test_part_one() {
        let forest = parse_input("./src/08/test.txt");
        assert_eq!(visible(forest), 21);
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

fn visible_sides(forest: &ndarray::Array2<u8>, coordinate: [usize; 2]) -> usize {
    let dim = forest.dim();
    let value = forest[coordinate];
    let mut visible_sides = 4;
    // same row to the west
    for x in 0..coordinate[1] {
        if forest[[coordinate[0], x]] >= value {
            visible_sides -= 1;
            break;
        }
    }
    // same row to the east
    for x in (coordinate[1] + 1)..dim.1 {
        if forest[[coordinate[0], x]] >= value {
            visible_sides -= 1;
            break;
        }
    }
    // same column to the north
    for y in 0..coordinate[0] {
        if forest[[y, coordinate[1]]] >= value {
            visible_sides -= 1;
            break;
        }
    }
    // same column to the south
    for y in (coordinate[0] + 1)..dim.0 {
        if forest[[y, coordinate[1]]] >= value {
            visible_sides -= 1;
            break;
        }
    }
    visible_sides
}

fn visible(forest: ndarray::Array2<u8>) -> usize {
    let (rows, cols) = forest.dim();
    let mut visible_trees = 0;
    for i in 0..cols {
        for j in 0..rows {
            if visible_sides(&forest, [i, j]) > 0 {
                visible_trees += 1;
            }
        }
    }
    visible_trees
}

fn main() {
    println!("Hello, day 8!");

    let input = parse_input("./input/08/input.txt");
    let visible_trees = visible(input);
    println!("Part 1: {}", visible_trees);
}
