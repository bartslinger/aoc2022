#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_coverage_for_row() {
        let input = parse_input("./src/15/test.txt");

        let coverage = coverage_in_row(&input, 10);
        assert_eq!(count_coverage_in_row(&coverage), 26);
    }

    #[test]
    fn test_find_distress_beacon() {
        let input = parse_input("./src/15/test.txt");
        assert_eq!(find_distress_beacon(&input, 20), 56000011);
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

fn overlaps_or_connects(a: &(i64, i64), b: &(i64, i64)) -> bool {
    a.1 >= (b.0 - 1)
}

fn merge(a: &(i64, i64), b: &(i64, i64)) -> (i64, i64) {
    let left = std::cmp::min(a.0, b.0);
    let right = std::cmp::max(a.1, b.1);
    (left, right)
}

fn coverage_in_row(input: &Vec<(Coordinate, Coordinate)>, row: i64) -> Vec<(i64, i64)> {
    let mut coverage = Vec::<(i64, i64)>::new();
    for (sensor, beacon) in input {
        let range = manhatten_distance(*sensor, *beacon);
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
        if overlaps_or_connects(merged_coverage.last().unwrap(), &range) {
            // overwrite
            let previous = merged_coverage.pop().unwrap();
            let new = merge(&previous, &range);
            merged_coverage.push(new);
        } else {
            merged_coverage.push(range);
        }
    }
    merged_coverage
}

fn count_coverage_in_row(input: &Vec<(i64, i64)>) -> i64 {
    input.iter().map(|(a, b)| b - a).sum()
}

fn find_distress_beacon(input: &Vec<(Coordinate, Coordinate)>, search_limit: i64) -> i64 {
    for y in 0..search_limit {
        let coverage = coverage_in_row(input, y);
        if coverage.len() > 1 {
            let x = (coverage[0].1 + 1) * 4000000;
            return x + y;
        }
    }
    0
}

fn main() {
    println!("Hello, day 15!");

    let pairs = parse_input("./input/15/input.txt");
    let coverage = coverage_in_row(&pairs, 2000000);
    let coverage_count = count_coverage_in_row(&coverage);
    println!("Part 1: {}", coverage_count);

    let tuning_frequency = find_distress_beacon(&pairs, 4000000);
    println!("Part 2: {}", tuning_frequency);
}
