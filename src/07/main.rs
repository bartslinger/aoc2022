use itertools::Itertools;
// use petgraph::dot::{Config, Dot};
use petgraph::graph::NodeIndex;
use petgraph::{Direction, Graph};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_dir_sizes() {
        let graph = parse_input("./src/07/test.txt");
        let e = node_index(&graph, "e");
        let a = node_index(&graph, "a");
        let d = node_index(&graph, "d");
        let root = node_index(&graph, "/");
        assert_eq!(dir_size(&graph, e), 584);
        assert_eq!(dir_size(&graph, a), 94853);
        assert_eq!(dir_size(&graph, d), 24933642);
        assert_eq!(dir_size(&graph, root), 48381165);
    }

    #[test]
    fn test_part_one() {
        let graph = parse_input("./src/07/test.txt");
        assert_eq!(part_one(&graph), 95437);
    }

    #[test]
    fn test_part_two() {
        let graph = parse_input("./src/07/test.txt");
        assert_eq!(smallest_directory_to_delete(&graph), 24933642);
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Type {
    Dir(String),
    File((usize, String)),
}

fn parse_input(path: &str) -> Graph<Type, i32> {
    let input = std::fs::read_to_string(path).unwrap();
    let mut graph = Graph::new();
    let root = graph.add_node(Type::Dir("/".to_string()));
    let mut current_dir = root;

    for line in input.lines() {
        let line: Vec<&str> = line.split(' ').collect();
        match (line.first().unwrap(), line.get(1).unwrap()) {
            (&"$", &"ls") => { /* ignore */ }
            (&"$", &"cd") => {
                let dir = line.get(2).unwrap();
                match *dir {
                    "/" => current_dir = root,
                    ".." => {
                        current_dir = graph
                            .neighbors_directed(current_dir, Direction::Incoming)
                            .next()
                            .unwrap();
                    }
                    dir => {
                        current_dir = graph
                            .neighbors_directed(current_dir, Direction::Outgoing)
                            .find(|i| {
                                let node = graph.node_weight(*i).unwrap();
                                *node == Type::Dir(dir.to_string())
                            })
                            .unwrap();
                    }
                }
            }
            (&"dir", name) => {
                let dir = graph.add_node(Type::Dir(name.to_string()));
                graph.add_edge(current_dir, dir, 1);
            }
            (size, name) => {
                let file = graph.add_node(Type::File((size.parse().unwrap(), name.to_string())));
                graph.add_edge(current_dir, file, 1);
            }
        }
    }
    graph
}

fn node_index(graph: &Graph<Type, i32>, name: &str) -> NodeIndex {
    graph
        .node_indices()
        .find(|i| graph.node_weight(*i).unwrap() == &Type::Dir(name.to_string()))
        .unwrap()
}

fn dir_size(graph: &Graph<Type, i32>, node: NodeIndex) -> usize {
    let children = graph.neighbors_directed(node, Direction::Outgoing);
    let mut total = 0;
    for child in children {
        let child_node = graph.node_weight(child).unwrap();
        match child_node {
            Type::Dir(_) => {
                total += dir_size(graph, child);
            }
            Type::File((size, _)) => {
                total += size;
            }
        }
    }
    total
}

fn part_one(graph: &Graph<Type, i32>) -> usize {
    graph
        .node_indices()
        .filter(|i| {
            let node = graph.node_weight(*i).unwrap();
            matches!(node, Type::Dir(_))
        })
        .map(|i| dir_size(graph, i))
        .filter(|size| *size <= 100000)
        .sum()
}

fn smallest_directory_to_delete(graph: &Graph<Type, i32>) -> usize {
    let root = node_index(graph, "/");
    let max_allowed = 40000000;
    let used = dir_size(graph, root);
    let to_free = used - max_allowed;

    graph
        .node_indices()
        .filter(|i| {
            let node = graph.node_weight(*i).unwrap();
            matches!(node, Type::Dir(_))
        })
        .map(|i| (dir_size(graph, i), i))
        .map(|(size, _)| size)
        .sorted()
        .find(|size| *size >= to_free)
        .unwrap()
}

fn main() {
    println!("Hello, day 7!");

    let graph = parse_input("./input/07/input.txt");
    let part_one_sum = part_one(&graph);
    println!("Part 1: {}", part_one_sum);

    let part_two = smallest_directory_to_delete(&graph);
    println!("Part 2: {}", part_two);
}
