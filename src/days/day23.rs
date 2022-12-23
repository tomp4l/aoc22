use std::collections::{HashMap, HashSet, VecDeque};

use super::point::Point2d;

pub fn run(lines: Vec<String>) -> Result<(), String> {
    let mut map = Map::from_strs(&lines);
    let mut rules = Rules::new();

    for i in 1..10000 {
        let moved = map.turn(&mut rules);

        if i == 10 {
            println!("Part 1 {}", map.empty_spaces_count());
        }

        if !moved {
            println!("Part 2 {}", i);
            break;
        }
    }

    Ok(())
}

#[derive(Debug)]
struct Map(HashSet<Point2d>);

impl Map {
    fn from_strs(strs: &[String]) -> Self {
        let mut y = 1;
        let mut map = HashSet::new();
        for line in strs {
            let mut x = 1;
            for char in line.chars() {
                if char == '#' {
                    let point = Point2d::new(x, y);
                    map.insert(point);
                }

                x += 1;
            }
            y += 1;
        }
        Map(map)
    }

    #[allow(dead_code)]
    fn print(&self) {
        let map = &self.0;
        let min_x = map.iter().min_by_key(|p| p.x()).unwrap().x();
        let min_y = map.iter().min_by_key(|p| p.y()).unwrap().y();

        let max_x = map.iter().max_by_key(|p| p.x()).unwrap().x();
        let max_y = map.iter().max_by_key(|p| p.y()).unwrap().y();

        for y in min_y..max_y + 1 {
            for x in min_x..max_x + 1 {
                if map.contains(&Point2d::new(x, y)) {
                    print!("#");
                } else {
                    print!(" ");
                }
            }
            println!()
        }
    }

    fn turn(&mut self, rules: &mut Rules) -> bool {
        let mut moves = Vec::new();
        for elf in &self.0 {
            if self.is_alone(elf) {
                continue;
            }

            for rule in &rules.0 {
                if rule.should_move(self, elf) {
                    moves.push((elf.clone(), rule.new_position(elf)));
                    break;
                }
            }
        }

        let mut all_moves = HashMap::new();
        for (_, m) in &moves {
            all_moves
                .entry(m.clone())
                .and_modify(|v| *v += 1)
                .or_insert(1);
        }

        moves.retain(|v| all_moves.get(&v.1).copied().unwrap_or_default() == 1);

        for (from, to) in &moves {
            self.move_elf(from, to);
        }

        rules.rotate();

        !moves.is_empty()
    }

    fn is_alone(&self, elf: &Point2d) -> bool {
        let n = elf.up();
        let s = elf.down();
        [
            n.left(),
            n.right(),
            n,
            elf.right(),
            s.right(),
            s.left(),
            s,
            elf.left(),
        ]
        .iter()
        .all(|p| !self.0.contains(p))
    }

    // fn elves(&self) -> Vec<&Point2d> {
    //     self.0.iter().collect()
    // }

    fn move_elf(&mut self, from: &Point2d, to: &Point2d) {
        self.0.remove(from);
        self.0.insert(to.clone());
    }

    fn empty_spaces_count(&self) -> i32 {
        let elves = &self.0;

        let min_x = elves.iter().min_by_key(|p| p.x()).unwrap().x();
        let min_y = elves.iter().min_by_key(|p| p.y()).unwrap().y();
        let max_x = elves.iter().max_by_key(|p| p.x()).unwrap().x();
        let max_y = elves.iter().max_by_key(|p| p.y()).unwrap().y();

        let mut count = 0;
        for x in min_x..max_x + 1 {
            for y in min_y..max_y + 1 {
                if !self.0.contains(&Point2d::new(x, y)) {
                    count += 1;
                }
            }
        }
        count
    }
}

#[derive(Debug)]
enum Rule {
    North,
    South,
    West,
    East,
}

impl Rule {
    fn should_move(&self, map: &Map, elf: &Point2d) -> bool {
        let (a, b, c) = match self {
            Rule::North => (elf.up().left(), elf.up(), elf.up().right()),
            Rule::South => (elf.down().left(), elf.down(), elf.down().right()),
            Rule::West => (elf.left().up(), elf.left(), elf.left().down()),
            Rule::East => (elf.right().up(), elf.right(), elf.right().down()),
        };

        !map.0.contains(&a) && !map.0.contains(&b) && !map.0.contains(&c)
    }

    fn new_position(&self, elf: &Point2d) -> Point2d {
        match self {
            Rule::North => elf.up(),
            Rule::South => elf.down(),
            Rule::West => elf.left(),
            Rule::East => elf.right(),
        }
    }
}

#[derive(Debug)]
struct Rules(VecDeque<Rule>);

impl Rules {
    fn new() -> Self {
        Rules(VecDeque::from([
            Rule::North,
            Rule::South,
            Rule::West,
            Rule::East,
        ]))
    }

    fn rotate(&mut self) {
        let first = self.0.pop_front().unwrap();
        self.0.push_back(first);
    }
}
