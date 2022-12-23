use std::collections::HashSet;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_surface_area() {
        let input = parse_input("./src/18/test.txt");

        assert_eq!(surface_area(input), 64);
    }
}

fn parse_input(path: &str) -> HashSet<(i32, i32, i32)> {
    let input = std::fs::read_to_string(path).unwrap();

    let mut set = HashSet::new();
    for line in input.lines() {
        let mut numbers = line.split(',');
        let x: i32 = numbers.next().unwrap().parse().unwrap();
        let y: i32 = numbers.next().unwrap().parse().unwrap();
        let z: i32 = numbers.next().unwrap().parse().unwrap();
        set.insert((x, y, z));
    }

    set
}

fn surface_area(input: HashSet<(i32, i32, i32)>) -> usize {
    let mut surface_area = input.len() * 6;

    for cube in input.iter() {
        // subtract 1 surface area for each adjecent cube
        if input.get(&(cube.0 - 1, cube.1, cube.2)).is_some() {
            surface_area -= 1;
        }
        if input.get(&(cube.0 + 1, cube.1, cube.2)).is_some() {
            surface_area -= 1;
        }
        if input.get(&(cube.0, cube.1 - 1, cube.2)).is_some() {
            surface_area -= 1;
        }
        if input.get(&(cube.0, cube.1 + 1, cube.2)).is_some() {
            surface_area -= 1;
        }
        if input.get(&(cube.0, cube.1, cube.2 - 1)).is_some() {
            surface_area -= 1;
        }
        if input.get(&(cube.0, cube.1, cube.2 + 1)).is_some() {
            surface_area -= 1;
        }
    }

    surface_area
}

fn main() {
    println!("Hello, day 18!");

    let input = parse_input("./input/18/input.txt");
    let area = surface_area(input);
    println!("Part 1: {}", area);
}
