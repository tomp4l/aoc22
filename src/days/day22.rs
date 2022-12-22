use std::collections::HashMap;

use super::point::Point2d;

pub fn run(lines: Vec<String>) -> Result<(), String> {
    let split: Vec<_> = lines.split(|l| l.is_empty()).collect();
    if split.len() != 2 {
        return Err("bad input".to_owned());
    }

    let mut map = Map::from_strs(split[0]);
    let path = Path::from_str(&split[1].concat());

    let Monkey(p, o) = path.traverse(&map);

    let orient_score = match o {
        Orient::Right => 0,
        Orient::Down => 1,
        Orient::Left => 2,
        Orient::Up => 3,
    };

    let part1 = 1000 * p.y() + 4 * p.x() + orient_score;

    println!("Part 1: {}", part1);

    map.make_cube();

    let Monkey(p, o) = path.traverse(&map);

    let orient_score = match o {
        Orient::Right => 0,
        Orient::Down => 1,
        Orient::Left => 2,
        Orient::Up => 3,
    };

    let part1 = 1000 * p.y() + 4 * p.x() + orient_score;

    println!("Part 2: {}", part1);

    Ok(())
}

#[derive(Debug)]
enum Rot {
    Left,
    Right,
}

#[derive(Debug)]
struct Step(Rot, u8);

#[derive(Debug)]
struct Path {
    forward: u8,
    rest: Vec<Step>,
}

impl Path {
    fn from_str(str: &str) -> Self {
        let mut chars = str.chars().peekable();

        let mut first = Vec::new();

        while let Some(c) = chars.peek() {
            if c == &'L' || c == &'R' {
                break;
            }
            first.push(*c);
            chars.next();
        }

        let mut rest = Vec::new();
        let mut rot = Rot::Left;
        let mut distance = Vec::new();

        while let Some(c) = chars.next() {
            if c == 'L' || c == 'R' {
                if !distance.is_empty() {
                    let d: String = std::mem::take(&mut distance).into_iter().collect();
                    let step = Step(rot, d.parse::<u8>().unwrap());
                    rest.push(step);
                }

                if c == 'L' {
                    rot = Rot::Left;
                } else {
                    rot = Rot::Right;
                }
            } else {
                distance.push(c);
            }
        }

        let d: String = distance.into_iter().collect();
        let step = Step(rot, d.parse::<u8>().unwrap());
        rest.push(step);

        Path {
            forward: first.into_iter().collect::<String>().parse::<u8>().unwrap(),
            rest: rest,
        }
    }

    fn traverse(&self, map: &Map) -> Monkey {
        let start = map
            .0
            .keys()
            .filter(|p| p.y() == 1)
            .min_by_key(|p| p.x())
            .unwrap()
            .to_owned();

        let mut monkey = Monkey::new(start);

        monkey.forward(map, self.forward);

        for Step(rot, distance) in &self.rest {
            monkey.rotate(rot);
            monkey.forward(map, *distance);
        }

        monkey
    }
}

#[derive(Debug)]
struct Monkey(Point2d, Orient);

impl Monkey {
    fn new(start: Point2d) -> Self {
        Monkey(start, Orient::Right)
    }

    fn forward(&mut self, map: &Map, distance: u8) {
        for _ in 0..distance {
            let (p, o) = map.next(&self.0, &self.1);
            if let Some(s) = map.0.get(&p) {
                match s {
                    Square::Empty => {
                        self.0 = p;
                        self.1 = o;
                    }
                    Square::Rock => break,
                }
            } else {
                unreachable!()
            }
        }
    }

