use itertools::Itertools;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mixing() {
        let input = parse_input("./src/20/test.txt");

        assert_eq!(mix(&input), vec![1, 2, -3, 4, 0, 3, -2]);
    }
    #[test]
    fn test_grove_coordinates() {
        let input = parse_input("./src/20/test.txt");

        assert_eq!(get_grove_coordinates(&input), 3);
    }
}

fn parse_input(path: &str) -> Vec<i64> {
    let input = std::fs::read_to_string(path).unwrap();
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[derive(Copy, Clone, Debug)]
struct Item {
    position: i64,
    number: i64,
}

fn mix(input: &[i64]) -> Vec<i64> {
    let mut items: Vec<Item> = input
        .iter()
        .enumerate()
        .map(|(position, number)| Item {
            position: position as i64,
            number: *number,
        })
        .collect();
    for i in 0..items.len() {
        let copied_item = *items.get(i).unwrap();
        let old_index = copied_item.position;
        let mut new_index = copied_item.position + (copied_item.number % (items.len() - 1) as i64);
        if new_index <= 0 {
            new_index += items.len() as i64 - 1;
        }
        if new_index > items.len() as i64 {
            new_index -= items.len() as i64 - 1;
        }

        for item in items.iter_mut() {
            if item.position == old_index {
                item.position = new_index;
            } else if item.position > old_index && item.position <= new_index {
                item.position -= 1;
            } else if item.position >= new_index && item.position < old_index {
                item.position += 1;
            }
        }
    }

    let sorted: Vec<Item> = items
        .into_iter()
        .sorted_by(|a, b| a.position.cmp(&b.position))
        .collect();

    sorted.iter().map(|i| i.number).collect()
}

fn get_grove_coordinates(input: &[i64]) -> i64 {
    let mixed = mix(input);
    let zero_position = mixed
        .iter()
        .find_position(|number| **number == 0)
        .unwrap()
        .0;
    let a = mixed.get((1000 + zero_position) % mixed.len()).unwrap();
    let b = mixed.get((2000 + zero_position) % mixed.len()).unwrap();
    let c = mixed.get((3000 + zero_position) % mixed.len()).unwrap();
    a + b + c
}

fn main() {
    println!("Hello, day 20!");

    let input = parse_input("./input/20/input.txt");
    let grove_coordinates = get_grove_coordinates(&input);
    println!("Part 1: {}", grove_coordinates);
}
