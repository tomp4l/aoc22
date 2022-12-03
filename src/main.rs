use std::{
    collections::HashMap,
    fs,
    io::{BufRead, BufReader},
};

use structopt::StructOpt;

mod days;

use days::day1;

#[derive(StructOpt)]
struct Opt {
    #[structopt(name = "day")]
    day: i32,
}

fn main() {
    let opt = Opt::from_args();
    let programs = HashMap::from([(1, Box::new(|lines| day1::run(lines)))]);
    let program = programs
        .get(&opt.day)
        .expect(format!("Undefined day: {}", opt.day).as_str());
    let file_contents: Vec<String> = fs::File::open(format!("input/day{}.txt", opt.day))
        .and_then(|file| BufReader::new(file).lines().collect())
        .expect("Could not open file");
    program(file_contents);

    // println!("{:?}", program);
}
