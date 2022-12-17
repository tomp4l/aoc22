use std::collections::{HashMap, HashSet};

pub fn run(lines: Vec<String>) -> Result<(), String> {
    let valves = parse(&lines);

    part1(&valves);
    part2(&valves);

    Ok(())
}

#[derive(Debug)]
struct Valve {
    flow_rate: i32,
    tunnels: Vec<String>,
}

fn parse(lines: &[String]) -> HashMap<String, Valve> {
    let mut map = HashMap::new();

    for line in lines {
        let split: Vec<_> = line.split(' ').collect();
        let id = split[1];
        let rate_str = split[4];
        let tunnel_strs = &split[9..];

        let flow_rate = rate_str[5..rate_str.len() - 1].parse::<i32>().unwrap();

        let tunnels = tunnel_strs
            .iter()
            .map(|s| s.strip_suffix(',').unwrap_or(s).to_string())
            .collect::<Vec<_>>();

        map.insert(
            id.to_string(),
            Valve {
                flow_rate,
                tunnels: tunnels,
            },
        );
    }

    map
}

#[derive(Debug, Clone)]
struct ValveState {
    opened: HashSet<String>,
    current: String,
    time: i32,
    pressure_released: i32,
}

fn part1(valves: &HashMap<String, Valve>) {
    let start = ValveState {
        opened: HashSet::new(),
        current: "AA".to_string(),
        time: 30,
        pressure_released: 0,
    };

    let mut states = vec![start];
    let mut max_pressure = 0;

    let mut flows: Vec<_> = valves
        .iter()
        .filter_map(|(k, v)| {
            if v.flow_rate > 0 {
                Some((k, v.flow_rate))
            } else {
                None
            }
        })
        .collect();
    flows.sort_by_key(|(_, v)| *v);
    flows.reverse();

    while let Some(state) = states.pop() {
        if state.time >= 0 {
            max_pressure = max_pressure.max(state.pressure_released);
        }

        if state.time <= 0 {
            println!("{}", max_pressure);
            continue;
        }

        let mut max_possible = state.pressure_released;
        let mut time = state.time - 1;
        for (s, v) in &flows {
            if !state.opened.contains(*s) {
                max_possible += v * time;
                time -= 2
            }
        }

        if max_possible <= max_pressure {
            continue;
        }

        let valve = &valves[&state.current];

        if valve.flow_rate > 0 && !state.opened.contains(&state.current) {
            let next_time = state.time - 1;
            let release = valve.flow_rate * next_time;
            let pressure_released = state.pressure_released + release;

            let mut next_opened = state.opened.clone();
            next_opened.insert(state.current.clone());

            let state = ValveState {
                opened: next_opened,

                current: state.current.clone(),
                time: next_time,
                pressure_released: pressure_released,
            };
            states.push(state);
        }

        for t in &valve.tunnels {
            let state = ValveState {
                opened: state.opened.clone(),
                current: t.clone(),
                time: state.time - 1,
                pressure_released: state.pressure_released,
            };
            states.push(state);
        }
    }
    println!("Part 1 {}", max_pressure);
}

fn part2(valves: &HashMap<String, Valve>) {}