use super::point::Point2d;
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

struct Grid(HashMap<Point2d, Tree>);

impl Grid {
    fn from_lines(lines: Vec<String>) -> Result<Grid, String> {
        let positions = lines
            .iter()
            .enumerate()
            .flat_map(|(y, l)| l.chars().enumerate().map(move |(x, c)| (x, y, c)));

        let mut map = HashMap::new();

        for (x, y, c) in positions {
            let size = c.to_string().parse::<i32>().map_err(|e| e.to_string())?;
            map.insert(Point2d::new(x as i32, y as i32), Tree(size));
        }

        Ok(Grid(map))
    }
}

fn visible<F>(grid: &HashMap<Point2d, Tree>, p: &Point2d, tree: &Tree, next: F) -> bool
where
    F: Fn(&Point2d) -> Point2d,
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

fn visible_distance<F>(grid: &HashMap<Point2d, Tree>, p: &Point2d, tree: &Tree, next: F) -> i32
where
    F: Fn(&Point2d) -> Point2d,
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
    fn tree_visible(&self, p: &Point2d) -> bool {
        let grid = &self.0;

        if let Some(tree) = grid.get(p) {
            visible(grid, p, tree, Point2d::left)
                || visible(grid, p, tree, Point2d::right)
                || visible(grid, p, tree, Point2d::up)
                || visible(grid, p, tree, Point2d::down)
        } else {
            false
        }
    }

    fn visible_distance(&self, p: &Point2d) -> i32 {
        let grid = &self.0;

        if let Some(tree) = grid.get(p) {
            visible_distance(grid, p, tree, Point2d::left)
                * visible_distance(grid, p, tree, Point2d::right)
                * visible_distance(grid, p, tree, Point2d::up)
                * visible_distance(grid, p, tree, Point2d::down)
        } else {
            0
        }
    }

    fn all_points(&self) -> Keys<Point2d, Tree> {
        self.0.keys()
    }
}
