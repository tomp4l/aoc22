use std::mem;

pub fn run(lines: Vec<String>) -> Result<(), String> {
    part(1, &lines, 20, true)?;
    part(2, &lines, 10000, false)
}

fn part(n: i32, lines: &[String], iterations: i32, reduce_worry: bool) -> Result<(), String> {
    let mut monkeys = lines
        .split(|l| l.is_empty())
        .map(Monkey::from_strs)
        .collect::<Result<Vec<_>, _>>()?;

    let mut all_rounds = vec![0; monkeys.len()];
    for _ in 0..iterations {
        all_rounds = round(&mut monkeys, reduce_worry)
            .inspections
            .into_iter()
            .zip(all_rounds)
            .map(|i| i.0 + i.1)
            .collect();
    }

    all_rounds.sort();
    all_rounds.reverse();

    let monkey_business =
        (*all_rounds.first().unwrap() as i64) * (*all_rounds.get(1).unwrap() as i64);

    println!("Part {} {}", n, monkey_business);
    Ok(())
}

#[derive(Debug)]
enum Operation {
    Add(i32),
    Mult(i32),
    Square,
}

impl Operation {
    fn apply(&self, i: i64) -> i64 {
        match self {
            Operation::Add(j) => i + *j as i64,
            Operation::Mult(j) => i * *j as i64,
            Operation::Square => i * i,
        }
    }
}

#[derive(Debug)]
struct Monkey {
    items: Vec<i32>,
    operation: Operation,
    divisibility: i32,
    if_true: usize,
    if_false: usize,
}

impl Monkey {
    fn from_strs(strs: &[String]) -> Result<Self, String> {
        if strs.len() != 6 {
            return Err(String::from("Wrong input size"));
        }

        let item_start = "  Starting items: ".len();
        let items = strs[1][item_start..]
            .split(", ")
            .map(|s| s.parse::<i32>().map_err(|e| e.to_string()))
            .collect::<Result<Vec<_>, _>>()?;

        let operation_start = "  Operation: new = old ".len();
        let operation_rest = &strs[2][operation_start..];
        let operation = if operation_rest.starts_with('*') {
            if let Ok(i) = operation_rest[2..].parse::<i32>() {
                Operation::Mult(i)
            } else {
                Operation::Square
            }
        } else {
            let i = operation_rest[2..]
                .parse::<i32>()
                .map_err(|e| e.to_string())?;
            Operation::Add(i)
        };

        let test_start = "  Test: divisible by ".len();
        let divisibility = strs[3][test_start..]
            .parse::<i32>()
            .map_err(|e| e.to_string())?;

        let if_true_start = "    If true: throw to monkey ".len();
        let if_true = strs[4][if_true_start..]
            .parse::<usize>()
            .map_err(|e| e.to_string())?;

        let if_false_start = "    If false: throw to monkey ".len();
        let if_false = strs[5][if_false_start..]
            .parse::<usize>()
            .map_err(|e| e.to_string())?;

        Ok(Monkey {
            items,
            operation,
            divisibility,
            if_true,
            if_false,
        })
    }
}

struct RoundStats {
    inspections: Vec<i32>,
}

fn round(monkeys: &mut [Monkey], reduce_worry: bool) -> RoundStats {
    let mut inspections = vec![0; monkeys.len()];
    let worry_reduction = if reduce_worry { 3 } else { 1 };
    let overall_divisibility = monkeys.iter().map(|m| m.divisibility).product::<i32>();

    for i in 0..monkeys.len() {
        let monkey = &mut monkeys[i];
        let items = mem::take(&mut monkey.items);
        inspections[i] = items.len() as i32;

        let mut true_values = Vec::new();
        let mut false_values = Vec::new();
        for item in items {
            let next_worry = monkey.operation.apply(item.into()) / worry_reduction;
            let mod_worry = (next_worry % overall_divisibility as i64) as i32;

            if mod_worry % monkey.divisibility == 0 {
                true_values.push(mod_worry)
            } else {
                false_values.push(mod_worry)
            };
        }

        let if_true = monkey.if_true;
        let if_false = monkey.if_false;
        monkeys[if_true].items.extend(true_values);
        monkeys[if_false].items.extend(false_values);
    }

    RoundStats { inspections }
}
