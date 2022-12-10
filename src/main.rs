use std::{
    collections::HashMap,
    fmt::Debug,
    fs,
    io::{BufRead, BufReader},
    process::exit,
};

use structopt::StructOpt;

mod days;

use days::*;

#[derive(StructOpt)]
struct Opt {
    #[structopt(name = "day")]
    day: i32,
}

fn default_error_handler<E: Debug, R>(error: E) -> R {
    println!("{:#?}", error);
    exit(1);
}

fn main() {
    let opt = Opt::from_args();
    let mut programs: HashMap<i32, Box<dyn Fn(Vec<String>) -> Result<_, _>>> = HashMap::new();
    programs.insert(1, Box::new(day1::run));
    programs.insert(2, Box::new(day2::run));
    programs.insert(3, Box::new(day3::run));
    programs.insert(4, Box::new(day4::run));
    programs.insert(5, Box::new(day5::run));
    programs.insert(6, Box::new(day6::run));
    programs.insert(7, Box::new(day7::run));
    programs.insert(8, Box::new(day8::run));
    programs.insert(9, Box::new(day9::run));
    programs.insert(10, Box::new(day10::run));

    let program = programs
        .get(&opt.day)
        .unwrap_or_else(|| default_error_handler(format!("Undefined day: {}", opt.day).as_str()));
    let file_contents: Vec<String> = fs::File::open(format!("input/day{}.txt", opt.day))
        .and_then(|file| BufReader::new(file).lines().collect())
        .unwrap_or_else(default_error_handler);
    program(file_contents).unwrap_or_else(default_error_handler)
}
