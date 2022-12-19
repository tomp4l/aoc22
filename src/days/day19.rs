use std::collections::{hash_map::Entry, HashMap, HashSet};

pub fn run(lines: Vec<String>) -> Result<(), String> {
    let blueprints: Vec<_> = lines.iter().map(|l| Blueprint::from_str(l)).collect();

    let start_robots = HashMap::from([(Resource::Ore, 1)]);
    let time = 24;
    let mut total_quality: u32 = 0;
    for b in &blueprints {
        let best = b.simulate(time, &start_robots);
        total_quality += best as u32 * b.id as u32;
    }
    println!("Part 1: {}", total_quality);

    let time = 32;
    let mut quality_product = 1;
    for b in blueprints.iter().take(3) {
        let best = b.simulate(time, &start_robots);
        quality_product *= best as u32;
    }

    println!("Part 2: {}", quality_product);

    Ok(())
}

#[derive(Hash, PartialEq, PartialOrd, Ord, Eq, Debug, Clone)]
enum Resource {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Debug)]
struct Cost(HashMap<Resource, u8>);

impl Cost {
    fn single(resource: Resource, amount: u8) -> Self {
        Cost(HashMap::from([(resource, amount)]))
    }

    fn double(resource1: Resource, amount1: u8, resource2: Resource, amount2: u8) -> Self {
        Cost(HashMap::from([(resource1, amount1), (resource2, amount2)]))
    }
}

#[derive(Debug)]
struct Blueprint {
    id: i32,
    ore_cost: Cost,
    clay_cost: Cost,
    obsidian_cost: Cost,
    geode_cost: Cost,
}

impl Blueprint {
    fn from_str(str: &str) -> Self {
        let mut first_split = str.split(": ");
        let id = first_split
            .next()
            .unwrap()
            .split(' ')
            .last()
            .unwrap()
            .parse::<i32>()
            .unwrap();

        let mut second_split = first_split.next().unwrap().split(". ");

        let mut ore_cost_str = second_split.next().unwrap().split(' ');
        let mut clay_cost_str = second_split.next().unwrap().split(' ');
        let mut obsidian_cost_str = second_split.next().unwrap().split(' ');
        let mut geode_cost_str = second_split.next().unwrap().split(' ');

        let ore_ore_cost = ore_cost_str.nth(4).unwrap().parse::<u8>().unwrap();
        let clay_ore_cost = clay_cost_str.nth(4).unwrap().parse::<u8>().unwrap();
        let obsidian_ore_cost = obsidian_cost_str.nth(4).unwrap().parse::<u8>().unwrap();
        let obsidian_clay_cost = obsidian_cost_str.nth(2).unwrap().parse::<u8>().unwrap();
        let geode_ore_cost = geode_cost_str.nth(4).unwrap().parse::<u8>().unwrap();
        let geode_obsidian_cost = geode_cost_str.nth(2).unwrap().parse::<u8>().unwrap();

        Blueprint {
            id,
            ore_cost: Cost::single(Resource::Ore, ore_ore_cost),
            clay_cost: Cost::single(Resource::Ore, clay_ore_cost),
            obsidian_cost: Cost::double(
                Resource::Ore,
                obsidian_ore_cost,
                Resource::Clay,
                obsidian_clay_cost,
            ),
            geode_cost: Cost::double(
                Resource::Ore,
                geode_ore_cost,
                Resource::Obsidian,
                geode_obsidian_cost,
            ),
        }
    }

    fn costs(&self) -> HashMap<Resource, &Cost> {
        HashMap::from([
            (Resource::Ore, &self.ore_cost),
            (Resource::Clay, &self.clay_cost),
            (Resource::Obsidian, &self.obsidian_cost),
            (Resource::Geode, &self.geode_cost),
        ])
    }

    fn simulate(&self, time: i32, start_robots: &HashMap<Resource, i32>) -> u8 {
        let start_state = State::new(start_robots);
        let mut states = HashMap::from([(start_state.robots, HashSet::from([start_state]))]);

        for i in (0..time).rev() {
            for state in std::mem::take(&mut states).values().flatten() {
                let mut next_state = state.clone();
                next_state.process_resources();
                let mut next_states = vec![next_state];

                for (resource, cost) in self.costs() {
                    let mut next_state = state.clone();

                    let mut has_resources = true;

                    for (res, res_cost) in &cost.0 {
                        has_resources = next_state.take_resource(res, *res_cost);
                        if !has_resources {
                            break;
                        }
                    }

                    if has_resources {
                        next_state.process_resources();
                        next_state.add_robot(resource);
                        next_states.push(next_state);
                    }
                }

                for next_state in next_states {
                    match states.entry(next_state.robots) {
                        Entry::Occupied(mut s) => {
                            s.get_mut().insert(next_state);
                        }
                        Entry::Vacant(v) => {
                            v.insert(HashSet::from([next_state]));
                        }
                    }
                }
            }

            let max_geode_robots = states.keys().map(|v| v[3]).max().unwrap_or_default();

            states.retain(|r, _| r[3] + 1 >= max_geode_robots);

            for (_, v) in states.iter_mut() {
                let max_geodes = v.iter().map(|s| s.resources[3]).max().unwrap_or_default();
                v.retain(|s| s.resources[3] == max_geodes);
            }
        }

        states
            .values()
            .flatten()
            .map(|s| s.resources[3])
            .max()
            .unwrap_or_default()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct State {
    robots: [u8; 4],
    resources: [u8; 4],
}

impl State {
    fn new(robots_map: &HashMap<Resource, i32>) -> Self {
        let ores = robots_map.get(&Resource::Ore).copied().unwrap_or_default();
        let clay = robots_map.get(&Resource::Clay).copied().unwrap_or_default();
        let obsidian = robots_map
            .get(&Resource::Obsidian)
            .copied()
            .unwrap_or_default();
        let geodes = robots_map
            .get(&Resource::Geode)
            .copied()
            .unwrap_or_default();

        let robots = [ores as u8, clay as u8, obsidian as u8, geodes as u8];

        State {
            robots,
            resources: [0; 4],
        }
    }

    fn index(resoure: &Resource) -> usize {
        match resoure {
            Resource::Ore => 0,
            Resource::Clay => 1,
            Resource::Obsidian => 2,
            Resource::Geode => 3,
        }
    }

    fn take_resource(&mut self, resoure: &Resource, amount: u8) -> bool {
        let i = State::index(resoure);
        let count = self.resources[i];
        if count >= amount {
            self.resources[i] -= amount;
            true
        } else {
            false
        }
    }

    fn add_robot(&mut self, resoure: Resource) {
        let i = State::index(&resoure);
        self.robots[i] += 1;
    }

    fn process_resources(&mut self) {
        for (i, a) in self.robots.iter().enumerate() {
            self.resources[i] += a;
        }
    }
}
