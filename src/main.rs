mod parser;
use days::*;
use std::env;
use std::time::Instant;

mod days;

fn main() {
  let args: Vec<String> = env::args().collect();
  let day: usize = args[1].parse().unwrap();
  match day {
    1 => day01::Day01::run_day("inputs/day01.txt"),
    2 => day02::Day02::run_day("inputs/day02.txt"),
    _ => panic!("Unknown day"),
  }
}
