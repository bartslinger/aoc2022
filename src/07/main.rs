use petgraph::dot::{Config, Dot};
use petgraph::{Direction, Graph};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_parsing() {
        let graph = parse_input("./src/07/test.txt");
        println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));
        assert!(false);
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

fn main() {
    println!("Hello, day 7!");
}
