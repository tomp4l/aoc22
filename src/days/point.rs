#[derive(Hash, Eq, PartialEq, Clone, Debug, PartialOrd, Ord)]
pub struct Point2d(i32, i32);

impl Point2d {
    pub fn new(x: i32, y: i32) -> Point2d {
        Point2d(x, y)
    }

    pub fn left(&self) -> Point2d {
        self.add_x(-1)
    }

    pub fn right(&self) -> Point2d {
        self.add_x(1)
    }

    pub fn up(&self) -> Point2d {
        self.add_y(-1)
    }

    pub fn down(&self) -> Point2d {
        self.add_y(1)
    }

    pub fn add_x(&self, x: i32) -> Point2d {
        Point2d(self.0 + x, self.1)
    }

    pub fn add_y(&self, y: i32) -> Point2d {
        Point2d(self.0, self.1 + y)
    }

    pub fn is_neighbour(&self, other: &Point2d) -> bool {
        self.0.abs_diff(other.0) <= 1 && self.1.abs_diff(other.1) <= 1
    }

    pub fn x(&self) -> i32 {
        self.0
    }

    pub fn y(&self) -> i32 {
        self.1
    }
}
