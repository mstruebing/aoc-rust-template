use nom::character::complete::{i32, newline};
use nom::multi::separated_list0;
use nom::IResult;

use crate::days::Day;

pub struct Day01;

impl Day for Day01 {
  type Input = Vec<i32>;

  fn parse(input: &str) -> IResult<&str, Self::Input> {
    separated_list0(newline, i32)(input)
  }

  type Output1 = i32;

  fn part_1(input: &Self::Input) -> Self::Output1 {
    for i in 0..input.len() {
      for j in i + 1..input.len() {
        if input[i] + input[j] == 2020 {
          return input[i] * input[j];
        }
      }
    }
    panic!("No answer found")
  }

  type Output2 = i32;

  // This is about 10x faster than using itertools' combinations
  fn part_2(input: &Self::Input) -> Self::Output2 {
    for i in 0..input.len() {
      for j in i + 1..input.len() {
        for k in j + 1..input.len() {
          if input[i] + input[j] + input[k] == 2020 {
            return input[i] * input[j] * input[k];
          }
        }
      }
    }
    panic!("No answer found")
  }
}
