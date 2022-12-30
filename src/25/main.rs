#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conversion() {
        assert_eq!(snafu_to_decimal("1=-0-2"), 1747);
        assert_eq!(snafu_to_decimal("2=0="), 198);
        assert_eq!(snafu_to_decimal("1121-1110-1=0"), 314159265);
    }

    #[test]
    fn test_example() {
        let input = parse_input("./src/25/test.txt");

        let sum = calculate_sum(&input);
        assert_eq!(sum, "2=-1=0");
    }
}

fn parse_input(path: &str) -> Vec<String> {
    let input = std::fs::read_to_string(path).unwrap();
    let lines = input.lines().map(|x| x.to_string()).collect();
    lines
}

fn snafu_to_decimal(input: &str) -> i64 {
    let mut number = 0;
    for (pos, char) in input.chars().rev().enumerate() {
        let multiplier = 5_i64.pow(pos as u32);
        let x: i64 = match char {
            '=' => -2,
            '-' => -1,
            '0' => 0,
            '1' => 1,
            '2' => 2,
            _ => panic!("invalid character"),
        };
        number += multiplier * x;
    }
    number
}

fn decimal_to_snafu(input: i64) -> String {
    let mut chars = vec![];

    let mut todo = input;
    while todo != 0 {
        let mut remainder = todo % 5;
        if remainder > 2 {
            remainder -= 5;
        }
        todo -= remainder;
        todo /= 5;
        let character = match remainder {
            -2 => '=',
            -1 => '-',
            0 => '0',
            1 => '1',
            2 => '2',
            _ => panic!("invalid conversion"),
        };
        chars.push(character);
    }
    chars.iter().rev().collect()
}

fn calculate_sum(input: &Vec<String>) -> String {
    let sum = input
        .iter()
        .map(|snafu| snafu_to_decimal(snafu.as_str()))
        .sum();

    decimal_to_snafu(sum)
}

fn main() {
    println!("Hello, day 25!");

    let input = parse_input("./input/25/input.txt");
    let sum = calculate_sum(&input);
    println!("Answer: {}", sum);
}
