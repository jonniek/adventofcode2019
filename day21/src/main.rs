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

  let mut spring_bot = intcode::VM::new(&program);

  spring_bot.output_ascii();
  spring_bot.input_ascii("NOT B T\nNOT C J\nOR T J\nNOT A T\nOR T J\nAND D J\nWALK\n");

  let out = spring_bot.output_ascii();
  println!("{:?}", out.unwrap());
}


fn part2(input: &str) {
  let program: Vec<i64> = input.split(",").map(|s| s.parse().unwrap()).collect();

  let mut spring_bot = intcode::VM::new(&program);

  spring_bot.output_ascii();

  // A B C D E F G H I

  // ? ?   # ? ? ? # ?
  // If there is a hole coming and we can double jump do it
  spring_bot.input_ascii("NOT C T\nAND D T\nAND H T\nOR T J\n");

  // ?   ? # ? ? ? # ?
  // the hole is now closer, if we can double jump do it
  spring_bot.input_ascii("NOT B T\nAND D T\nAND H T\nOR T J\n");

  // the hole is on next step, just jump and hope for the best
  spring_bot.input_ascii("NOT A T\nOR T J\n");

  spring_bot.input_ascii("RUN\n");

  let out = spring_bot.output_ascii();
  println!("{:?}", out.unwrap());
}