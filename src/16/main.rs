use itertools::Itertools;
use petgraph::algo::dijkstra;
use petgraph::graph::{NodeIndex, UnGraph};
use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flow_calculation() {
        let valves = parse_input("./src/16/test.txt");
        let distances = calculate_distances(&valves);

        let start = valves.get(0).unwrap(); // AA
        let ordered_valves = vec![
            valves.get(3).unwrap(), // DD
            valves.get(1).unwrap(), // BB
            valves.get(9).unwrap(), // JJ
            valves.get(7).unwrap(), // HH
            valves.get(4).unwrap(), // EE
            valves.get(2).unwrap(), // CC
        ];
        let flow = calculate_pressure_release(&distances, start, ordered_valves);
        assert_eq!(flow, 1651);
    }

    #[test]
    fn test_find_highest_flow() {
        let valves = parse_input("./src/16/test.txt");
        let distances = calculate_distances(&valves);

        let most_pressure = find_most_pressure(&distances, &valves);
        assert_eq!(most_pressure, 1651);
    }
}

#[derive(Debug, Hash, Eq, PartialEq)]
struct Valve {
    name: String,
    flow_rate: u32,
    tunnels: Vec<String>,
}

fn parse_input(path: &str) -> Vec<Valve> {
    let input = std::fs::read_to_string(path).unwrap();
    let mut valves = vec![];
    for line in input.lines() {
        let name = line[6..8].to_string();
        let line = &line[23..];
        let line = line.replace("; tunnels lead to valves ", ", ");
        let line = line.replace("; tunnel leads to valve ", ", ");
        let mut line = line.split(", ");
        let flow_rate = line.next().unwrap().parse().unwrap();
        let tunnels: Vec<String> = line.map(|item| item.to_string()).collect();
        let valve = Valve {
            name,
            flow_rate,
            tunnels,
        };
        valves.push(valve);
    }
    valves
}

fn node_index(graph: &UnGraph<&Valve, i32>, name: &String) -> NodeIndex {
    graph
        .node_indices()
        .find(|i| {
            let node = graph.node_weight(*i).unwrap();
            &node.name == name
        })
        .unwrap()
}

fn relevant_valves(valves: &[Valve]) -> Vec<&Valve> {
    valves
        .iter()
        .filter(|valve| valve.name == "AA" || valve.flow_rate > 0)
        .collect()
}

fn calculate_distances(valves: &Vec<Valve>) -> HashMap<(&Valve, &Valve), u32> {
    let mut graph = UnGraph::<&Valve, i32>::new_undirected();
    for valve in valves {
        graph.add_node(valve);
    }
    for valve in valves {
        let start_node = node_index(&graph, &valve.name);

        let tunnels = &valve.tunnels;
        for tunnel in tunnels {
            // Find node index of tunnel destination
            let destination_node = node_index(&graph, tunnel);
            graph.add_edge(start_node, destination_node, 1);
        }
    }

    let mut distance_map = HashMap::<(&Valve, &Valve), u32>::new();
    for valve in relevant_valves(valves) {
        let start_node = node_index(&graph, &valve.name);
        let distances = dijkstra(&graph, start_node, None, |_| 1);
        for (destination_index, distance) in distances {
            let destination_node = graph.node_weight(destination_index).unwrap();
            if (destination_node.name == "AA" || destination_node.flow_rate > 0) && distance > 0 {
                distance_map.insert((valve, destination_node), distance);
            }
        }
    }
    distance_map
}

fn calculate_pressure_release(
    distance_map: &HashMap<(&Valve, &Valve), u32>,
    start: &Valve,
    valves: Vec<&Valve>,
) -> u32 {
    let mut flow_rate = 0;
    let mut time = 0;
    let mut total_pressure = 0;
    let mut previous = start;
    for valve in valves {
        // Walk the distance
        let distance = distance_map.get(&(previous, valve)).unwrap();
        let minutes = distance + 1;
        if time + minutes > 30 {
            // Valve unreachable
            break;
        }
        time += minutes;
        total_pressure += flow_rate * minutes;
        flow_rate += valve.flow_rate;
        previous = valve;
    }

    let remaining_minutes = 30 - time;
    total_pressure += remaining_minutes * flow_rate;

    println!("Total {}", total_pressure);
    total_pressure
}

fn find_most_pressure(distance_map: &HashMap<(&Valve, &Valve), u32>, valves: &Vec<Valve>) -> u32 {
    let start_valve = valves.iter().find(|valve| valve.name == "AA").unwrap();
    let non_zero_valves: Vec<&Valve> = valves.iter().filter(|valve| valve.flow_rate > 0).collect();
    let len = non_zero_valves.len();
    let mut max_pressure = 0;
    for permutation in non_zero_valves.into_iter().permutations(len) {
        let pressure = calculate_pressure_release(distance_map, start_valve, permutation);
        if pressure > max_pressure {
            max_pressure = pressure
        }
        // println!(
        //     "Nonzero valves: {:?}",
        //     permutation.iter().map(|v| &v.name).collect::<Vec<_>>()
        // );
    }

    max_pressure
}

fn main() {
    println!("Hello, day 16!");
}
