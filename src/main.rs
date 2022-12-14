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

type Programs = HashMap<i32, Box<dyn Fn(Vec<String>) -> Result<(), String>>>;

fn main() {
    let opt = Opt::from_args();
    let mut programs: Programs = HashMap::new();
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
    programs.insert(11, Box::new(day11::run));
    programs.insert(12, Box::new(day12::run));
    programs.insert(13, Box::new(day13::run));
    programs.insert(14, Box::new(day14::run));
    programs.insert(15, Box::new(day15::run));
    programs.insert(16, Box::new(day16::run));
    programs.insert(17, Box::new(day17::run));
    programs.insert(18, Box::new(day18::run));
    programs.insert(19, Box::new(day19::run));
    programs.insert(20, Box::new(day20::run));
    programs.insert(21, Box::new(day21::run));
    programs.insert(22, Box::new(day22::run));
    programs.insert(23, Box::new(day23::run));
    programs.insert(24, Box::new(day24::run));
    programs.insert(25, Box::new(day25::run));

    let program = programs
        .get(&opt.day)
        .unwrap_or_else(|| default_error_handler(format!("Undefined day: {}", opt.day).as_str()));
    let file_contents: Vec<String> = fs::File::open(format!("input/day{}.txt", opt.day))
        .and_then(|file| BufReader::new(file).lines().collect())
        .unwrap_or_else(default_error_handler);
    program(file_contents).unwrap_or_else(default_error_handler)
}
