use std::collections::{HashMap, HashSet, VecDeque};

use super::point::Point2d;

pub fn run(lines: Vec<String>) -> Result<(), String> {
    let grid = Grid::from_strs(&lines);

    println!(
        "Part 1 {}",
        distance(
            &grid.start(),
            &grid,
            |e| e.is_highest(),
            |c, n| n.can_traverse(c)
        )
        .unwrap_or_default()
    );

    println!(
        "Part 2 {}",
        distance(
            &grid.end(),
            &grid,
            |e| e.height() == 1,
            |c, n| c.can_traverse(n)
        )
        .unwrap_or_default()
    );

    Ok(())
}

#[derive(PartialEq, Debug)]
enum Elevation {
    Lowest,
    Highest,
    Level(char),
}

impl Elevation {
    fn from_char(c: char) -> Self {
        if c == 'E' {
            Elevation::Highest
        } else if c == 'S' {
            Elevation::Lowest
        } else {
            Elevation::Level(c)
        }
    }

    fn height(&self) -> u8 {
        match self {
            Elevation::Highest => 26,
            Elevation::Lowest => 1,
            Elevation::Level(l) => *l as u8 - b'a' + 1,
        }
    }

    fn can_traverse(&self, from: &Self) -> bool {
        self.height() - 1 <= from.height()
    }

    fn is_highest(&self) -> bool {
        matches!(self, Elevation::Highest)
    }
}

fn distance<F1, F2>(start: &Point2d, grid: &Grid, target: F1, can_traverse: F2) -> Option<u32>
where
    F1: Fn(&Elevation) -> bool,
    F2: Fn(&Elevation, &Elevation) -> bool,
{
    let mut visited = HashSet::new();
    let mut positions = VecDeque::new();
    positions.push_back(PositionDistance(0, start.clone()));
    visited.insert(start.clone());

    loop {
        if let Some(next) = positions.pop_front() {
            let point = &next.1;
            let neighbours = vec![point.left(), point.right(), point.up(), point.down()];
            let current = grid.0.get(point).unwrap();
            for n in neighbours {
                if !visited.contains(&n) {
                    if let Some(e) = grid.0.get(&n) {
                        if can_traverse(current, e) {
                            if target(e) {
                                return Some(next.0 + 1);
                            }
                            positions.push_back(PositionDistance(1 + next.0, n.clone()));
                            visited.insert(n.clone());
                        }
                    }
                }
            }
        } else {
            return None;
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct PositionDistance(u32, Point2d);

struct Grid(HashMap<Point2d, Elevation>);

impl Grid {
    fn from_strs(strs: &[String]) -> Self {
        let mut map = HashMap::new();
        strs.iter()
            .enumerate()
            .flat_map(|(y, s)| {
                s.chars()
                    .enumerate()
                    .map(move |(x, c)| (x, y, Elevation::from_char(c)))
            })
            .for_each(|(x, y, e)| {
                map.insert(
                    Point2d::new(x.try_into().unwrap(), y.try_into().unwrap()),
                    e,
                );
            });
        Grid(map)
    }

    fn start(&self) -> Point2d {
        self.0
            .iter()
            .find(|(_, e)| **e == Elevation::Lowest)
            .unwrap()
            .0
            .clone()
    }

    fn end(&self) -> Point2d {
        self.0
            .iter()
            .find(|(_, e)| **e == Elevation::Highest)
            .unwrap()
            .0
            .clone()
    }
}
