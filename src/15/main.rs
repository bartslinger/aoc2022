use std::collections::HashSet;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_coverage_for_row() {
        let input = parse_input("./src/15/test.txt");

        assert_eq!(coverage_in_row(&input, 10), 26);
    }
}

type Coordinate = (i64, i64);

fn parse_input(path: &str) -> Vec<(Coordinate, Coordinate)> {
    let input = std::fs::read_to_string(path).unwrap();
    let mut output = vec![];
    for line in input.lines() {
        let line = line.replace("Sensor at x=", "");
        let line = line.replace(" y=", "");
        let line = line.replace(": closest beacon is at x=", ",");
        let mut parts = line.split(',');
        let pair = (
            (
                parts.next().unwrap().parse().unwrap(),
                parts.next().unwrap().parse().unwrap(),
            ),
            (
                parts.next().unwrap().parse().unwrap(),
                parts.next().unwrap().parse().unwrap(),
            ),
        );
        output.push(pair);
    }
    output
}

fn manhatten_distance(a: (i64, i64), b: (i64, i64)) -> i64 {
    let dx = b.0 - a.0;
    let dy = b.1 - a.1;
    dx.abs() + dy.abs()
}

fn overlaps(a: &(i64, i64), b: &(i64, i64)) -> bool {
    a.1 >= b.0
}

fn merge(a: &(i64, i64), b: &(i64, i64)) -> (i64, i64) {
    (a.0, b.1)
}

fn coverage_in_row(input: &Vec<(Coordinate, Coordinate)>, row: i64) -> i64 {
    let mut coverage = Vec::<(i64, i64)>::new();
    for pair in input {
        let sensor = pair.0;
        let beacon = pair.1;
        let range = manhatten_distance(sensor, beacon);
        let dy = row - sensor.1;
        if range >= dy.abs() {
            let remaining = range - dy.abs();
            let left = sensor.0 - remaining;
            let right = sensor.0 + remaining;
            coverage.push((left, right));
        }
    }
    // Sort coverage ranges
    coverage.sort_by(|a, b| a.0.cmp(&b.0));
    let mut merged_coverage = vec![coverage[0]];
    for range in coverage {
        if overlaps(&range, merged_coverage.last().unwrap()) {
            // overwrite
            let previous = merged_coverage.pop().unwrap();
            let new = merge(&previous, &range);
            merged_coverage.push(new);
        }
    }
    merged_coverage.iter().map(|(a, b)| b - a).sum()
}

fn main() {
    println!("Hello, day 15!");

    let pairs = parse_input("./input/15/input.txt");
    let coverage = coverage_in_row(&pairs, 2000000);
    println!("Part 1: {}", coverage);
}
