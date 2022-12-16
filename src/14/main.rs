use itertools::Itertools;
use std::collections::HashSet;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_sand_pile() {
        let rock = parse_input("./src/14/test.txt");
        assert_eq!(count_sand_pile(&rock), 24);
    }
}

fn parse_coordinate(input: &str) -> (i32, i32) {
    let mut input = input.split(',');
    let x = input.next().unwrap().parse().unwrap();
    let y = input.next().unwrap().parse().unwrap();
    (x, y)
}

fn draw_rock(rock: &mut HashSet<(i32, i32)>, start: (i32, i32), finish: (i32, i32)) {
    let dx = finish.0 - start.0;
    let dy = finish.1 - start.1;
    let steps = dx.abs() + dy.abs();
    for i in 0..=steps {
        let coordinate = (start.0 + i * dx / steps, start.1 + i * dy / steps);
        rock.insert(coordinate);
    }
}

fn parse_input(path: &str) -> HashSet<(i32, i32)> {
    let input = std::fs::read_to_string(path).unwrap();
    let mut rock = HashSet::new();
    for line in input.lines() {
        let corners = line.split(" -> ");
        for segment in corners.tuple_windows::<(&str, &str)>() {
            let start = parse_coordinate(segment.0);
            let finish = parse_coordinate(segment.1);
            draw_rock(&mut rock, start, finish);
        }
    }
    rock
}

fn lowest_rock(rock: &HashSet<(i32, i32)>) -> i32 {
    rock.iter().map(|(_x, y)| *y).max().unwrap()
}

fn drop_sand_unit(rock: &HashSet<(i32, i32)>, sand: &mut HashSet<(i32, i32)>) -> bool {
    let bottom = lowest_rock(rock);
    let mut unit = (500, 0);
    loop {
        if unit.1 > bottom {
            return false;
        }
        let below = (unit.0, unit.1 + 1);
        if rock.get(&below).is_none() && sand.get(&below).is_none() {
            unit = below;
            continue;
        }
        let left = (unit.0 - 1, unit.1 + 1);
        if rock.get(&left).is_none() && sand.get(&left).is_none() {
            unit = left;
            continue;
        }
        let right = (unit.0 + 1, unit.1 + 1);
        if rock.get(&right).is_none() && sand.get(&right).is_none() {
            unit = right;
            continue;
        }
        break;
    }
    sand.insert(unit);
    true
}

fn count_sand_pile(rock: &HashSet<(i32, i32)>) -> usize {
    let mut sand = HashSet::<(i32, i32)>::new();
    while drop_sand_unit(rock, &mut sand) {}
    sand.len()
}

fn main() {
    println!("Hello, day 14!");

    let rock = parse_input("./input/14/input.txt");
    let count = count_sand_pile(&rock);
    println!("Part 1: {}", count);
}
