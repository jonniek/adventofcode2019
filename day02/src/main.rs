use std::io::{self, Read};

fn main() {
  let mut input = String::new();
  io::stdin().read_to_string(&mut input).unwrap();

  part1(&input);
  part2(&input);
}

fn part1(input: &str) {
  let program: Vec<i32> = input.split(",").map(|s| s.parse().unwrap()).collect();

  let result = execute_program(program, 12, 2);

  println!("{:?}", result);
}


fn part2(input: &str) {
  let program: Vec<i32> = input.split(",").map(|s| s.parse().unwrap()).collect();

  for noun in 0..100 {
    for verb in 0..100 {
      let result = execute_program(program.clone(), noun, verb);
      if result == 19690720 {
        println!("{}", 100 * noun + verb);
        return
      }
    }
  }
}

fn execute_program(mut program: Vec<i32>, noun: i32, verb: i32) -> i32 {
  program[1] = noun;
  program[2] = verb;

  let mut offset: usize = 0;
  let mut operation = program[offset];

  while operation != 99 {

    let values: (i32, i32) = (
      program[program[offset + 1 as usize] as usize],
      program[program[offset + 2 as usize] as usize]
    );

    let value = match operation {
      1 => values.0 + values.1,
      2 => values.0 * values.1,
      _ => panic!("Invalid operation code")
    };

    let operation_index = program[offset + 3] as usize;
    program[operation_index] = value;

    offset += 4;
    operation = program[offset];
  }

  program[0]
}