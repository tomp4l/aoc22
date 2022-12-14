use std::collections::HashMap;

use super::point::Point2d;

pub fn run(lines: Vec<String>) -> Result<(), String> {
    let mut caves = Caves::from_strs(&lines);

    let mut sand = 0;
    while caves.drop_sand() {
        sand += 1;
    }

    println!("Part 1 {}", sand);

    caves.with_floor();
    while caves.drop_sand() {
        sand += 1;
    }

    println!("Part 2 {}", sand);

    Ok(())
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
enum Occupied {
    Rock,
    Sand,
}

struct Caves(HashMap<Point2d, Occupied>, i32, bool);

impl Caves {
    fn from_strs(strs: &[String]) -> Self {
        let mut map = HashMap::new();

        for str in strs {
            let coords: Vec<_> = str
                .split(" -> ")
                .map(|s| {
                    let split: Vec<_> = s.split(',').collect();
                    let x = split[0].parse::<i32>().unwrap();
                    let y = split[1].parse::<i32>().unwrap();

                    Point2d::new(x, y)
                })
                .collect();

            coords.windows(2).for_each(|c| {
                let p1 = &c[0];
                let p2 = &c[1];

                let do_move = |p: &Point2d| {
                    if p1.x() < p2.x() {
                        p.right()
                    } else if p1.x() > p2.x() {
                        p.left()
                    } else if p1.y() < p2.y() {
                        p.down()
                    } else if p1.y() > p2.y() {
                        p.up()
                    } else {
                        panic!("unreachable")
                    }
                };

                let mut p = p1.clone();
                loop {
                    map.insert(p.clone(), Occupied::Rock);
                    if p == *p2 {
                        break;
                    }
                    p = do_move(&p)
                }
            });
        }

        let min_y = map.keys().max_by_key(|p| p.y()).unwrap().y();
        Caves(map, min_y, false)
    }

    fn bottom(&self) -> i32 {
        self.1
    }

    fn with_floor(&mut self) {
        self.2 = true
    }

    fn drop_sand(&mut self) -> bool {
        let start = Point2d::new(500, 0);

        if self.0.contains_key(&start) {
            return false;
        }

        let mut position = start.clone();
        let has_floor = self.2;
        let min_y = self.bottom();

        let mut cont = true;
        while position.y() <= min_y && cont {
            let below = position.down();
            let below_left = below.left();
            let below_right = below.right();

            let attempts = vec![below, below_left, below_right];

            cont = false;
            for attempt in attempts {
                if !self.0.contains_key(&attempt) {
                    position = attempt.clone();
                    cont = true;
                    break;
                }
            }
        }

        let settled = !cont || has_floor;

        if settled {
            self.0.insert(position, Occupied::Sand);
        }

        settled
    }
}
