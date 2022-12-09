use std::collections::HashSet;

use super::point::Point2d;

pub fn run(lines: Vec<String>) -> Result<(), String> {
    let parsed = lines
        .iter()
        .map(|l| Move::from_str(l))
        .collect::<Result<Vec<_>, _>>()?;

    let mut bridge = Bridge::new(1);
    parsed.iter().for_each(|m| {
        bridge.do_move(m);
    });
    let part1 = bridge.last_tail_visited.len();
    println!("Part 1 {}", part1);

    let mut bridge = Bridge::new(9);
    parsed.iter().for_each(|m| {
        bridge.do_move(m);
    });
    let part2 = bridge.last_tail_visited.len();
    println!("Part 2 {}", part2);

    Ok(())
}

#[derive(Debug)]
enum Move {
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32),
}

impl Move {
    fn from_str(str: &str) -> Result<Move, String> {
        if str.len() > 2 {
            let amount = str[2..].parse::<i32>().map_err(|e| e.to_string())?;
            match str.chars().nth(0).unwrap() {
                'U' => Ok(Move::Up(amount)),
                'D' => Ok(Move::Down(amount)),
                'L' => Ok(Move::Left(amount)),
                'R' => Ok(Move::Right(amount)),

                other => Err(format!("Unknown direction {}", other)),
            }
        } else {
            Err("Too short".to_string())
        }
    }

    fn amount(&self) -> i32 {
        match self {
            Move::Up(i) => *i,
            Move::Down(i) => *i,
            Move::Left(i) => *i,
            Move::Right(i) => *i,
        }
    }

    fn move1(&self, point: &Point2d) -> Point2d {
        match self {
            Move::Up(_) => point.up(),
            Move::Down(_) => point.down(),
            Move::Left(_) => point.left(),
            Move::Right(_) => point.right(),
        }
    }
}

#[derive(Debug)]
struct Bridge {
    head: Point2d,
    tails: Vec<Point2d>,
    last_tail_visited: HashSet<Point2d>,
}

impl Bridge {
    fn new(length: i32) -> Self {
        let mut last_tail_visited = HashSet::new();
        last_tail_visited.insert(Point2d::new(0, 0));
        let mut tails = Vec::new();
        for _ in 0..length {
            tails.push(Point2d::new(0, 0))
        }
        Bridge {
            head: Point2d::new(0, 0),
            tails,
            last_tail_visited,
        }
    }

    fn do_move(&mut self, m: &Move) {
        for _ in 0..m.amount() {
            self.head = m.move1(&self.head);
            let mut current = self.head.clone();

            for tail in self.tails.iter_mut() {
                if !current.is_neighbour(&tail) {
                    let dx = current.x() - tail.x();
                    let dy = current.y() - tail.y();
                    let mut next_tail = tail.clone();

                    if dx >= 1 {
                        next_tail = tail.right()
                    } else if dx <= -1 {
                        next_tail = tail.left()
                    }
                    if dy >= 1 {
                        next_tail = next_tail.down()
                    } else if dy <= -1 {
                        next_tail = next_tail.up()
                    }

                    *tail = next_tail;
                }

                current = tail.clone();
            }
            self.last_tail_visited.insert(current);
        }
    }
}
