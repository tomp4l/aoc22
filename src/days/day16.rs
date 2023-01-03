use std::{
    collections::{HashMap, HashSet, VecDeque},
    sync::{
        atomic::{AtomicI32, Ordering},
        Arc, Mutex,
    },
    thread,
};

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

fn parse(lines: &[String]) -> Valves {
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

        map.insert(id.to_string(), Valve { flow_rate, tunnels });
    }

    Valves::from_map(map)
}

#[derive(Debug)]
struct Valves {
    dist_start: HashMap<String, i32>,
    dist_between: HashMap<(String, String), i32>,
    flows: HashMap<String, i32>,
}

impl Valves {
    fn from_map(valves: HashMap<String, Valve>) -> Self {
        let with_flow: Vec<_> = valves.iter().filter(|(_, v)| v.flow_rate > 0).collect();
        let mut dist_start = HashMap::new();

        for (v, _) in &with_flow {
            let d = distance_between(&valves, "AA", v);
            dist_start.insert((*v).to_owned(), d);
        }

        let mut dist_between = HashMap::new();

        for (a, _) in &with_flow {
            for (b, _) in &with_flow {
                if a != b && !dist_between.contains_key(&((*a).to_owned(), (*b).to_owned())) {
                    let d = distance_between(&valves, a, b);

                    dist_between.insert(((*a).to_owned(), (*b).to_owned()), d);
                    dist_between.insert(((*b).to_owned(), (*a).to_owned()), d);
                }
            }
        }

        let mut flows = HashMap::new();

        for (v, f) in &with_flow {
            flows.insert((*v).to_owned(), f.flow_rate);
        }

        Valves {
            dist_start,
            dist_between,
            flows,
        }
    }
}

fn distance_between(valves: &HashMap<String, Valve>, from: &str, to: &str) -> i32 {
    let mut visited = HashSet::new();
    visited.insert(from);
    let mut postions = VecDeque::new();
    postions.push_front((from, 0));

    while let Some((v, d)) = postions.pop_back() {
        let valve = &valves[v];
        for tunnel in &valve.tunnels {
            if tunnel == to {
                return d + 1;
            } else {
                postions.push_front((tunnel, d + 1));
                visited.insert(tunnel);
            }
        }
    }

    panic!("missing node")
}

fn part1(valves: &Valves) {
    println!("Part 1: {}", parts(valves, 30, false))
}

#[derive(Debug)]
struct StateElephant<'a> {
    opened: HashSet<&'a String>,
    current_pos: &'a String,
    elephant_pos: &'a String,
    pressure: i32,
    current_time: i32,
    elephant_time: i32,
}

fn part2(valves: &Valves) {
    println!("Part 2: {}", parts(valves, 26, true))
}

fn parts(valves: &Valves, moves: i32, has_elephant: bool) -> i32 {
    let max_pressure = Arc::new(AtomicI32::new(0));

    let mut states = VecDeque::new();
    let all_valves: HashSet<_> = valves.flows.keys().collect();

    for (v, d) in &valves.dist_start {
        for (v_e, d_e) in &valves.dist_start {
            if has_elephant && v != v_e || !has_elephant && v == v_e {
                let flow = valves.flows[v];
                let time = moves - d - 1;
                let pressure = time * flow;

                let flow_e = valves.flows[v_e];
                let time_e = moves - d_e - 1;
                let pressure_e = if has_elephant { time_e * flow_e } else { 0 };

                let state = StateElephant {
                    opened: HashSet::from([v, v_e]),
                    current_pos: v,
                    elephant_pos: v_e,
                    pressure: pressure + pressure_e,
                    current_time: time,
                    elephant_time: time_e,
                };
                states.push_back(state);
            }
        }
    }

    let num_threads = std::thread::available_parallelism()
        .map(|u| u.get())
        .unwrap_or(1);

    let batch_size = 100;

    while let Some(next) = states.pop_front() {
        if states.len() > num_threads * 100 {
            break;
        }

        if next.pressure > max_pressure.load(Ordering::Relaxed) {
            max_pressure.store(next.pressure, Ordering::Relaxed);
        }

        let (time, current, is_elephant) =
            if !has_elephant || next.current_time > next.elephant_time {
                (next.current_time, next.current_pos, false)
            } else {
                (next.elephant_time, next.elephant_pos, true)
            };

        for p in all_valves.difference(&next.opened) {
            let distance = valves.dist_between[&(current.to_owned(), (*p).to_owned())];

            let next_time = time - distance - 1;

            if next_time <= 0 {
                continue;
            }

            let next_pressure = next.pressure + next_time * valves.flows[*p];

            let mut opened = next.opened.clone();
            opened.insert(p);

            let state = if is_elephant {
                StateElephant {
                    opened,
                    current_pos: next.current_pos,
                    elephant_pos: p,
                    pressure: next_pressure,
                    current_time: next.current_time,
                    elephant_time: next_time,
                }
            } else {
                StateElephant {
                    opened,
                    current_pos: p,
                    elephant_pos: next.elephant_pos,
                    pressure: next_pressure,
                    current_time: next_time,
                    elephant_time: next.elephant_time,
                }
            };
            states.push_back(state);
        }
    }

    thread::scope(|s| {
        let mutex = Arc::new(Mutex::new(states));

        for _ in 0..num_threads {
            let data = mutex.clone();
            let max_pressure = max_pressure.clone();
            let all_valves = all_valves.clone();
            s.spawn(move || loop {
                let mut states = Vec::new();
                {
                    let mut all_states = data.lock().unwrap();

                    if all_states.is_empty() {
                        break;
                    }
                    for _ in 0..batch_size {
                        if let Some(x) = all_states.pop_front() {
                            states.push(x);
                        }
                    }
                }

                let mut new_states = Vec::new();

                while let Some(next) = states.pop() {
                    if next.pressure > max_pressure.load(Ordering::Relaxed) {
                        max_pressure.store(next.pressure, Ordering::Relaxed);
                    }
                    let (time, current, is_elephant) =
                        if !has_elephant || next.current_time > next.elephant_time {
                            (next.current_time, next.current_pos, false)
                        } else {
                            (next.elephant_time, next.elephant_pos, true)
                        };

                    for p in all_valves.difference(&next.opened) {
                        let distance = valves.dist_between[&(current.to_owned(), (*p).to_owned())];

                        let next_time = time - distance - 1;

                        if next_time <= 0 {
                            continue;
                        }

                        let next_pressure = next.pressure + next_time * valves.flows[*p];

                        let mut opened = next.opened.clone();
                        opened.insert(p);

                        let state = if is_elephant {
                            StateElephant {
                                opened,
                                current_pos: next.current_pos,
                                elephant_pos: p,
                                pressure: next_pressure,
                                current_time: next.current_time,
                                elephant_time: next_time,
                            }
                        } else {
                            StateElephant {
                                opened,
                                current_pos: p,
                                elephant_pos: next.elephant_pos,
                                pressure: next_pressure,
                                current_time: next_time,
                                elephant_time: next.elephant_time,
                            }
                        };
                        new_states.push(state);
                    }
                }

                data.lock().unwrap().extend(new_states);
            });
        }
    });

    max_pressure.load(Ordering::Relaxed)
}
