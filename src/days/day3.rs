use std::collections::HashSet;

#[derive(Debug)]
struct Backpack<'a> {
    front: &'a str,
    back: &'a str,
}

impl<'a> Backpack<'a> {
    fn from_str(str: &'a str) -> Result<Self, String> {
        let len = str.len() / 2;
        Ok(Backpack::<'a> {
            front: &str[..len],
            back: &str[len..],
        })
    }

    fn common_score(&self) -> i32 {
        let intersection: Vec<char> = self
            .front_set()
            .intersection(&self.back_set())
            .copied()
            .collect();

        let char = *intersection.first().unwrap();

        score(char)
    }

    fn front_set(&self) -> HashSet<char> {
        self.front.chars().collect()
    }

    fn back_set(&self) -> HashSet<char> {
        self.back.chars().collect()
    }

    fn full_set(&self) -> HashSet<char> {
        let mut s = self.front_set();
        s.extend(self.back_set());
        s
    }
}

fn common_badge((b1, b2, b3): (&Backpack, &Backpack, &Backpack)) -> i32 {
    let i1: HashSet<char> = b1
        .full_set()
        .intersection(&b2.full_set())
        .copied()
        .collect();

    let i2: Vec<char> = i1.intersection(&b3.full_set()).copied().collect();

    let char = *i2.first().unwrap();

    score(char)
}

fn score(char: char) -> i32 {
    const LOWERCASE_A: u8 = 'a' as u8;
    const UPPERCASE_A: u8 = 'A' as u8;

    let char_code = char as u8;

    if char_code >= LOWERCASE_A {
        (char_code - LOWERCASE_A + 1) as i32
    } else {
        (char_code - UPPERCASE_A + 27) as i32
    }
}
pub fn run(lines: Vec<String>) -> Result<(), String> {
    let parsed = lines
        .iter()
        .map(|s| Backpack::from_str(s))
        .collect::<Result<Vec<_>, _>>()?;

    let part1: i32 = parsed.iter().map(|i| i.common_score()).sum();
    println!("Part 1 {}", part1);

    let grouped = parsed
        .as_slice()
        .chunks(3)
        .map(|chunk| (&chunk[0], &chunk[1], &chunk[2]))
        .collect::<Vec<(&Backpack, &Backpack, &Backpack)>>();

    let part2: i32 = grouped.into_iter().map(|t| common_badge(t)).sum();

    println!("Part 2 {}", part2);

    Ok(())
}
