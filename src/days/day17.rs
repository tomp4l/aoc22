const TARGET_ROCKS: u64 = 1000000000000;

pub fn run(lines: Vec<String>) -> Result<(), String> {
    let parsed: Vec<_> = lines[0]
        .chars()
        .map(|c| {
            if c == '>' {
                GasJet::Right
            } else {
                GasJet::Left
            }
        })
        .collect();

    let mut chamber = Chamber::new();
    let mut blasts = Blasts::new(parsed.clone());

    for r in 0..2022 {
        let rock = match r % 5 {
            0 => Rock::Horizontal,
            1 => Rock::Plus,
            2 => Rock::LShape,
            3 => Rock::Verticle,
            4 => Rock::Square,
            _ => unreachable!(),
        };

        chamber.drop(&rock, &mut blasts);
    }

    let part1 = chamber.height();

    println!("Part 1: {}", part1);

    let mut blasts = Blasts::new(parsed);
    let mut chamber = Chamber::new();

    let mut last_height = 0;
    let mut deltas = Vec::new();

    for r in 0..10000 {
        let rock = match r % 5 {
            0 => Rock::Horizontal,
            1 => Rock::Plus,
            2 => Rock::LShape,
            3 => Rock::Verticle,
            4 => Rock::Square,
            _ => unreachable!(),
        };

        chamber.drop(&rock, &mut blasts);

        let height = chamber.height();

        deltas.push(height - last_height);

        last_height = height;
    }

    let len = deltas.len();

    for j in (1..(len / 3)).rev() {
        let mut matches = true;
        for k in 0..j {
            matches = deltas[len - 1 - k] == deltas[len - 1 - k - j];
            matches = matches && deltas[len - 1 - k] == deltas[len - 1 - k - (2 * j)];
            if !matches {
                break;
            }
        }
        if matches {
            let repetition = &deltas[len - j..];

            let rep_len: u64 = repetition.len().try_into().unwrap();
            let rep_height: u64 = repetition.iter().sum::<usize>().try_into().unwrap();

            let mut remainder = deltas.clone();
            while let Some(r) = remainder.strip_suffix(repetition) {
                remainder = r.to_vec();
            }

            let rem_len: u64 = remainder.len().try_into().unwrap();
            let rem_height: u64 = remainder.iter().sum::<usize>().try_into().unwrap();

            let reps = (TARGET_ROCKS - rem_len) / rep_len;
            let rem = TARGET_ROCKS - rem_len - rep_len * (reps);

            let extra_height: u64 = repetition
                .iter()
                .take(rem as usize)
                .sum::<usize>()
                .try_into()
                .unwrap();

            println!("Part 2: {}", rem_height + reps * rep_height + extra_height);

            break;
        }
    }

    Ok(())
}

#[derive(Debug, Clone, Copy)]
enum GasJet {
    Left,
    Right,
}

#[derive(Debug)]
enum Rock {
    Horizontal,
    Plus,
    LShape,
    Verticle,
    Square,
}

impl Rock {
    fn left(&self, x: usize) -> usize {
        if x == 0 {
            0
        } else {
            x - 1
        }
    }

    fn right(&self, x: usize) -> usize {
        let min = match self {
            Rock::Horizontal => 7 - 4,
            Rock::Plus => 7 - 3,
            Rock::LShape => 7 - 3,
            Rock::Verticle => 7 - 1,
            Rock::Square => 7 - 2,
        };
        (x + 1).min(min)
    }

    fn draw(&self, x: usize) -> Vec<[bool; 7]> {
        let mut vec = Vec::new();
        match self {
            Rock::Horizontal => {
                let mut row = [false; 7];
                for i in 0..4 {
                    row[x + i] = true;
                }
                vec.push(row);
            }
            Rock::Plus => {
                let mut row1 = [false; 7];
                row1[x + 1] = true;
                vec.push(row1);
                let mut row2 = [false; 7];
                for i in 0..3 {
                    row2[x + i] = true;
                }
                vec.push(row2);
                let mut row3 = [false; 7];
                row3[x + 1] = true;
                vec.push(row3);
            }
            Rock::LShape => {
                let mut row1 = [false; 7];
                for i in 0..3 {
                    row1[x + i] = true;
                }
                vec.push(row1);
                let mut row2 = [false; 7];
                row2[x + 2] = true;
                vec.push(row2);
                let mut row3 = [false; 7];
                row3[x + 2] = true;
                vec.push(row3);
            }
            Rock::Verticle => {
                let mut row1 = [false; 7];
                row1[x] = true;
                vec.push(row1);
                let mut row2 = [false; 7];
                row2[x] = true;
                vec.push(row2);
                let mut row3 = [false; 7];
                row3[x] = true;
                vec.push(row3);
                let mut row4 = [false; 7];
                row4[x] = true;
                vec.push(row4);
            }
            Rock::Square => {
                let mut row1 = [false; 7];
                row1[x] = true;
                row1[x + 1] = true;
                vec.push(row1);
                let mut row2 = [false; 7];
                row2[x] = true;
                row2[x + 1] = true;
                vec.push(row2);
            }
        };

        vec
    }
}

struct Blasts {
    order: Vec<GasJet>,
    pointer: usize,
}

impl Blasts {
    fn new(order: Vec<GasJet>) -> Self {
        Blasts { order, pointer: 0 }
    }

    fn next(&mut self) -> GasJet {
        let next = self.order[self.pointer];
        self.pointer = (self.pointer + 1) % self.order.len();
        next
    }
}

#[derive(Debug)]
struct Chamber {
    occupied: Vec<[bool; 7]>,
}

impl Chamber {
    fn new() -> Self {
        Chamber {
            occupied: Vec::new(),
        }
    }

    fn drop(&mut self, rock: &Rock, blasts: &mut Blasts) {
        let mut x = 2;

        let empty = self
            .occupied
            .iter()
            .rev()
            .take_while(|r| **r == [false; 7])
            .count();

        let to_add = 4 - empty;

        for _ in 0..to_add {
            self.occupied.push([false; 7]);
        }

        for i in (0..self.occupied.len()).rev() {
            let direction = blasts.next();
            let next_x = match direction {
                GasJet::Left => rock.left(x),
                GasJet::Right => rock.right(x),
            };

            let draw = rock.draw(next_x);

            let mut conflicts = false;
            for (x, rock_row) in draw.iter().enumerate() {
                if let Some(row) = self.occupied.get(i + x) {
                    conflicts = row.iter().zip(rock_row).any(|(l, r)| *l && *r);
                    if conflicts {
                        break;
                    }
                }
            }

            if !conflicts {
                x = next_x;
            }

            let draw = rock.draw(x);
            let mut conflicts = false;
            if i > 0 {
                for (x, row_rock) in draw.iter().enumerate() {
                    if let Some(row) = self.occupied.get(i - 1 + x) {
                        conflicts = row.iter().zip(row_rock).any(|(l, r)| *l && *r);
                        if conflicts {
                            break;
                        }
                    }
                }
            }

            if i == 0 || conflicts {
                for (x, row) in draw.iter().enumerate() {
                    for (j, r) in row.iter().enumerate() {
                        self.occupied[i + x][j] = self.occupied[i + x][j] || *r;
                    }
                }

                break;
            }
        }
    }

    #[allow(dead_code)]
    fn print(&self) {
        for x in self.occupied.iter().rev() {
            for b in x {
                if *b {
                    print!("#")
                } else {
                    print!(" ")
                }
            }
            println!()
        }
    }

    fn height(&self) -> usize {
        self.occupied
            .iter()
            .take_while(|x| x.iter().any(|b| *b))
            .count()
    }
}
