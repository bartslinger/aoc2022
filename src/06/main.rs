use itertools::Itertools;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        assert_eq!(find_first_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
        assert_eq!(find_first_marker("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(find_first_marker("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(find_first_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(find_first_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }
}

fn find_first_marker(input: &str) -> usize {
    let input = input.to_string().chars().collect::<Vec<char>>();
    for (index, window) in input.windows(4).enumerate() {
        if window.iter().unique().count() == 4 {
            return index + 4;
        }
    }
    panic!("no marker found");
}

fn main() {
    println!("Hello, day 6!");

    let input = std::fs::read_to_string("./input/06/input.txt").unwrap();
    let marker_id = find_first_marker(&input);
    println!("Part 1: {}", marker_id);
}
