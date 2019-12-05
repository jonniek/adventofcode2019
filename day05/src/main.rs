use std::io::{self, Read};

fn main() {
  let mut input = String::new();
  io::stdin().read_to_string(&mut input).unwrap();

  part1(&input);
  part2(&input);
}

fn part1(input: &str) {
  let program: Vec<i32> = input.split(",").map(|s| s.parse().unwrap()).collect();
  execute_program(program, 1);
}

fn part2(input: &str) {
  let program: Vec<i32> = input.split(",").map(|s| s.parse().unwrap()).collect();
  execute_program(program, 5);
}

const POSITION_MODE: i32 = 0;
const IMMIDIATE_MODE: i32 = 1;

// helper to get param value based on mode
fn get_param_value(program: &Vec<i32>, param_raw: i32, mode: i32) -> i32 {
  match mode {
    POSITION_MODE => program[param_raw as usize],
    IMMIDIATE_MODE => param_raw,
    _ => panic!("Invalid mode {}", mode),
  }
}

fn execute_program(mut program: Vec<i32>, input: i32) {
  let mut instruction_pointer: usize = 0;

  loop {
    let opcode = program[instruction_pointer] % 100;

    // early exit before accessing invalid indices
    if opcode == 99 {
      break;
    }

    let mode_1 = (program[instruction_pointer] / 100) % 10;
    let mode_2 = (program[instruction_pointer] / 1000) % 10;
    // mode_3 is not used by our opcodes
    // let mode_3 = (program[instruction_pointer] / 10000) % 10;

    // helper variables that access raw parameter value
    let param_1_raw = program[instruction_pointer + 1];
    let param_2_raw = program[instruction_pointer + 2];
    let param_3_raw = program[instruction_pointer + 3];

    match opcode {
      1 => {
        let param_1 = get_param_value(&program, param_1_raw, mode_1);
        let param_2 = get_param_value(&program, param_2_raw, mode_2);

        program[param_3_raw as usize] = param_1 + param_2;

        instruction_pointer += 4;
      },
      2 => {
        let param_1 = get_param_value(&program, param_1_raw, mode_1);
        let param_2 = get_param_value(&program, param_2_raw, mode_2);

        program[param_3_raw as usize] = param_1 * param_2;

        instruction_pointer += 4;
      },
      3 => {
        program[param_1_raw as usize] = input;

        instruction_pointer += 2;
      },
      4 => {
        let output = get_param_value(&program, param_1_raw, mode_1);

        println!("{:?}", output);

        instruction_pointer += 2;
      },
      5 => {
        let param_1 = get_param_value(&program, param_1_raw, mode_1);
        let param_2 = get_param_value(&program, param_2_raw, mode_2);

        if param_1 != 0 {
          instruction_pointer = param_2 as usize;
        } else {
          instruction_pointer += 3;
        }
      },
      6 => {
        let param_1 = get_param_value(&program, param_1_raw, mode_1);
        let param_2 = get_param_value(&program, param_2_raw, mode_2);

        if param_1 == 0 {
          instruction_pointer = param_2 as usize;
        } else {
          instruction_pointer += 3;
        }
      },
      7 => {
        let param_1 = get_param_value(&program, param_1_raw, mode_1);
        let param_2 = get_param_value(&program, param_2_raw, mode_2);

        program[param_3_raw as usize] = if param_1 < param_2 { 1 } else { 0 };

        instruction_pointer += 4;
      },
      8 => {
        let param_1 = get_param_value(&program, param_1_raw, mode_1);
        let param_2 = get_param_value(&program, param_2_raw, mode_2);

        program[param_3_raw as usize] = if param_1 == param_2 { 1 } else { 0 };

        instruction_pointer += 4;
      },
      _ => panic!("Invalid operation code {}", opcode)
    };
  }
}