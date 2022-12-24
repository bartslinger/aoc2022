use itertools::Itertools;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mixing() {
        let input = parse_input("./src/20/test.txt");

        assert_eq!(mix(&input, 1), vec![1, 2, -3, 4, 0, 3, -2]);
        assert_eq!(
            mix_with_decryption_key(&input, 1),
            vec![
                0,
                -2434767459,
                3246356612,
                -1623178306,
                2434767459,
                1623178306,
                811589153
            ]
        );
    }
    #[test]
    fn test_grove_coordinates() {
        let input = parse_input("./src/20/test.txt");
        let mixed = mix(&input, 1);

        assert_eq!(get_grove_coordinates(&mixed), 3);
    }

    #[test]
    fn test_with_decryption_key() {
        let input = parse_input("./src/20/test.txt");
        let mixed = mix_with_decryption_key(&input, 10);
        assert_eq!(get_grove_coordinates(&mixed), 1623178306);
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

fn mix(input: &[i64], rounds: usize) -> Vec<i64> {
    let mut items: Vec<Item> = input
        .iter()
        .enumerate()
        .map(|(position, number)| Item {
            position: position as i64,
            number: *number,
        })
        .collect();
    for _ in 0..rounds {
        for i in 0..items.len() {
            let copied_item = *items.get(i).unwrap();
            let old_index = copied_item.position;
            let mut new_index =
                (copied_item.position + copied_item.number) % (items.len() - 1) as i64;
            if new_index == old_index {
            } else if new_index <= 0 {
                new_index += items.len() as i64 - 1;
            } else if new_index > items.len() as i64 {
                new_index -= items.len() as i64 - 1;
            }
            assert!(new_index >= 0);
            assert!(new_index < input.len() as i64);

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
    }

    let sorted: Vec<Item> = items
        .into_iter()
        .sorted_by(|a, b| a.position.cmp(&b.position))
        .collect();

    sorted.iter().map(|i| i.number).collect()
}

fn mix_with_decryption_key(input: &[i64], rounds: usize) -> Vec<i64> {
    // multiply with decryption key
    let input: Vec<i64> = input.iter().map(|x| x * 811589153).collect();
    mix(&input, rounds)
}

fn get_grove_coordinates(input: &[i64]) -> i64 {
    let zero_position = input
        .iter()
        .find_position(|number| **number == 0)
        .unwrap()
        .0;
    let a = input.get((1000 + zero_position) % input.len()).unwrap();
    let b = input.get((2000 + zero_position) % input.len()).unwrap();
    let c = input.get((3000 + zero_position) % input.len()).unwrap();
    a + b + c
}

fn main() {
    println!("Hello, day 20!");

    let input = parse_input("./input/20/input.txt");
    let mixed = mix(&input, 1);
    let grove_coordinates = get_grove_coordinates(&mixed);
    println!("Part 1: {}", grove_coordinates);

    let mixed = mix_with_decryption_key(&input, 10);
    let grove_coordinates = get_grove_coordinates(&mixed);
    println!("Part 2: {}", grove_coordinates);
}
