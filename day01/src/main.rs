use std::io::{self, Read};

fn main() {
  let mut input = String::new();
  io::stdin().read_to_string(&mut input).unwrap();

  part1(&input);
  part2(&input);
}

fn part1(input: &str) {
  let mut mass: i32 = 0;
  for module in input.lines() {
    let module_mass: i32 = module.parse().unwrap_or(0);
    mass += required_fuel(module_mass);
  }
  println!("{}", mass);
}

fn part2(input: &str) {
  let mut mass: i32 = 0;
  for module in input.lines() {
    let module_mass: i32 = module.parse().unwrap_or(0);
    mass += fuel_loop(module_mass);
  }
  println!("{}", mass);
}

fn fuel_loop(module_mass: i32) -> i32 {
  let mut mass: i32 = 0;
  let mut next_mass = required_fuel(module_mass);
  while next_mass > 0 {
    mass += next_mass;
    next_mass = required_fuel(next_mass);
  }
  mass
}

fn required_fuel(module_mass: i32) -> i32 {
  module_mass / 3 - 2
}