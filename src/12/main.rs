use petgraph::algo::dijkstra;
use petgraph::graph::{Graph, NodeIndex};
use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = parse_input("./src/12/test.txt");
        assert_eq!(shortest_path_distance(&input), 31);
    }

    #[test]
    fn find_closest_a() {
        let input = parse_input("./src/12/test.txt");
        assert_eq!(shortest_hiking_distance(&input), 29);
    }
}

fn map_letter(input: char) -> char {
    match input {
        'S' => 'a',
        'E' => 'z',
        other => other,
    }
}
fn possible(from: char, to: char) -> bool {
    // This check assumes pathfinding from finish to start
    let a = map_letter(from);
    let b = map_letter(to);
    let diff = (b as i32) - (a as i32);
    diff >= -1
}

fn connect_if_possible(graph: &mut Graph<char, ()>, index_a: &NodeIndex, index_b: &NodeIndex) {
    let a = *graph.node_weight(*index_a).unwrap();
    let b = *graph.node_weight(*index_b).unwrap();
    if possible(a, b) {
        graph.add_edge(*index_a, *index_b, ());
    }
    if possible(b, a) {
        graph.add_edge(*index_b, *index_a, ());
    }
}

fn parse_input(path: &str) -> Graph<char, ()> {
    let input = std::fs::read_to_string(path).unwrap();

    let mut graph = Graph::new();
    let mut grid = HashMap::<(usize, usize), NodeIndex>::new();
    let mut width = 0;
    let mut height = 0;

    // Create nodes
    for (y, line) in input.lines().enumerate() {
        height = y + 1;
        for (x, value) in line.chars().enumerate() {
            width = x + 1;

            let node = graph.add_node(value);
            grid.insert((x, y), node);
        }
    }

    // Connect edges
    for y in 0..height {
        for x in 0..width {
            // check neighbors
            if x > 0 {
                connect_if_possible(
                    &mut graph,
                    grid.get(&(x, y)).unwrap(),
                    grid.get(&(x - 1, y)).unwrap(),
                );
            }
            if y > 0 {
                connect_if_possible(
                    &mut graph,
                    grid.get(&(x, y)).unwrap(),
                    grid.get(&(x, y - 1)).unwrap(),
                );
            }
        }
    }
    graph
}

fn find_node_index(graph: &Graph<char, ()>, value: char) -> NodeIndex {
    graph
        .node_indices()
        .find(|i| {
            let node = graph.node_weight(*i).unwrap();
            *node == value
        })
        .unwrap()
}

fn shortest_path_distance(graph: &Graph<char, ()>) -> i32 {
    // Search from finish to start which is easier for part 2
    let start = find_node_index(graph, 'E');
    let finish = find_node_index(graph, 'S');
    let shortest_path = dijkstra(graph, start, Some(finish), |_| 1);
    *shortest_path.get(&finish).unwrap()
}

fn shortest_hiking_distance(graph: &Graph<char, ()>) -> i32 {
    let start = find_node_index(graph, 'E');
    let distances = dijkstra(graph, start, None, |_| 1);
    distances
        .iter()
        .map(|(i, distance)| (graph.node_weight(*i).unwrap(), distance))
        .filter(|(value, _)| **value == 'a')
        .map(|(_, distance)| *distance)
        .min()
        .unwrap()
}

fn main() {
    println!("Hello, day 12!");

    let input = parse_input("./input/12/input.txt");
    let distance = shortest_path_distance(&input);
    println!("Part 1: {}", distance);

    let distance = shortest_hiking_distance(&input);
    println!("Part 2: {}", distance);
}
