use std::collections::HashSet;

pub fn run(lines: Vec<String>) -> Result<(), String> {
    let parsed = lines
        .iter()
        .map(|l| Cube::from_str(l))
        .collect::<Result<Vec<_>, _>>()?;

    let mut count = 0;

    for a in &parsed {
        for b in &parsed {
            if a.is_adjacent(b) {
                count += 1;
            }
        }
    }

    let total = parsed.len() * 6;
    println!("Part 1: {}", total - count);

    let as_set: HashSet<_> = parsed.iter().collect();
    let min_x = as_set.iter().min_by_key(|c| c.x).copied().unwrap().x;
    let min_y = as_set.iter().min_by_key(|c| c.y).copied().unwrap().y;
    let min_z = as_set.iter().min_by_key(|c| c.z).copied().unwrap().z;
    let max_x = as_set.iter().max_by_key(|c| c.x).copied().unwrap().x;
    let max_y = as_set.iter().max_by_key(|c| c.y).copied().unwrap().y;
    let max_z = as_set.iter().max_by_key(|c| c.z).copied().unwrap().z;

    let mut negative = HashSet::new();

    let lower_bound = 1;
    let upper_bound = 2;

    negative.insert(Cube {
        x: min_x - lower_bound,
        y: min_y - lower_bound,
        z: min_z - lower_bound,
    });

    let mut size = 0;

    loop {
        for x in min_x - lower_bound..max_x + upper_bound {
            for y in min_y - lower_bound..max_y + upper_bound {
                for z in min_z - lower_bound..max_z + upper_bound {
                    let cube = Cube { x, y, z };
                    if !as_set.contains(&cube) {
                        let mut neighbour = false;
                        for c in &negative {
                            if c.is_adjacent(&cube) {
                                neighbour = true;
                                break;
                            }
                        }
                        if neighbour {
                            negative.insert(cube);
                        }
                    }
                }
            }
        }
        let new_size = negative.len();
        if new_size == size {
            break;
        } else {
            size = new_size;
        }
    }

    let mut count = 0;

    for a in &negative {
        for b in &negative {
            if a.is_adjacent(b) {
                count += 1;
            }
        }
    }

    let x_side = max_x - min_x + lower_bound + upper_bound;
    let y_side = max_y - min_y + lower_bound + upper_bound;
    let z_side = max_z - min_z + lower_bound + upper_bound;

    let total = (negative.len() * 6) as i32 - count;
    let exterior = (x_side * y_side * 2) + (x_side * z_side * 2) + (z_side * y_side * 2);

    println!("Part 2: {}", total - exterior);

    Ok(())
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct Cube {
    x: i32,
    y: i32,
    z: i32,
}

impl Cube {
    fn from_str(str: &str) -> Result<Self, String> {
        let mut split = str.split(",");

        let x = split
            .next()
            .and_then(|p| p.parse::<i32>().ok())
            .ok_or_else(|| "could not parse x".to_owned())?;

        let y = split
            .next()
            .and_then(|p| p.parse::<i32>().ok())
            .ok_or_else(|| "could not parse x".to_owned())?;
        let z = split
            .next()
            .and_then(|p| p.parse::<i32>().ok())
            .ok_or_else(|| "could not parse x".to_owned())?;

        Ok(Cube { x, y, z })
    }

    fn is_adjacent(&self, other: &Cube) -> bool {
        let (a, b) = if self.x == other.x && self.y == other.y {
            (self.z, other.z)
        } else if self.x == other.x && self.z == other.z {
            (self.y, other.y)
        } else if self.z == other.z && self.y == other.y {
            (self.x, other.x)
        } else {
            return false;
        };

        a.abs_diff(b) == 1
    }
}
