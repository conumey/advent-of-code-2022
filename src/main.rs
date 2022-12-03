use std::{fs::{File}, io::{BufReader, BufRead}, env};

pub mod days;

fn main() {
    println!("Advent of Code 2022!\n");
    println!("Day 1:");
    days::day1::do_day_1();
}

pub fn read_input(day: i8) -> Vec<String> {
    let path = env::current_dir().unwrap();
    println!("The current directory is {}", path.display());
    let input = format!("src/input/day{}.txt", day);
    let file = File::open(input).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}