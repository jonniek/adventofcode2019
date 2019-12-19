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

  let mut sum = 0;

  for y in 0..50 {
    for x in 0..50 {
      let mut vm = intcode::VM::new(&program);
      vm.read_input(Some(x as i64));
      let output = vm.read_input(Some(y as i64));
      sum += intcode::read_output(output).unwrap();
    }
  }

  println!("{}", sum);
}

fn part2(input: &str) {
  let program: Vec<i64> = input.split(",").map(|s| s.parse().unwrap()).collect();

  let square_offset = 99;

  let mut y = 100;
  let mut x = 0;
  loop {
    y += 1;

    loop {
      if has_gravity(&program, (x, y)) {

        if has_gravity(&program, (x + square_offset, y - square_offset)) {
          println!("{}", (x * 10000) + y - square_offset);
          return ();
        }

        break;
      } else {
        x += 1;
      }
    }

  }
}

fn has_gravity(program: &Vec<i64>, pos: (usize, usize)) -> bool {
  let mut vm = intcode::VM::new(&program);
  vm.read_input(Some(pos.0 as i64));
  let output = vm.read_input(Some(pos.1 as i64));
  match output {
    intcode::Output::Value(value) => {
      return value == 1;
    },
    _ => panic!("Unexpected output value {:?}", output),
  };
}