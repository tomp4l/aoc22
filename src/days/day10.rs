use std::collections::HashSet;

pub fn run(lines: Vec<String>) -> Result<(), String> {
    let parsed = lines
        .iter()
        .map(|l| Instruction::from_str(l))
        .collect::<Result<Vec<_>, _>>()?;

    part1(&parsed);
    part2(&parsed);

    Ok(())
}

#[derive(Debug)]
enum Instruction {
    Noop,
    Addx(i32),
}

impl Instruction {
    fn from_str(str: &str) -> Result<Self, String> {
        if str.starts_with("addx ") {
            let x = str[5..].parse::<i32>().map_err(|e| e.to_string())?;
            Ok(Instruction::Addx(x))
        } else {
            Ok(Instruction::Noop)
        }
    }
}

fn part1(parsed: &Vec<Instruction>) {
    let magic_cycles = HashSet::<i32>::from_iter(vec![20, 60, 100, 140, 180, 220]);
    let mut strength = 0;

    computer(parsed, |cycle, x| {
        if magic_cycles.contains(&cycle) {
            strength += cycle * x;
        }
    });

    println!("Part 1 {}", strength)
}

fn part2(parsed: &Vec<Instruction>) {
    let mut pixels = vec![false; 240];
    computer(parsed, |cycle, x| {
        let pos = (cycle - 1) % 40;
        if pos >= (x - 1) && pos <= (x + 1) {
            if let Some(p) = pixels.get_mut((cycle - 1) as usize) {
                *p = true;
            }
        }
    });

    println!();
    println!("Part 2");
    pixels.chunks(40).for_each(|l| {
        let line = l
            .iter()
            .map(|b| if *b { "#" } else { "." })
            .collect::<String>();
        println!("{}", line)
    });
}

fn computer<F>(parsed: &Vec<Instruction>, mut fun: F)
where
    F: FnMut(i32, i32) -> (),
{
    let mut cycle = 1;
    let mut pointer = 0;
    let mut x = 1;
    loop {
        fun(cycle, x);
        match parsed.get(pointer).expect("pointer should stay in range") {
            Instruction::Noop => (),
            Instruction::Addx(add) => {
                cycle += 1;
                fun(cycle, x);
                x += add
            }
        }
        cycle += 1;
        pointer += 1;
        if pointer >= parsed.len() {
            break;
        }
    }
}
