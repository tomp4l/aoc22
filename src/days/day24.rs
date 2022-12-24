use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use super::point::Point2d;

pub fn run(lines: Vec<String>) -> Result<(), String> {
    let mut valley = Valley::from_strs(&lines);

    parts(&mut valley);

    Ok(())
}

#[derive(Debug)]
struct Valley {
    blizzards: HashMap<Point2d, Blizzard>,
    max_x: i32,
    max_y: i32,
}

impl Valley {
    fn from_strs(strs: &[String]) -> Self {
        let mut lines = strs.iter();
        let mut blizzards = HashMap::new();
        lines.next();

        let mut y = 1;
        for line in lines {
            let mut x = 1;
            if &line[1..2] == "#" {
                let max_x = line.len() as i32 - 2;
                let max_y = y - 1;
                return Valley {
                    blizzards,
                    max_x,
                    max_y,
                };
            } else {
                let mut chars = line.chars();
                chars.next();
                for char in chars {
                    let point = Point2d::new(x, y);
                    let directions = match char {
                        '>' => vec![Direction::Right],
                        '<' => vec![Direction::Left],
                        'v' => vec![Direction::Down],
                        '^' => vec![Direction::Up],
                        '.' | '#' => Vec::new(),
                        _ => unimplemented!(),
                    };

                    if !directions.is_empty() {
                        blizzards.insert(point, Blizzard(directions));
                    }
                    x += 1;
                }
            }
            y += 1;
        }

        unreachable!()
    }

    #[allow(dead_code)]
    fn print(&self, positions: &HashSet<Point2d>) {
        for y in 1..self.max_y + 1 {
            for x in 1..self.max_x + 1 {
                if let Some(Blizzard(dir)) = self.blizzards.get(&Point2d::new(x, y)) {
                    if dir.len() == 1 {
                        let c = match dir[0] {
                            Direction::Up => "^",
                            Direction::Left => "<",
                            Direction::Down => "v",
                            Direction::Right => ">",
                        };
                        print!("{}", c);
                    } else {
                        print!("{}", dir.len());
                    }
                } else if positions.contains(&Point2d::new(x, y)) {
                    print!("o")
                } else {
                    print!(" ")
                }
            }
            println!()
        }
    }

    fn step(&mut self) {
        let mut next_points: Vec<_> = std::mem::take(&mut self.blizzards)
            .iter()
            .flat_map(|(p, b)| {
                b.0.iter().map(|d| match d {
                    Direction::Up => {
                        let next = p.up();
                        let wrapped = if next.y() == 0 {
                            next.add_y(self.max_y)
                        } else {
                            next
                        };
                        (wrapped, Direction::Up)
                    }
                    Direction::Down => {
                        let next = p.down();
                        let wrapped = if next.y() == self.max_y + 1 {
                            next.add_y(-self.max_y)
                        } else {
                            next
                        };
                        (wrapped, Direction::Down)
                    }
                    Direction::Left => {
                        let next = p.left();
                        let wrapped = if next.x() == 0 {
                            next.add_x(self.max_x)
                        } else {
                            next
                        };
                        (wrapped, Direction::Left)
                    }
                    Direction::Right => {
                        let next = p.right();
                        let wrapped = if next.x() == self.max_x + 1 {
                            next.add_x(-self.max_x)
                        } else {
                            next
                        };
                        (wrapped, Direction::Right)
                    }
                })
            })
            .collect();

        next_points.sort_by_key(|p| p.0.clone());

        next_points
            .into_iter()
            .group_by(|a| a.0.clone())
            .into_iter()
            .for_each(|(p, g)| {
                self.blizzards.insert(p, Blizzard(g.map(|t| t.1).collect()));
            });
    }
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Blizzard(Vec<Direction>);

fn parts(valley: &mut Valley) {
    let mut minute = 0;
    let start = Point2d::new(1, 0);
    let end = Point2d::new(valley.max_x, valley.max_y + 1);

    let mut positions = HashSet::from([start.clone()]);
    let mut goal = end.clone();
    let mut part2 = false;
    loop {
        let mut completed = false;
        valley.step();
        for position in std::mem::take(&mut positions) {
            if position == goal {
                completed = true;
                break;
            }

            let up = position.up();
            let down = position.down();
            let left = position.left();
            let right = position.right();

            for p in [up, down, left, right, position] {
                if (p != start && p != end)
                    && (p.x() <= 0 || p.y() <= 0 || p.x() > valley.max_x || p.y() > valley.max_y)
                {
                    continue;
                }

                if !valley.blizzards.contains_key(&p) {
                    positions.insert(p);
                }
            }
        }

        if completed {
            if !part2 {
                println!("Part 1 {}", minute);
                part2 = true;
                goal = start.clone();
                positions = HashSet::from([Point2d::new(valley.max_x, valley.max_y + 1)]);
            } else if goal == start {
                goal = end.clone();
                positions = HashSet::from([start.clone()]);
            } else {
                println!("Part 2 {}", minute);
                break;
            }
        }
        minute += 1;
    }
}
