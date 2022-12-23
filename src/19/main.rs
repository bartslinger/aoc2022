#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let blueprints = parse_input("./src/19/test.txt");

        assert_eq!(maximize_geodes(&blueprints[0]), 9);
        assert_eq!(maximize_geodes(&blueprints[1]), 12);
        //
        assert_eq!(quality_levels(&blueprints), 33);
    }
}

#[derive(Debug)]
struct Blueprint {
    id: u32,
    ore_robot_cost: u32,             /* ore */
    clay_robot_cost: u32,            /* ore */
    obsidian_robot_cost: (u32, u32), /* (ore, clay) */
    geode_robot_cost: (u32, u32),    /* (ore, obsidian) */
}

fn parse_input(path: &str) -> Vec<Blueprint> {
    let input = std::fs::read_to_string(path).unwrap();
    let mut blueprints = vec![];
    for line in input.lines() {
        let line = line.replace("Blueprint ", "");
        let line = line.replace(": Each ore robot costs ", ",");
        let line = line.replace(" ore. Each clay robot costs ", ",");
        let line = line.replace(" ore. Each obsidian robot costs ", ",");
        let line = line.replace(" ore and ", ",");
        let line = line.replace(" clay. Each geode robot costs ", ",");
        let line = line.replace(" obsidian.", "");
        let mut numbers = line.split(',');
        let blueprint = Blueprint {
            id: numbers.next().unwrap().parse().unwrap(),
            ore_robot_cost: numbers.next().unwrap().parse().unwrap(),
            clay_robot_cost: numbers.next().unwrap().parse().unwrap(),
            obsidian_robot_cost: (
                numbers.next().unwrap().parse().unwrap(),
                numbers.next().unwrap().parse().unwrap(),
            ),
            geode_robot_cost: (
                numbers.next().unwrap().parse().unwrap(),
                numbers.next().unwrap().parse().unwrap(),
            ),
        };
        blueprints.push(blueprint);
    }
    blueprints
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct State {
    time: u32,
    ore_robots: u32,
    clay_robots: u32,
    obsidian_robots: u32,
    geode_robots: u32,
    ore: u32,
    clay: u32,
    obsidian: u32,
    geodes: u32,
}

impl State {
    fn can_build_ore_robot(&self, blueprint: &Blueprint) -> bool {
        self.ore >= blueprint.ore_robot_cost
    }

    fn can_build_clay_robot(&self, blueprint: &Blueprint) -> bool {
        self.ore >= blueprint.clay_robot_cost
    }

    fn can_build_obsidian_robot(&self, blueprint: &Blueprint) -> bool {
        self.ore >= blueprint.obsidian_robot_cost.0 && self.clay >= blueprint.obsidian_robot_cost.1
    }

    fn can_build_geode_robot(&self, blueprint: &Blueprint) -> bool {
        self.ore >= blueprint.geode_robot_cost.0 && self.obsidian >= blueprint.geode_robot_cost.1
    }

    fn step(&mut self) {
        self.time += 1;
        self.ore += self.ore_robots;
        self.clay += self.clay_robots;
        self.obsidian += self.obsidian_robots;
        self.geodes += self.geode_robots;
    }

    fn build_ore_robot(mut self, blueprint: &Blueprint) -> Self {
        self.ore -= blueprint.ore_robot_cost;
        self.step();
        self.ore_robots += 1;
        self
    }

    fn build_clay_robot(mut self, blueprint: &Blueprint) -> Self {
        self.ore -= blueprint.clay_robot_cost;
        self.step();
        self.clay_robots += 1;
        self
    }

    fn build_obsidian_robot(mut self, blueprint: &Blueprint) -> Self {
        self.ore -= blueprint.obsidian_robot_cost.0;
        self.clay -= blueprint.obsidian_robot_cost.1;
        self.step();
        self.obsidian_robots += 1;
        self
    }

    fn build_geode_robot(mut self, blueprint: &Blueprint) -> Self {
        self.ore -= blueprint.geode_robot_cost.0;
        self.obsidian -= blueprint.geode_robot_cost.1;
        self.step();
        self.geode_robots += 1;
        self
    }

    fn get_options(&self, blueprint: &Blueprint) -> Vec<State> {
        let mut options = vec![];

        // do nothing
        let mut step_only = *self;
        step_only.step();
        options.push(step_only);

        let max_useful_ore_robots = blueprint.ore_robot_cost.max(
            blueprint.clay_robot_cost.max(
                blueprint
                    .obsidian_robot_cost
                    .0
                    .max(blueprint.geode_robot_cost.0),
            ),
        );

        if self.time < 22
            && self.ore_robots < max_useful_ore_robots
            && self.can_build_ore_robot(blueprint)
        {
            options.push((*self).build_ore_robot(blueprint));
        }

        if self.time < 21 && self.can_build_clay_robot(blueprint) {
            options.push((*self).build_clay_robot(blueprint));
        }

        if self.time < 22 && self.can_build_obsidian_robot(blueprint) {
            options.push((*self).build_obsidian_robot(blueprint));
        }

        if self.time < 23 && self.can_build_geode_robot(blueprint) {
            options.push((*self).build_geode_robot(blueprint));
        }

        options
    }

    fn is_definitely_worse_than(&self, rhs: &State) -> bool {
        // if self.time >= rhs.time
        //     && self.geode_robots < rhs.geode_robots
        //     && self.geodes <= rhs.geodes
        // {
        //     return true;
        // }
        self.time >= rhs.time
            && self.ore_robots <= rhs.ore_robots
            && self.clay_robots <= rhs.clay_robots
            && self.obsidian_robots <= rhs.obsidian_robots
            && self.geode_robots <= rhs.geode_robots
            && self.ore <= rhs.ore
            && self.clay <= rhs.clay
            && self.obsidian <= rhs.obsidian
            && self.geodes <= rhs.geodes
    }

    fn is_worse_than_any_of(&self, rhs: &[State]) -> bool {
        for rhs_item in rhs.iter() {
            if self == rhs_item {
                continue;
            }
            if self.is_definitely_worse_than(rhs_item) {
                return true;
            }
        }
        false
    }
}

fn maximize_geodes(blueprint: &Blueprint) -> u32 {
    let begin_state = State {
        time: 0,
        ore_robots: 1,
        clay_robots: 0,
        obsidian_robots: 0,
        geode_robots: 0,
        ore: 0,
        clay: 0,
        obsidian: 0,
        geodes: 0,
    };

    let mut states: Vec<State> = begin_state.get_options(blueprint);
    for m in 1..24 {
        let mut new_states: Vec<State> = vec![];
        for state in &states {
            let options = state.get_options(blueprint);
            for option in options {
                if !option.is_worse_than_any_of(&states) {
                    new_states.push(option);
                }
            }
        }
        // also filter all the new states
        let mut new_filtered_states = vec![];
        for new_state in &new_states {
            if !new_state.is_worse_than_any_of(&new_states) {
                new_filtered_states.push(*new_state);
            }
        }
        states = new_filtered_states;
        println!("Minute: {} => States: {}", m + 1, states.len());
    }

    states.iter().map(|s| s.geodes).max().unwrap()
}

fn quality_levels(blueprints: &[Blueprint]) -> u32 {
    let mut quality_level = 0;
    for blueprint in blueprints.iter() {
        let geodes = maximize_geodes(blueprint);
        println!("Blueprint {}: {}", blueprint.id, geodes);
        quality_level += blueprint.id * geodes;
    }
    quality_level
}

fn main() {
    println!("Hello, day 19!");

    let blueprints = parse_input("./input/19/input.txt");
    let part_one = quality_levels(&blueprints);

    println!("Part 1: {}", part_one);
}
