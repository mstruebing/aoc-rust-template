mod parser;
use chrono::prelude::*;
use days::*;
use std::time::Instant;

use clap::{App, AppSettings, Arg, SubCommand};

mod days;

fn main() {
  let matches = App::new("Advent of Code template")
    .version("2021")
    .author("Rik van Toor <rik@rikvt.dev>")
    .about("A template for solving Advent of Code puzzles in Rust")
    .setting(AppSettings::SubcommandRequiredElseHelp)
    .subcommands(vec![SubCommand::with_name("run")
      .about("Execute one or multiple days")
      .arg(
        Arg::with_name("day")
          .help("The number of the day you want to run")
          .takes_value(true),
      )
      .arg(
        Arg::with_name("all")
          .short("a")
          .long("all")
          .help("Runs all days sequentially"),
      )])
    .get_matches();

  if let Some(matches) = matches.subcommand_matches("run") {
    if matches.is_present("all") {
      run_all_days();
    } else {
      match matches.value_of("day") {
        Some(day) => run_day(day),
        None => {
          println!("No day parameter specified, attempting to run today");
          let now = Local::now();
          let now_day = now.day();
          if now.month() == 12 && now_day >= 1 && now_day <= 25 {
            println!("Running day {}", now_day);
            run_day(&format!("{}", now_day));
          } else {
            println!("Today is not a valid Advent of Code day. Please specify a day");
          }
        }
      }
    }
  }
}

fn run_all_days() {
  (1..26).map(run_day_helper).collect()
}

fn run_day(day: &str) {
  match day.parse() {
    Ok(i) => {
      if (1..=25).contains(&i) {
        run_day_helper(i)
      } else {
        panic!("{} is not a valid day. Only days 1-25 are allowed.", i)
      }
    }
    Err(_) => {
      panic!("{} is not a valid day. Please provide a number.", day)
    }
  }
}

// Panics if you provide a value outside the range of 1 to 25
fn run_day_helper(day: usize) {
  println!("======== DAY {} ========", day);
  // I'd like to do this with a macro, but I'm not sure how to do it.
  let input_fp = &format!("inputs/day{:02}.txt", day);
  match day {
    1 => day01::Day01::run_day(input_fp),
    2 => day02::Day02::run_day(input_fp),
    3 => day03::Day03::run_day(input_fp),
    4 => day04::Day04::run_day(input_fp),
    5 => day05::Day05::run_day(input_fp),
    6 => day06::Day06::run_day(input_fp),
    7 => day07::Day07::run_day(input_fp),
    8 => day08::Day08::run_day(input_fp),
    9 => day09::Day09::run_day(input_fp),
    10 => day10::Day10::run_day(input_fp),
    11 => day11::Day11::run_day(input_fp),
    12 => day12::Day12::run_day(input_fp),
    13 => day13::Day13::run_day(input_fp),
    14 => day14::Day14::run_day(input_fp),
    15 => day15::Day15::run_day(input_fp),
    16 => day16::Day16::run_day(input_fp),
    17 => day17::Day17::run_day(input_fp),
    18 => day18::Day18::run_day(input_fp),
    19 => day19::Day19::run_day(input_fp),
    20 => day20::Day20::run_day(input_fp),
    21 => day21::Day21::run_day(input_fp),
    22 => day22::Day22::run_day(input_fp),
    23 => day23::Day23::run_day(input_fp),
    24 => day24::Day24::run_day(input_fp),
    25 => day25::Day25::run_day(input_fp),
    d => panic!("Provided unsupported day {}", d),
  }
}
