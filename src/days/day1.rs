use std::collections::LinkedList;

pub fn run(lines: Vec<String>) -> Result<(), String> {
    let as_ints = lines
        .iter()
        .map(|l| (l.parse::<i32>().ok()))
        .collect::<Vec<Option<i32>>>();

    parts(as_ints);

    Ok(())
}

struct Collect {
    current: i32,
    collected: LinkedList<i32>,
}

fn parts(lines: Vec<Option<i32>>) {
    let initial = Collect {
        current: 0,
        collected: LinkedList::new(),
    };
    let grouped = lines.iter().fold(initial, |prev, current| match current {
        Option::Some(value) => {
            let mut next = prev;
            next.current += value;
            next
        }
        Option::None => {
            let mut next_collected = prev.collected;

            next_collected.push_front(prev.current);
            Collect {
                current: 0,
                collected: next_collected,
            }
        }
    });

    let mut all_collected = grouped.collected;
    all_collected.push_back(grouped.current);

    let part1 = all_collected.iter().map(|i| *i).max().unwrap_or_default();

    println!("Part 1: {}", part1);

    let mut sorted: Vec<_> = all_collected.iter().map(|i| *i).collect();
    sorted.sort();
    sorted.reverse();
    sorted.truncate(3);

    let part2: i32 = sorted.iter().sum();
    println!("Part 2: {}", part2);
}
