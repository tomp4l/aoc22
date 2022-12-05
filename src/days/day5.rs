use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
struct Crate {
    label: char,
}

#[derive(Debug)]
struct Stacks {
    crates: HashMap<u8, Vec<Crate>>,
}

#[derive(Debug)]
struct Instruction {
    amount: u8,
    from: u8,
    to: u8,
}

impl Stacks {
    fn from_strs(strs: &[String]) -> Result<Stacks, String> {
        let mut vec = strs.iter().collect::<Vec<_>>();

        let first = vec.pop().ok_or("missing header")?;
        let columns = first
            .chars()
            .enumerate()
            .filter_map(|(i, c)| c.to_string().parse::<u8>().ok().map(|v| (i, v)))
            .collect::<Vec<_>>();

        let mut crates = HashMap::new();

        columns.iter().for_each(|(_, v)| {
            crates.insert(*v, Vec::<Crate>::new());
        });

        vec.reverse();

        for line in vec {
            let mut chars = line.chars();
            let mut last_index: usize = 0;
            for (index, column) in &columns {
                let next_index = *index - last_index;
                if let Some(label) = chars.nth(next_index) {
                    if label != ' ' {
                        let existing = crates.get_mut(column).expect("set above");
                        existing.push(Crate { label })
                    }
                }
                last_index = *index + 1;
            }
        }

        Ok(Stacks { crates })
    }

    fn from_to(&mut self, inst: &Instruction) -> Option<(&mut Vec<Crate>, &mut Vec<Crate>)> {
        let mut from = None;
        let mut to = None;
        for (k, v) in self.crates.iter_mut() {
            if *k == inst.from {
                from = Some(v)
            } else if *k == inst.to {
                to = Some(v)
            }
        }
        from.and_then(|f| to.map(|t| (f, t)))
    }

    fn apply_instruction(&mut self, inst: &Instruction) {
        if let Some((from, to)) = self.from_to(inst) {
            for _ in 0..inst.amount {
                if let Some(next) = from.pop() {
                    to.push(next)
                }
            }
        }
    }

    fn apply_instruction_9001(&mut self, inst: &Instruction) {
        if let Some((from, to)) = self.from_to(inst) {
            let mut removed = from
                .drain((from.len() - inst.amount as usize)..)
                .collect::<Vec<_>>();
            to.append(&mut removed);
        }
    }

    fn top_stacks(&self) -> String {
        let mut stacks = self.crates.iter().collect::<Vec<_>>();
        stacks.sort_by_key(|(k, _)| *k);
        stacks
            .into_iter()
            .filter_map(|(_, v)| v.last().map(|v| v.label.to_string()))
            .collect::<Vec<_>>()
            .join("")
    }
}

impl Instruction {
    fn from_str(str: &str) -> Result<Instruction, String> {
        lazy_static! {
            static ref RE: Regex = Regex::new("move ([0-9]+) from ([0-9]+) to ([0-9]+)").unwrap();
        }

        RE.captures(str)
            .and_then(|captures| {
                let amount = captures[1].parse::<u8>().ok()?;
                let from = captures[2].parse::<u8>().ok()?;
                let to = captures[3].parse::<u8>().ok()?;

                Some(Instruction { amount, from, to })
            })
            .ok_or(format!("unparseable line {}", str))
    }
}

pub fn run(lines: Vec<String>) -> Result<(), String> {
    let point = lines.partition_point(|l| !(l.is_empty() || l.starts_with("move")));

    let mut stacks = Stacks::from_strs(&lines[..point])?;
    let instructions = &lines[point + 1..]
        .iter()
        .map(|s| Instruction::from_str(s))
        .collect::<Result<Vec<_>, _>>()?;

    instructions
        .iter()
        .for_each(|i| stacks.apply_instruction(i));

    println!("Part 1 {}", stacks.top_stacks());

    let mut stacks = Stacks::from_strs(&lines[..point])?;

    instructions
        .iter()
        .for_each(|i| stacks.apply_instruction_9001(i));

    println!("Part 2 {}", stacks.top_stacks());

    Ok(())
}
