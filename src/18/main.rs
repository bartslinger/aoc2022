use std::collections::HashSet;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_surface_area() {
        let input = parse_input("./src/18/test.txt");

        assert_eq!(surface_area(&input), 64);
    }

    #[test]
    fn test_exterior_surface_area() {
        let input = parse_input("./src/18/test.txt");

        assert_eq!(exterior_surface_area(&input), 58);
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
        if x < 0 || y < 0 || z < 0 || x >= 30 || y >= 30 || z >= 30 {
            panic!("Expect all to be within 0 and 30");
        }
        set.insert((x, y, z));
    }

    set
}

fn surface_area(input: &HashSet<(i32, i32, i32)>) -> usize {
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

fn exterior_surface_area(droplet: &HashSet<(i32, i32, i32)>) -> usize {
    // create set of air around the lava droplet, starting at 0,0,0 and expanding to 30,30,30
    let mut outside_air = HashSet::<(i32, i32, i32)>::new();
    let mut next_outside_air = HashSet::<(i32, i32, i32)>::new();
    next_outside_air.insert((-1, -1, -1));
    while !next_outside_air.is_empty() {
        let mut next = HashSet::<(i32, i32, i32)>::new();
        for item in next_outside_air.iter() {
            let to_consider = vec![
                (item.0 - 1, item.1, item.2),
                (item.0 + 1, item.1, item.2),
                (item.0, item.1 - 1, item.2),
                (item.0, item.1 + 1, item.2),
                (item.0, item.1, item.2 - 1),
                (item.0, item.1, item.2 + 1),
            ];
            for coordinate in to_consider {
                if coordinate.0 >= -1
                    && coordinate.0 < 30
                    && coordinate.1 >= -1
                    && coordinate.1 < 30
                    && coordinate.2 >= -1
                    && coordinate.2 < 30
                    && droplet.get(&coordinate).is_none()
                    && outside_air.get(&coordinate).is_none()
                    && next_outside_air.get(&coordinate).is_none()
                {
                    next.insert(coordinate);
                }
            }

            // save the current value to outside air
            outside_air.insert(*item);
        }
        next_outside_air = next;
    }

    let outside_air_surface = surface_area(&outside_air);
    let outside_air_cube_surface = (30 + 1) * (30 + 1) * 6;
    outside_air_surface - outside_air_cube_surface
}

fn main() {
    println!("Hello, day 18!");

    let input = parse_input("./input/18/input.txt");
    let area = surface_area(&input);
    println!("Part 1: {}", area);

    let area = exterior_surface_area(&input);
    println!("Part 2: {}", area);
}
