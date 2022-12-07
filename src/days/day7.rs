use std::collections::HashMap;

pub fn run(lines: Vec<String>) -> Result<(), String> {
    let parsed = lines
        .iter()
        .map(|s| Output::from_str(s))
        .collect::<Result<Vec<_>, _>>()?;

    let root = Dir::from_output(parsed);
    let small = root.find_small_dirs();
    let part1: u32 = small.iter().map(|v| v.1).sum();

    println!("Part 1 {}", part1);

    let size = root.dir_sizes().total();
    let available = 70000000 - size;
    let to_delete = 30000000 - available;

    let part2 = root.find_smallest(to_delete);

    println!("Part 2 {}", part2);

    Ok(())
}

#[derive(Debug)]
enum Command {
    Ls,
    Cd(String),
}

#[derive(Debug)]
enum LsOut {
    Dir(String),
    File(String, u32),
}

#[derive(Debug)]
enum Output {
    Command(Command),
    Ls(LsOut),
}

impl Command {
    fn from_str(str: &str) -> Result<Command, String> {
        if str.starts_with("$ cd ") {
            let dir = &str[5..];
            Ok(Command::Cd(dir.to_string()))
        } else {
            Ok(Command::Ls)
        }
    }
}

impl LsOut {
    fn from_str(str: &str) -> Result<LsOut, String> {
        if str.starts_with("dir ") {
            let name = &str[4..];
            Ok(LsOut::Dir(name.to_string()))
        } else {
            let split = str.find(" ").ok_or("could not find")?;
            let name = str[split + 1..].to_string();
            let size = str[..split].parse::<u32>().map_err(|e| e.to_string())?;
            Ok(LsOut::File(name, size))
        }
    }
}

impl Output {
    fn from_str(str: &str) -> Result<Output, String> {
        if str.starts_with("$") {
            let command = Command::from_str(str)?;
            Ok(Output::Command(command))
        } else {
            let ls_out = LsOut::from_str(str)?;
            Ok(Output::Ls(ls_out))
        }
    }
}

#[derive(Debug)]
struct Dir {
    name: String,
    content: Vec<Fs>,
}

#[derive(Debug)]
enum Fs {
    Dir(Dir),
    File(String, u32),
}

impl Dir {
    fn from_output<T: IntoIterator<Item = Output>>(output: T) -> Dir {
        let mut current_id = 0;
        let mut current_dir = current_id;

        let mut dirs = HashMap::new();
        dirs.insert(
            current_id,
            (
                "/".to_string(),
                Vec::<(String, i32)>::new(), // dir name + id
                Vec::new(),                  // files
                None,
            ),
        );

        for output in output.into_iter() {
            match output {
                Output::Command(Command::Ls) => (),
                Output::Command(Command::Cd(dir)) => {
                    if dir != "/" {
                        let t = dirs.get(&current_dir).unwrap();
                        if dir == ".." {
                            current_dir = t.3.unwrap()
                        } else {
                            let d = t.1.iter().find(|(n, _)| *n == dir).unwrap();
                            current_dir = d.1
                        }
                    }
                }
                Output::Ls(LsOut::File(name, size)) => {
                    let file = Fs::File(name, size);
                    let t = dirs.get_mut(&current_dir).unwrap();
                    t.2.push(file);
                }
                Output::Ls(LsOut::Dir(name)) => {
                    current_id += 1;
                    let parent = Some(current_dir);
                    dirs.insert(current_id, (name.clone(), Vec::new(), Vec::new(), parent));
                    let t = dirs.get_mut(&current_dir).unwrap();
                    t.1.push((name.to_string(), current_id));
                }
            }
        }

        let mut built_dirs = HashMap::new();

        loop {
            if dirs.is_empty() {
                break;
            }
            let keys = dirs.keys().copied().collect::<Vec<_>>();
            for k in keys {
                let v = dirs.get(&k).unwrap();
                let all_dirs = v.1.iter().map(|v| v.1).all(|k| built_dirs.contains_key(&k));

                if all_dirs {
                    let v1 = dirs.remove(&k).unwrap();

                    let mut ds: Vec<_> =
                        v1.1.into_iter()
                            .map(|(_, k)| Fs::Dir(built_dirs.remove(&k).unwrap()))
                            .collect();
                    ds.extend(v1.2);
                    let dir = Dir {
                        name: v1.0,
                        content: ds,
                    };
                    built_dirs.insert(k, dir);
                }
            }
        }

        built_dirs.remove(&0).unwrap()
    }

    fn dir_sizes(&self) -> SizedDir {
        let files: u32 = self
            .content
            .iter()
            .map(|f| match f {
                Fs::File(_, s) => *s,
                Fs::Dir(_) => 0,
            })
            .sum();

        let dirs: Vec<_> = self
            .content
            .iter()
            .filter_map(|f| match f {
                Fs::File(_, _) => None,
                Fs::Dir(dir) => Some((dir.name.clone(), dir.dir_sizes().total())),
            })
            .collect();

        SizedDir { files, dirs }
    }

    fn find_small_dirs(&self) -> Vec<(String, u32)> {
        let mut small: Vec<_> = Vec::new();

        for fs in &self.content {
            if let Fs::Dir(dir) = fs {
                let size = dir.dir_sizes().total();
                if size < 100000 {
                    small.push((dir.name.clone(), size));
                }
                small.extend(dir.find_small_dirs());
            }
        }

        small
    }

    fn find_smallest(&self, min_size: u32) -> u32 {
        let mut smallest_size = self.dir_sizes().total();
        for fs in &self.content {
            if let Fs::Dir(dir) = fs {
                if dir.dir_sizes().total() >= min_size {
                    let size = dir.find_smallest(min_size);
                    if size <= smallest_size {
                        smallest_size = size;
                    }
                }
            }
        }

        smallest_size
    }
}

#[derive(Debug)]
struct SizedDir {
    files: u32,
    dirs: Vec<(String, u32)>,
}

impl SizedDir {
    fn total(&self) -> u32 {
        let dirs: u32 = self.dirs.iter().map(|v| v.1).sum();
        dirs + self.files
    }
}
