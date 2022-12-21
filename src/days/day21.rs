use std::collections::HashMap;

pub fn run(lines: Vec<String>) -> Result<(), String> {
    let monkeys = lines
        .iter()
        .map(|l| parse(l))
        .collect::<Result<HashMap<_, _>, _>>()?;

    let results = get_results(monkeys.iter().collect());

    println!("Part 1: {}", results.get("root").unwrap());

    let human = "humn".to_string();
    let start = 0.0;
    let mut human_val = start;

    let mut values = Vec::new();
    let mut delta = 0.0;
    loop {
        let mut monkeys: HashMap<_, _> = monkeys.iter().collect();
        let root = monkeys.remove(&"root".to_owned());
        if let Some(Monkey::Op(m1, _, m2)) = root {
            let value = Monkey::Val(human_val);
            monkeys.insert(&human, &value);
            let results = get_results(monkeys);
            delta = results.get(m2.as_str()).unwrap() - results.get(m1.as_str()).unwrap();

            values.push(*results.get(m1.as_str()).unwrap());
        }
        human_val += 1.0;
        if human_val > start + 100.0 {
            break;
        }
    }

    let gradients: Vec<_> = values.windows(2).map(|w| w[1] - w[0]).collect();
    let avg = gradients.iter().sum::<f64>() / gradients.len() as f64;

    loop {
        if delta == 0.0 {
            break;
        } else {
            let difference = delta / avg;
            human_val += difference;
        }

        let mut monkeys: HashMap<_, _> = monkeys.iter().collect();
        let root = monkeys.remove(&"root".to_owned());
        if let Some(Monkey::Op(m1, _, m2)) = root {
            let value = Monkey::Val(human_val);
            monkeys.insert(&human, &value);
            let results = get_results(monkeys);
            delta = results.get(m2.as_str()).unwrap() - results.get(m1.as_str()).unwrap();

            values.push(*results.get(m1.as_str()).unwrap());
        }
    }

    println!("Part 2: {}", human_val);

    Ok(())
}

fn get_results<'a>(monkeys: HashMap<&'a String, &Monkey>) -> HashMap<&'a str, f64> {
    let mut remaining = monkeys;
    let mut results: HashMap<&str, f64> = HashMap::new();

    loop {
        if remaining.is_empty() {
            break;
        }

        let keys: Vec<_> = remaining.keys().copied().collect();
        for m in keys {
            let monkey = remaining[m];

            match monkey {
                Monkey::Op(m1, op, m2) => {
                    if let Some(v1) = results.get(m1.as_str()) {
                        if let Some(v2) = results.get(m2.as_str()) {
                            let v = match op {
                                Op::Add => v1 + v2,
                                Op::Sub => v1 - v2,
                                Op::Mul => v1 * v2,
                                Op::Div => v1 / v2,
                            };
                            results.insert(m, v);
                            remaining.remove(m);
                        }
                    };
                }

                Monkey::Val(v) => {
                    results.insert(m, *v);
                    remaining.remove(m);
                }
            };
        }
    }
    results
}

enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

enum Monkey {
    Op(String, Op, String),
    Val(f64),
}

fn parse(str: &str) -> Result<(String, Monkey), String> {
    let mut split = str.split(": ");

    let name = split.next().ok_or("no name".to_owned())?.to_owned();

    let op = split.next().ok_or("no op".to_owned())?;

    let parsed_monkey = op.parse::<f64>().map(Monkey::Val).or_else(|_| {
        let split = op.split(' ').collect::<Vec<_>>();
        if split.len() != 3 {
            Err("Wrong input length".to_owned())
        } else {
            let m1 = split[0].to_owned();
            let m2 = split[2].to_owned();

            let op = match split[1] {
                "+" => Ok(Op::Add),
                "-" => Ok(Op::Sub),
                "*" => Ok(Op::Mul),
                "/" => Ok(Op::Div),
                _ => Err("Unrecognised op".to_owned()),
            };

            op.map(|o| Monkey::Op(m1, o, m2))
        }
    });

    parsed_monkey.map(|o| (name, o))
}
