mod intcode;

use std::io::{self, Read};

fn main() {
  let mut input = String::new();
  io::stdin().read_to_string(&mut input).unwrap();

  part1(&input);
  part2(&input);
}

fn part1(input: &str) {
  let program: Vec<i64> = input.split(",").map(|s| s.parse().unwrap()).collect();

  let mut vm = intcode::VM::new(&program);

  let input = 1;

  let mut output = vm.read_input(Some(input));
  loop {
    match output {
      intcode::Output::Halt => break,
      intcode::Output::Value(value) => println!("{}", value),
      _ => (),
    };

    output = vm.read_input(None);
  }
}

fn part2(input: &str) {
  let program: Vec<i64> = input.split(",").map(|s| s.parse().unwrap()).collect();

  let mut vm = intcode::VM::new(&program);

  let input = 2;

  let mut output = vm.read_input(Some(input));
  loop {
    match output {
      intcode::Output::Halt => break,
      intcode::Output::Value(value) => println!("{}", value),
      _ => (),
    };

    output = vm.read_input(None);
  }
}