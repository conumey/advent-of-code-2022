use std::{fs::{File}, io::{BufReader, BufRead}};

pub mod days;

fn main() {
    println!("Advent of Code 2022!\n");
    days::day1::do_day_1();
    days::day2::do_day_2();
    days::day3::do_day_3();
    days::day4::do_day_4();
}

pub fn read_input(day: i8) -> Vec<String> {
    let input = format!("src/input/day{}.txt", day);
    let file = File::open(input).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}