use super::point::Point2d;

const Y_CHECK: i32 = 2000000;

const MIN_SEARCH: i32 = 0;
const MAX_SEARCH: i32 = 4000000;

pub fn run(lines: Vec<String>) -> Result<(), String> {
    let parsed = lines
        .iter()
        .map(|l| Sensor::from_str(l))
        .collect::<Result<Vec<_>, _>>()?;

    let mut ranges = Ranges::new();

    parsed
        .iter()
        .filter_map(|p| p.x_range(Y_CHECK))
        .for_each(|p| ranges.add(p));

    let mut beacons_at_check: Vec<_> = parsed
        .iter()
        .map(|s| s.closest_beacon.clone())
        .filter(|p| p.y() == Y_CHECK)
        .collect();

    beacons_at_check.sort();
    beacons_at_check.dedup();

    let part1 = ranges.size() - (beacons_at_check.len() as i32);

    println!("Part 1 {}", part1);

    for y in MIN_SEARCH..(MAX_SEARCH + 1) {
        let mut ranges = Ranges::new();

        parsed
            .iter()
            .filter_map(|p| p.x_range(y))
            .for_each(|p| ranges.add(p));

        if let Some(x) = ranges.first_free_space(MIN_SEARCH, MAX_SEARCH) {
            let tuning_freq: i64 = (x as i64) * 4000000 + (y as i64);
            println!("Part 2 {}", tuning_freq);
            break;
        }
    }

    Ok(())
}

#[derive(Debug)]
struct Sensor {
    location: Point2d,
    closest_beacon: Point2d,
}

impl Sensor {
    fn from_str(str: &str) -> Result<Self, String> {
        let split: Vec<_> = str.split(' ').collect();
        if split.len() != 10 {
            return Err("Wrong number of words".to_string());
        }

        let x_loc_str = split[2];
        let y_loc_str = split[3];
        let x_beac_str = split[8];
        let y_beac_str = split[9];

        let x_loc = x_loc_str
            .strip_prefix("x=")
            .and_then(|s| s.strip_suffix(','))
            .and_then(|s| s.parse::<i32>().ok())
            .ok_or_else(|| "failed to parse x location".to_string())?;
        let y_loc = y_loc_str
            .strip_prefix("y=")
            .and_then(|s| s.strip_suffix(':'))
            .and_then(|s| s.parse::<i32>().ok())
            .ok_or_else(|| "failed to parse y location".to_string())?;
        let x_beac = x_beac_str
            .strip_prefix("x=")
            .and_then(|s| s.strip_suffix(','))
            .and_then(|s| s.parse::<i32>().ok())
            .ok_or_else(|| "failed to parse x beacon".to_string())?;
        let y_beac = y_beac_str
            .strip_prefix("y=")
            .and_then(|s| s.parse::<i32>().ok())
            .ok_or_else(|| "failed to parse y beacon".to_string())?;

        Ok(Sensor {
            location: Point2d::new(x_loc, y_loc),
            closest_beacon: Point2d::new(x_beac, y_beac),
        })
    }

    fn beacon_distance(&self) -> u32 {
        self.location.x().abs_diff(self.closest_beacon.x())
            + self.location.y().abs_diff(self.closest_beacon.y())
    }

    fn x_range(&self, y: i32) -> Option<(i32, i32)> {
        let distance = self.beacon_distance();

        if self.location.y().abs_diff(y) > distance {
            return None;
        }

        let diff: i32 = (distance - self.location.y().abs_diff(y))
            .try_into()
            .unwrap();

        let x_min = self.location.x() - diff;
        let x_max = self.location.x() + diff;

        Some((x_min, x_max))
    }
}

struct Ranges(Vec<(i32, i32)>);

impl Ranges {
    fn new() -> Self {
        Ranges(Vec::new())
    }

    fn add(&mut self, mut r: (i32, i32)) {
        let mut merged = false;
        for p in self.0.iter_mut() {
            if r.1 >= p.0 && r.0 <= p.1 {
                r = (p.0.min(r.0), p.1.max(r.1));
                *p = r;
                merged = true;
            }
        }

        if !merged {
            self.0.push(r);
            self.0.sort()
        } else {
            self.0.dedup();

            let biggest = self
                .0
                .iter()
                .copied()
                .filter(|(x, _)| *x == r.0)
                .max_by_key(|t| t.1)
                .unwrap();

            self.0.retain(|p| p.0 != biggest.0 || *p == biggest);
        }
    }

    fn first_free_space(&self, min_x: i32, max_x: i32) -> Option<i32> {
        for b in &self.0 {
            if b.0 > min_x && b.0 <= max_x + 1 {
                return Some(b.0 - 1);
            }
        }

        None
    }

    fn size(&self) -> i32 {
        self.0.iter().map(|(a, b)| b - a + 1).sum()
    }
}

#[cfg(test)]
mod test {
    use super::Ranges;

    #[test]
    fn range() {
        let mut ranges = Ranges::new();

        ranges.add((10, 10));

        assert_eq!(ranges.size(), 1);
        assert_eq!(ranges.first_free_space(0, 10), Some(9));

        ranges.add((0, 0));

        assert_eq!(ranges.size(), 2);
        assert_eq!(ranges.first_free_space(0, 10), Some(9));

        ranges.add((10, 10));

        assert_eq!(ranges.size(), 2);
        assert_eq!(ranges.first_free_space(0, 10), Some(9));

        ranges.add((9, 11));

        assert_eq!(ranges.size(), 4);
        assert_eq!(ranges.first_free_space(0, 10), Some(8));

        ranges.add((10, 12));

        assert_eq!(ranges.size(), 5);

        ranges.add((8, 10));

        assert_eq!(ranges.size(), 6);
        assert_eq!(ranges.first_free_space(0, 10), Some(7));

        ranges.add((15, 20));

        assert_eq!(ranges.size(), 12);

        ranges.add((11, 17));

        assert_eq!(ranges.size(), 14);

        ranges.add((0, 24));

        assert_eq!(ranges.size(), 25);
        assert_eq!(ranges.first_free_space(0, 10), None);
    }
}
