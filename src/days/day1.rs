pub fn run(lines: Vec<String>) -> Result<(), String> {
    let mut sorted: Vec<_> = lines
        .split(String::is_empty)
        .map(|i| i.iter().map(|l| l.parse::<i32>().unwrap()).sum::<i32>())
        .collect();
    sorted.sort();
    sorted.reverse();

    let part1: i32 = sorted.iter().take(1).sum();

    println!("Part 1: {}", part1);

    let part2: i32 = sorted.iter().take(3).sum();
    println!("Part 2: {}", part2);
    Ok(())
}
