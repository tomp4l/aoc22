use std::collections::{hash_map::Keys, HashMap};

pub fn run(lines: Vec<String>) -> Result<(), String> {
    let grid = Grid::from_lines(lines)?;

    let part1 = grid.all_points().filter(|p| grid.tree_visible(p)).count();

    println!("Part 1 {}", part1);

    let part2 = grid
        .all_points()
        .map(|p| grid.visible_distance(p))
        .max()
        .unwrap_or_default();

    println!("Part 2 {}", part2);
    Ok(())
}

#[derive(Debug)]
struct Tree(i32);
#[derive(Hash, Eq, PartialEq, Debug)]
struct Point(i32, i32);

impl Point {
    fn left(&self) -> Point {
        Point(self.0 - 1, self.1)
    }

    fn right(&self) -> Point {
        Point(self.0 + 1, self.1)
    }

    fn up(&self) -> Point {
        Point(self.0, self.1 - 1)
    }

    fn down(&self) -> Point {
        Point(self.0, self.1 + 1)
    }
}

struct Grid(HashMap<Point, Tree>);

impl Grid {
    fn from_lines(lines: Vec<String>) -> Result<Grid, String> {
        let positions = lines
            .iter()
            .enumerate()
            .flat_map(|(y, l)| l.chars().enumerate().map(move |(x, c)| (x, y, c)));

        let mut map = HashMap::new();

        for (x, y, c) in positions {
            let size = c.to_string().parse::<i32>().map_err(|e| e.to_string())?;
            map.insert(Point(x as i32, y as i32), Tree(size));
        }

        Ok(Grid(map))
    }
}

fn visible<F>(grid: &HashMap<Point, Tree>, p: &Point, tree: &Tree, next: F) -> bool
where
    F: Fn(&Point) -> Point,
{
    let mut point = next(p);
    while let Some(other) = grid.get(&point) {
        if other.0 >= tree.0 {
            return false;
        }
        point = next(&point);
    }

    true
}

fn visible_distance<F>(grid: &HashMap<Point, Tree>, p: &Point, tree: &Tree, next: F) -> i32
where
    F: Fn(&Point) -> Point,
{
    let mut point = next(p);
    let mut distance = 0;
    while let Some(other) = grid.get(&point) {
        distance += 1;
        if other.0 >= tree.0 {
            return distance;
        }
        point = next(&point);
    }

    distance
}

impl Grid {
    fn tree_visible(&self, p: &Point) -> bool {
        let grid = &self.0;

        if let Some(tree) = grid.get(p) {
            visible(grid, p, tree, Point::left)
                || visible(grid, p, tree, Point::right)
                || visible(grid, p, tree, Point::up)
                || visible(grid, p, tree, Point::down)
        } else {
            false
        }
    }

    fn visible_distance(&self, p: &Point) -> i32 {
        let grid = &self.0;

        if let Some(tree) = grid.get(p) {
            visible_distance(grid, p, tree, Point::left)
                * visible_distance(grid, p, tree, Point::right)
                * visible_distance(grid, p, tree, Point::up)
                * visible_distance(grid, p, tree, Point::down)
        } else {
            0
        }
    }

    fn all_points(&self) -> Keys<Point, Tree> {
        self.0.keys()
    }
}
