use itertools::Itertools;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples_part_one() {
        assert_eq!(find_first_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4), 7);
        assert_eq!(find_first_marker("bvwbjplbgvbhsrlpgdmjqwftvncz", 4), 5);
        assert_eq!(find_first_marker("nppdvjthqldpwncqszvftbrmjlhg", 4), 6);
        assert_eq!(
            find_first_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4),
            10
        );
        assert_eq!(find_first_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4), 11);
    }

    #[test]
    fn test_examples_part_two() {
        assert_eq!(find_first_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14), 19);
        assert_eq!(find_first_marker("bvwbjplbgvbhsrlpgdmjqwftvncz", 14), 23);
        assert_eq!(find_first_marker("nppdvjthqldpwncqszvftbrmjlhg", 14), 23);
        assert_eq!(
            find_first_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14),
            29
        );
        assert_eq!(
            find_first_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14),
            26
        );
    }
}

fn find_first_marker(input: &str, distinct_characters: usize) -> usize {
    let input = input.to_string().chars().collect::<Vec<char>>();
    for (index, window) in input.windows(distinct_characters).enumerate() {
        if window.iter().unique().count() == distinct_characters {
            return index + distinct_characters;
        }
    }
    panic!("no marker found");
}

fn main() {
    println!("Hello, day 6!");

    let input = std::fs::read_to_string("./input/06/input.txt").unwrap();
    let marker_id = find_first_marker(&input, 4);
    println!("Part 1: {}", marker_id);

    let marker_id = find_first_marker(&input, 14);
    println!("Part 2: {}", marker_id);
}
