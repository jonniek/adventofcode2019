use std::io::{self, Read};
use std::collections::HashSet;

extern crate itertools;
use itertools::Itertools;
use std::mem;

fn main() {
  let mut input = String::new();
  io::stdin().read_to_string(&mut input).unwrap();

  part1(&input);
  part2(&input);
}

fn part1(input: &str) {
  let values: Vec<i32> = input.split("").filter_map(|v| {
    match v.parse() {
      Ok(s) => Some(s),
      Err(_) => None,
    }
  }).collect();

  let mut input_signal = values;
  for index in 0..100 {
    input_signal = calculate_next_signal(input_signal);
  }
  let head: Vec<String> = input_signal.into_iter().take(8).map(|i| i.to_string()).collect();
  let result = head.join("");
  println!("{}", result);
}

fn calculate_next_signal(input_signal: Vec<i32>) -> Vec<i32> {
  let mut next_signal = vec!();

  let pattern = [0, 1, 0, -1];
  let length = input_signal.len();

  for index in 0..length {

    let mut iter = pattern.iter().cycle();

    let mut sum = 0;
    let mut repeat = 1;

    let mut multiplier = iter.next().unwrap();

    for val in input_signal.iter()  {
      if repeat > index {
        multiplier = iter.next().unwrap();
        repeat = 0;
      }
      sum += val * multiplier;
      repeat += 1;
    }

    next_signal.push(sum.abs() % 10);
  }

  next_signal
}

fn part2(input: &str) {
  let values: Vec<i32> = input.split("").filter_map(|v| {
    match v.parse() {
      Ok(s) => Some(s),
      Err(_) => None,
    }
  }).collect();

  let offset: usize = input[0..7].parse().unwrap();
  let full_input: Vec<i32> = values.iter().cloned().cycle().take(values.len() * 10000).skip(offset).collect();

  // verify length is less than offset so that we can multiply
  // always by 1 rather than varying multipliers
  assert!(full_input.len() < offset);

  let mut prev_signal = full_input.clone();
  let mut next_signal = full_input;
  for n in 0..100 {
    let mut sum = 0;
    // reverse, since every value only depends on the sum after it
    // this way we can do n iterations instead of n^2
    for (index, value) in prev_signal.iter().enumerate().rev() {
      sum += *value;
      next_signal[index] = sum.abs() % 10;
    }
    // assigning next to prev is not allowed with rust ownership model
    // so swap the memory, after all loops the newest value is the prev_signal
    mem::swap(&mut prev_signal, &mut next_signal);
  }

  let head: Vec<String> = prev_signal.into_iter().take(8).map(|i| i.to_string()).collect();
  let result = head.join("");
  println!("{}", result);
}