    fn rotate(&mut self, rot: &Rot) {
        let next = match (&self.1, rot) {
            (Orient::Right, Rot::Right) => Orient::Down,
            (Orient::Right, Rot::Left) => Orient::Up,
            (Orient::Left, Rot::Right) => Orient::Up,
            (Orient::Left, Rot::Left) => Orient::Down,
            (Orient::Up, Rot::Right) => Orient::Right,
            (Orient::Up, Rot::Left) => Orient::Left,
            (Orient::Down, Rot::Right) => Orient::Left,
            (Orient::Down, Rot::Left) => Orient::Right,
        };
        self.1 = next;
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Orient {
    Up,
    Down,
    Left,
    Right,
}

enum Square {
    Empty,
    Rock,
}

#[derive(Debug)]
struct Neighbours(Face, Face, Orient, Face, Orient, Face, Orient, Face, Orient);

struct Map(HashMap<Point2d, Square>, Vec<Neighbours>, bool);

impl Map {
    fn from_strs(strs: &[String]) -> Self {
        let mut y = 1;
        let mut map = HashMap::new();
        for line in strs {
            let mut x = 1;
            for char in line.chars() {
                let point = Point2d::new(x, y);
                if char == '.' {
                    map.insert(point, Square::Empty);
                } else if char == '#' {
                    map.insert(point, Square::Rock);
                }

                x += 1;
            }
            y += 1;
        }

        let mut width = map.len() as i32;
        for y in 1..y {
            width = map
                .keys()
                .filter(|p| p.y() == y)
                .map(|p| p.x())
                .max()
                .unwrap_or_default()
                - map
                    .keys()
                    .filter(|p| p.y() == y)
                    .map(|p| p.x())
                    .min()
                    .unwrap_or_default();
        }

        let top_left = map
            .keys()
            .filter(|p| p.y() == 1)
            .min_by_key(|p| p.x())
            .unwrap()
            .to_owned();

        let side = Face(
            top_left.clone(),
            top_left.add_x(width),
            top_left.add_y(width),
            top_left.add_x(width).add_y(width),
        );

        let mut sides = vec![side.clone()];
        let mut remaining_sides = vec![side];

        while let Some(side) = remaining_sides.pop() {
            let top_left = &side.0;
            let bottom_right = &side.3;
            if map.contains_key(&top_left.left()) {
                let side = Face(
                    top_left.left().add_x(-width),
                    top_left.left(),
                    top_left.left().add_x(-width).add_y(width),
                    top_left.left().add_y(width),
                );

                if !sides.contains(&side) {
                    sides.push(side.clone());
                    remaining_sides.push(side);
                }
            }
            if map.contains_key(&top_left.up()) {
                let side = Face(
                    top_left.up().add_y(-width),
                    top_left.up().add_y(-width).add_x(width),
                    top_left.up(),
                    top_left.up().add_x(width),
                );

                if !sides.contains(&side) {
                    sides.push(side.clone());
                    remaining_sides.push(side);
                }
            }
            if map.contains_key(&bottom_right.right()) {
                let side = Face(
                    bottom_right.right().add_y(-width),
                    bottom_right.right().add_y(-width).add_x(width),
                    bottom_right.right(),
                    bottom_right.right().add_x(width),
                );

                if !sides.contains(&side) {
                    sides.push(side.clone());
                    remaining_sides.push(side);
                }
            }
            if map.contains_key(&bottom_right.down()) {
                let side = Face(
                    bottom_right.down().add_x(-width),
                    bottom_right.down(),
                    bottom_right.down().add_x(-width).add_y(width),
                    bottom_right.down().add_y(width),
                );

                if !sides.contains(&side) {
                    sides.push(side.clone());
                    remaining_sides.push(side);
                }
            }
        }

        let mut side_faces = Vec::new();

        for side in &sides {
            let l = left_face(side, &sides);
            let r = right_face(side, &sides);
            let u = up_face(side, &sides);
            let d = down_face(side, &sides);

            if l.is_some() {
                let left = l.unwrap();
                if r.is_some() {
                    todo!()
                } else {
                    if u.is_some() {
                        let up = u.unwrap();
                        if d.is_some() {
                            todo!()
                        } else {
                            let ld = down_face(left, &sides);
                            if ld.is_some() {
                                let down = ld.unwrap();
                                let uu = up_face(up, &sides);
                                if uu.is_some() {
                                    let uur = right_face(uu.unwrap(), &sides);
                                    if uur.is_some() {
                                        let right = uur.unwrap();
                                        let sides = Neighbours(
                                            side.clone(),
                                            left.clone(),
                                            Orient::Right,
                                            up.clone(),
                                            Orient::Down,
                                            right.clone(),
                                            Orient::Right,
                                            down.clone(),
                                            Orient::Right,
                                        );
                                        side_faces.push(sides);
                                    } else {
                                        todo!();
                                    }
                                } else {
                                    todo!();
                                }
                            } else {
                                todo!()
                            }
                        }
                    } else {
                        if d.is_some() {
                            todo!()
                        } else {
                            let ld = down_face(left, &sides);
                            if ld.is_some() {
                                let down = ld.unwrap();
                                let dd = down_face(down, &sides);
                                if dd.is_some() {
                                    let right = dd.unwrap();
                                    let rl = left_face(right, &sides);
                                    if rl.is_some() {
                                        let rld = down_face(rl.unwrap(), &sides);
                                        if rld.is_some() {
                                            let up = rld.unwrap();
                                            let sides = Neighbours(
                                                side.clone(),
                                                left.clone(),
                                                Orient::Right,
                                                up.clone(),
                                                Orient::Down,
                                                right.clone(),
                                                Orient::Right,
                                                down.clone(),
                                                Orient::Right,
                                            );
                                            side_faces.push(sides);
                                        } else {
                                            todo!()
                                        }
                                    } else {
                                        todo!()
                                    }
                                } else {
                                    todo!()
                                }
                            } else {
                                todo!()
                            }
                        }
                    }
                }
            } else {
                if r.is_some() {
                    let right = r.unwrap();
                    if u.is_some() {
                        todo!()
                    } else {
                        if d.is_some() {
                            let down = d.unwrap();
                            let dd = down_face(down, &sides);
                            if dd.is_some() {
                                let down_down = dd.unwrap();
                                let ddl = left_face(down_down, &sides);
                                if ddl.is_some() {
                                    let left = ddl.unwrap();
                                    let ld = down_face(left, &sides);
                                    if ld.is_some() {
                                        let up = ld.unwrap();
                                        let sides = Neighbours(
                                            side.clone(),
                                            left.clone(),
                                            Orient::Left,
                                            up.clone(),
                                            Orient::Left,
                                            right.clone(),
                                            Orient::Left,
                                            down.clone(),
                                            Orient::Up,
                                        );
                                        side_faces.push(sides);
                                    } else {
                                        todo!()
                                    }
                                } else {
                                    todo!()
                                }
                            } else {
                                let ru = up_face(right, &sides);
                                if ru.is_some() {
                                    let up = ru.unwrap();
                                    let uu = up_face(up, &sides);
                                    if uu.is_some() {
                                        let left = uu.unwrap();
                                        let sides = Neighbours(
                                            side.clone(),
                                            left.clone(),
                                            Orient::Left,
                                            up.clone(),
                                            Orient::Left,
                                            right.clone(),
                                            Orient::Left,
                                            down.clone(),
                                            Orient::Up,
                                        );
                                        side_faces.push(sides);
                                    } else {
                                        todo!();
                                    }
                                } else {
                                    todo!()
                                }
                            }
                        } else {
                            todo!()
                        }
                    }
                } else {
                    if u.is_some() {
                        let up = u.unwrap();
                        if d.is_some() {
                            let down = d.unwrap();
                            let ur = right_face(up, &sides);
                            if ur.is_some() {
                                let right = ur.unwrap();
                                let dl = left_face(down, &sides);
                                if dl.is_some() {
                                    let left = dl.unwrap();
                                    let sides = Neighbours(
                                        side.clone(),
                                        left.clone(),
                                        Orient::Up,
                                        up.clone(),
                                        Orient::Down,
                                        right.clone(),
                                        Orient::Down,
                                        down.clone(),
                                        Orient::Up,
                                    );
                                    side_faces.push(sides);
                                } else {
                                    todo!()
                                }
                            } else {
                                todo!()
                            }
                        } else {
                            let ur = right_face(up, &sides);
                            if ur.is_some() {
                                let right = ur.unwrap();
                                let ru = up_face(right, &sides);
                                if ru.is_some() {
                                    let ruu = up_face(ru.unwrap(), &sides);
                                    if ruu.is_some() {
                                        let left = ruu.unwrap();
                                        let lr = right_face(left, &sides);
                                        if lr.is_some() {
                                            let down = lr.unwrap();
                                            let sides = Neighbours(
                                                side.clone(),
                                                left.clone(),
                                                Orient::Up,
                                                up.clone(),
                                                Orient::Down,
                                                right.clone(),
                                                Orient::Down,
                                                down.clone(),
                                                Orient::Up,
                                            );
                                            side_faces.push(sides);
                                        } else {
                                            todo!()
                                        }
                                    } else {
                                        todo!()
                                    }
                                } else {
                                    todo!()
                                }
                            } else {
                                todo!();
                            }
                        }
                    } else {
                        todo!();
                    }
                }
            }
        }

        Map(map, side_faces, false)
    }

    fn make_cube(&mut self) {
        self.2 = true
    }

    fn next(&self, point: &Point2d, orient: &Orient) -> (Point2d, Orient) {
        let next = |p: &Point2d| match orient {
            Orient::Down => p.down(),
            Orient::Up => p.up(),
            Orient::Left => p.left(),
            Orient::Right => p.right(),
        };

        let prev = |p: &Point2d| match orient {
            Orient::Down => p.up(),
            Orient::Up => p.down(),
            Orient::Left => p.right(),
            Orient::Right => p.left(),
        };

        let mut ret = next(point);
        let mut or = orient.clone();
        if !self.0.contains_key(&ret) {
            if !self.2 {
                while self.0.contains_key(&prev(&ret)) {
                    ret = prev(&ret);
                }
            } else {
                let faces = &self.1;

                let p = point;

                let left = faces
                    .iter()
                    .find(|f| f.0 .0.x() == p.x() && f.0 .0.y() <= p.y() && f.0 .2.y() >= p.y());

                let right = faces
                    .iter()
                    .find(|f| f.0 .1.x() == p.x() && f.0 .0.y() <= p.y() && f.0 .2.y() >= p.y());

                let up = faces
                    .iter()
                    .find(|f| f.0 .0.y() == p.y() && f.0 .0.x() <= p.x() && f.0 .1.x() >= p.x());

                let down = faces
                    .iter()
                    .find(|f| f.0 .2.y() == p.y() && f.0 .0.x() <= p.x() && f.0 .1.x() >= p.x());

                match orient {
                    Orient::Left => {
                        let current = &left.unwrap().0;
                        let side = &left.unwrap().1;
                        let orient = &left.unwrap().2;

                        let pos = p.y() - current.0.y();

                        match orient {
                            Orient::Up => {
                                ret = side.0.add_x(pos);
                                or = Orient::Down;
                            }
                            Orient::Left => {
                                ret = side.2.add_y(-pos);
                                or = Orient::Right;
                            }
                            _ => todo!(),
                        }
                    }
                    Orient::Right => {
                        let current = &right.unwrap().0;
                        let side = &right.unwrap().5;
                        let orient = &right.unwrap().6;

                        let pos = p.y() - current.0.y();

                        match orient {
                            Orient::Down => {
                                ret = side.2.add_x(pos);
                                or = Orient::Up;
                            }
                            Orient::Right => {
                                ret = side.3.add_y(-pos);
                                or = Orient::Left;
                            }
                            _ => todo!(),
                        }
                    }
                    Orient::Up => {
                        let current = &up.unwrap().0;
                        let side = &up.unwrap().3;
                        let orient = &up.unwrap().4;

                        let pos = p.x() - current.0.x();
                        match orient {
                            Orient::Left => {
                                ret = side.0.add_y(pos);
                                or = Orient::Right;
                            }
                            Orient::Down => {
                                ret = side.2.add_x(pos);
                                or = Orient::Up;
                            }
                            _ => todo!(),
                        }
                    }
                    Orient::Down => {
                        let current = &down.unwrap().0;
                        let side = &down.unwrap().7;
                        let orient = &down.unwrap().8;
                        let pos = p.x() - current.0.x();
                        match orient {
                            Orient::Up => {
                                ret = side.0.add_x(pos);
                                or = Orient::Down;
                            }
                            Orient::Right => {
                                ret = side.1.add_y(pos);
                                or = Orient::Left;
                            }
                            _ => todo!(),
                        }
                    }
                }
            }
        }

        (ret, or)
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Face(Point2d, Point2d, Point2d, Point2d);

fn left_face<'a>(face: &Face, faces: &'a [Face]) -> Option<&'a Face> {
    faces.iter().find(|f| f.1.is_neighbour(&face.0))
}

fn right_face<'a>(face: &Face, faces: &'a [Face]) -> Option<&'a Face> {
    faces.iter().find(|f| f.0.is_neighbour(&face.1))
}

fn down_face<'a>(face: &Face, faces: &'a [Face]) -> Option<&'a Face> {
    faces.iter().find(|f| f.1.is_neighbour(&face.3))
}

fn up_face<'a>(face: &Face, faces: &'a [Face]) -> Option<&'a Face> {
    faces.iter().find(|f| f.3.is_neighbour(&face.1))
}
