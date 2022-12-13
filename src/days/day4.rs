#[derive(Debug)]
struct Pair {
    first: (u8, u8),
    second: (u8, u8),
}

impl Pair {
    fn from_str(str: &str) -> Result<Self, String> {
        let first_split = str.find('-').ok_or("Missing first -")?;
        let comma = str.find(',').ok_or("Missing ,")?;
        let second_split = comma + str[comma..].find('-').ok_or("Missing second -")?;

        let f1 = str[..first_split]
            .parse::<u8>()
            .map_err(|e| format!("{}", e))?;
        let f2 = str[first_split + 1..comma]
            .parse::<u8>()
            .map_err(|e| format!("{}", e))?;
        let s1 = str[comma + 1..second_split]
            .parse::<u8>()
            .map_err(|e| format!("{}", e))?;
        let s2 = str[second_split + 1..]
            .parse::<u8>()
            .map_err(|e| format!("{}", e))?;

        Ok(Pair {
            first: (f1, f2),
            second: (s1, s2),
        })
    }

    fn pair_contains(&self) -> bool {
        let (f1, f2) = self.first;
        let (s1, s2) = self.second;

        (f1 >= s1 && f2 <= s2) || (s1 >= f1 && s2 <= f2)
    }

    fn overlaps(&self) -> bool {
        let (f1, f2) = self.first;
        let (s1, s2) = self.second;

        (f1 >= s1 && f1 <= s2) || (s1 >= f1 && s1 <= f2)
    }
}

pub fn run(lines: Vec<String>) -> Result<(), String> {
    let parsed = lines
        .into_iter()
        .map(|s| Pair::from_str(&s))
        .collect::<Result<Vec<_>, _>>()?;

    let part1 = parsed.iter().filter(|p| p.pair_contains()).count();

    println!("Part 1: {}", part1);

    let part2 = parsed.iter().filter(|p| p.overlaps()).count();

    println!("Part 2: {}", part2);

    Ok(())
}
