use std::{cmp::Ordering, iter::Peekable, str::Chars};

pub fn run(lines: Vec<String>) -> Result<(), String> {
    let parsed_lines: Vec<_> = lines
        .iter()
        .map(|l| {
            if l.is_empty() {
                None
            } else {
                Some(List::from_str(l))
            }
        })
        .collect();

    let parsed_pairs: Vec<_> = parsed_lines
        .split(|l| l.is_none())
        .map(|i| Pair(i[0].clone().unwrap(), i[1].clone().unwrap()))
        .collect();

    let part1 = parsed_pairs
        .iter()
        .enumerate()
        .filter(|(_, p)| p.compare())
        .map(|(i, _)| i + 1)
        .sum::<usize>();

    println!("Part 1 {}", part1);

    let mut packets: Vec<_> = parsed_lines.iter().filter_map(|p| p.as_ref()).collect();
    let m1 = List::from_str("[[2]]");
    let m2 = List::from_str("[[6]]");
    packets.push(&m1);
    packets.push(&m2);
    packets.sort();

    let part2 = packets
        .into_iter()
        .enumerate()
        .filter(|(_, v)| **v == m1 || **v == m2)
        .map(|(i, _)| i + 1)
        .product::<usize>();

    println!("Part 2 {}", part2);

    Ok(())
}

#[derive(Debug, Clone, Eq)]
enum List {
    Nested(Vec<List>),
    Value(u8),
}

impl List {
    fn from_str(str: &str) -> Self {
        List::from_chars(&mut str.chars().peekable())
    }

    fn from_chars(chars: &mut Peekable<Chars>) -> Self {
        let mut nested = Vec::new();
        if let Some('[') = chars.next() {
            'parse_nested: loop {
                if let Some('[') = chars.peek() {
                    let n = List::from_chars(chars);
                    nested.push(n);

                    let a = chars.next().expect("missing closing brace");
                    if a == ']' {
                        break 'parse_nested;
                    }
                } else {
                    let a = chars.next().expect("missing closing brace");
                    if a == ']' {
                        break 'parse_nested;
                    }

                    let mut n = vec![a];
                    'parse_number: loop {
                        let a = chars.next().expect("missing closing brace");
                        if a == ',' {
                            break 'parse_number;
                        } else if a == ']' {
                            let i = n.iter().collect::<String>().parse::<u8>().expect("number");
                            let v = List::Value(i);
                            nested.push(v);
                            break 'parse_nested;
                        } else {
                            n.push(a);
                        }
                    }
                    let i = n.iter().collect::<String>().parse::<u8>().expect("number");
                    let v = List::Value(i);
                    nested.push(v);
                }
            }
            List::Nested(nested)
        } else {
            panic!("missing opening brace");
        }
    }
}

impl PartialEq for List {
    fn eq(&self, right: &Self) -> bool {
        self.cmp(right).is_eq()
    }
}

impl PartialOrd for List {
    fn partial_cmp(&self, right: &Self) -> Option<std::cmp::Ordering> {
        Some(match (self, right) {
            (List::Value(l), List::Value(r)) => l.cmp(r),
            (List::Nested(l), List::Nested(r)) => l.cmp(r),
            (l @ List::Nested(_), r @ List::Value(_)) => l.cmp(&List::Nested(vec![r.clone()])),
            (l @ List::Value(_), r @ List::Nested(_)) => List::Nested(vec![l.clone()]).cmp(r),
        })
    }
}

impl Ord for List {
    fn cmp(&self, right: &Self) -> Ordering {
        self.partial_cmp(right).unwrap()
    }
}

#[derive(Debug)]
struct Pair(List, List);

impl Pair {
    fn compare(&self) -> bool {
        self.0 <= self.1
    }
}
