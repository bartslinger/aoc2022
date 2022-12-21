extern crate core;

use petgraph::algo::dijkstra;
use petgraph::graph::{NodeIndex, UnGraph};
use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_highest_flow() {
        let valves = parse_input("./src/16/test.txt");
        let distances = calculate_distances(&valves);

        let start_valve = valves.iter().find(|valve| valve.name == "AA").unwrap();
        let non_zero_valves: Vec<&Valve> =
            valves.iter().filter(|valve| valve.flow_rate > 0).collect();
        let most_pressure = find_most_pressure(
            &distances,
            State {
                previous_valve: start_valve,
                time: 0,
                flow_rate: 0,
                total_pressure: 0,
            },
            non_zero_valves,
        );
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
    let relevant_valves: Vec<&Valve> = valves
        .iter()
        .filter(|valve| valve.name == "AA" || valve.flow_rate > 0)
        .collect();
    for valve in relevant_valves {
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

struct State<'a> {
    previous_valve: &'a Valve,
    time: u32,
    flow_rate: u32,
    total_pressure: u32,
}

fn find_most_pressure(
    distance_map: &HashMap<(&Valve, &Valve), u32>,
    state: State,
    remaining_valves: Vec<&Valve>,
) -> u32 {
    let remaining_time = 30 - state.time;
    let mut max_pressure = state.total_pressure + remaining_time * state.flow_rate;
    for i in 0..remaining_valves.len() {
        let mut new_remaining_valves = remaining_valves.clone();
        let new_valve = new_remaining_valves.remove(i);

        let minutes = *distance_map
            .get(&(state.previous_valve, new_valve))
            .unwrap()
            + 1;
        let new_time = state.time + minutes;
        if new_time >= 30 {
            // This valve adds nothing
            continue;
        }
        let new_total_pressure = state.total_pressure + state.flow_rate * minutes;
        let new_flow_rate = state.flow_rate + new_valve.flow_rate;
        // Open more valves
        max_pressure = max_pressure.max(find_most_pressure(
            distance_map,
            State {
                previous_valve: new_valve,
                time: new_time,
                flow_rate: new_flow_rate,
                total_pressure: new_total_pressure,
            },
            new_remaining_valves,
        ));
    }

    max_pressure
}

fn main() {
    println!("Hello, day 16!");

    let valves = parse_input("./input/16/input.txt");
    let distances = calculate_distances(&valves);

    let start_valve = valves.iter().find(|valve| valve.name == "AA").unwrap();
    let non_zero_valves: Vec<&Valve> = valves.iter().filter(|valve| valve.flow_rate > 0).collect();
    let most_pressure = find_most_pressure(
        &distances,
        State {
            previous_valve: start_valve,
            time: 0,
            flow_rate: 0,
            total_pressure: 0,
        },
        non_zero_valves,
    );
    println!("Part 1: {}", most_pressure);
}
