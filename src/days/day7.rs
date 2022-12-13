pub fn run(lines: Vec<String>) -> Result<(), String> {
    let parsed = lines
        .iter()
        .map(|s| Output::from_str(s))
        .collect::<Result<Vec<_>, _>>()?;

    let mut fs = Fs::from_output(parsed);

    let sized = fs.sized_dirs();

    let part1: u32 = sized
        .iter()
        .filter_map(|s| if s.size <= 100000 { Some(s.size) } else { None })
        .sum();

    println!("Part 1 {}", part1);

    let size = sized.last().unwrap().size;
    let available = 70000000 - size;
    let to_delete = 30000000 - available;

    let part2 = sized
        .iter()
        .filter_map(|s| {
            if s.size >= to_delete {
                Some(s.size)
            } else {
                None
            }
        })
        .min()
        .unwrap_or_default();

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
        if let Some(dir) = str.strip_prefix("$ cd ") {
            Ok(Command::Cd(dir.to_string()))
        } else {
            Ok(Command::Ls)
        }
    }
}

impl LsOut {
    fn from_str(str: &str) -> Result<LsOut, String> {
        if let Some(name) = str.strip_prefix("dir ") {
            Ok(LsOut::Dir(name.to_string()))
        } else {
            let split = str.find(' ').ok_or("could not find")?;
            let name = str[split + 1..].to_string();
            let size = str[..split].parse::<u32>().map_err(|e| e.to_string())?;
            Ok(LsOut::File(name, size))
        }
    }
}

impl Output {
    fn from_str(str: &str) -> Result<Output, String> {
        if str.starts_with('$') {
            let command = Command::from_str(str)?;
            Ok(Output::Command(command))
        } else {
            let ls_out = LsOut::from_str(str)?;
            Ok(Output::Ls(ls_out))
        }
    }
}

struct Dir {
    name: String,
    sub_dirs: Vec<usize>,
    parent: Option<usize>,
    files: Vec<File>,
    maybe_size: Option<u32>,
}
struct File(String, u32);

const ROOT: usize = 0;
struct Fs {
    pwd: usize,
    dirs: Vec<Dir>,
}

#[derive(Debug)]
struct SizedDir {
    size: u32,
}

impl Fs {
    fn new() -> Self {
        Fs {
            pwd: ROOT,
            dirs: vec![Dir {
                name: "/".to_string(),
                sub_dirs: Vec::new(),
                parent: None,
                files: Vec::new(),
                maybe_size: None,
            }],
        }
    }

    fn cd(&mut self, dir: &str) {
        if dir == "/" {
            self.pwd = ROOT;
        } else if dir == ".." {
            if let Some(p) = self.dirs[self.pwd].parent {
                self.pwd = p
            }
        } else if let Some(p) = self.dirs[self.pwd]
            .sub_dirs
            .iter()
            .copied()
            .find(|d| self.dirs[*d].name == dir)
        {
            self.pwd = p;
        }
    }

    fn touch(&mut self, name: &str, size: u32) {
        self.dirs[self.pwd].files.push(File(name.to_string(), size))
    }

    fn mkdir(&mut self, name: &str) {
        let dir = Dir {
            name: name.to_string(),
            sub_dirs: Vec::new(),
            parent: Some(self.pwd),
            files: Vec::new(),
            maybe_size: None,
        };
        let id = self.dirs.len();
        self.dirs.push(dir);
        self.dirs[self.pwd].sub_dirs.push(id);
    }

    fn from_output<T: IntoIterator<Item = Output>>(output: T) -> Self {
        let mut fs = Fs::new();
        for output in output.into_iter() {
            match output {
                Output::Command(Command::Ls) => (),
                Output::Command(Command::Cd(dir)) => {
                    fs.cd(&dir);
                }
                Output::Ls(LsOut::File(name, size)) => {
                    fs.touch(&name, size);
                }
                Output::Ls(LsOut::Dir(name)) => fs.mkdir(&name),
            }
        }
        fs
    }

    fn sized_dirs(&mut self) -> Vec<SizedDir> {
        let mut sized = Vec::new();

        let sizes: Vec<_> = self
            .dirs
            .iter_mut()
            .enumerate()
            .map(|(i, d)| (i, d.maybe_size))
            .rev()
            .collect();

        for (i, s) in sizes {
            if let Some(size) = s {
                sized.push(SizedDir { size });
            } else {
                let d = &self.dirs[i];
                let file_size: u32 = d.files.iter().map(|f| f.1).sum();

                let dir_size: u32 = d
                    .sub_dirs
                    .iter()
                    .map(|d| {
                        self.dirs[*d]
                            .maybe_size
                            .expect("filled in by previous iteration")
                    })
                    .sum();
                let size = file_size + dir_size;
                sized.push(SizedDir { size });
                self.dirs[i].maybe_size = Some(size);
            }
        }

        sized
    }
}
