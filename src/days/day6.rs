use std::collections::{hash_map::RandomState, HashSet};

fn markers(str: &str, size: usize) -> usize {
    let position = str
        .chars()
        .collect::<Vec<_>>()
        .as_slice()
        .windows(size)
        .map(|w| {
            let set: HashSet<char, RandomState> = HashSet::from_iter(w.into_iter().copied());
            set.len()
        })
        .take_while(|l| *l != size)
        .count();
    position + size
}

pub fn run(lines: Vec<String>) -> Result<(), String> {
    let line = lines.first().ok_or("missing input")?;

    let part1 = markers(line, 4);
    println!("Part 1 {}", part1);

    let part2 = markers(line, 14);
    println!("Part 2 {}", part2);

    Ok(())
}
