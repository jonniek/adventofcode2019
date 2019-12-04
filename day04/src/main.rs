use std::io::{self, Read};
use std::collections::HashMap;
use std::cmp;

fn main() {
  let mut input = String::new();
  io::stdin().read_to_string(&mut input).unwrap();

  part1(&input);
  part2(&input);
}

fn part1(input: &str) {
  let mut numbers = input.split("-");

  let min = numbers.next().unwrap().parse::<i32>().unwrap();
  let max = numbers.next().unwrap().parse::<i32>().unwrap();

  let valid_count = (min..max + 1).fold(0, |count, password| {
    match valid_password_1(&password.to_string()) {
      true => count + 1,
      false => count
    }
  });

  println!("{:?}", valid_count);
}

fn part2(input: &str) {
  let mut numbers = input.split("-");

  let min = numbers.next().unwrap().parse::<i32>().unwrap();
  let max = numbers.next().unwrap().parse::<i32>().unwrap();

  let valid_count = (min..max + 1).fold(0, |count, password| {
    match valid_password_2(&password.to_string()) {
      true => count + 1,
      false => count
    }
  });

  println!("{:?}", valid_count);
}

fn valid_password_1(password: &str) -> bool {
  let mut had_adjacent = false;

  // assume password doesn't start with 0
  let mut previous_n = 0;

  let mut max: u32 = 0;

  for n in password.chars().map(|c| c.to_digit(10).unwrap()) {
    if n == previous_n {
      had_adjacent = true;
    }
    previous_n = n;

    if n < max {
      // No decreasing order allowed
      return false;
    }
    max = cmp::max(n, max);
  }

  had_adjacent
}


fn valid_password_2(password: &str) -> bool {
  let mut char_map: HashMap<u32, u32> = HashMap::new();

  let mut max: u32 = 0;

  for n in password.chars().map(|c| c.to_digit(10).unwrap()) {
    let count = char_map.entry(n).or_insert(0);
    *count += 1;

    if n < max {
      // No decreasing order allowed
      return false;
    }
    max = cmp::max(n, max);
  }

  match char_map.iter().find(|(_key, value)| **value == 2) {
    Some(_) => true,
    None => false,
  }
}