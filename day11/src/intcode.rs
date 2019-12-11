pub struct VM {
  program: Vec<i64>,
  pointer: usize,
  relative_base: i64,
}

// not sure how big this should be? 2^16 for now
const MIN_PROGRAM_SIZE: usize = 65536;

const POSITION_MODE: i64 = 0;
const IMMIDIATE_MODE: i64 = 1;
const RELATIVE_MODE: i64 = 2;

#[derive(PartialEq, Debug)]
pub enum Output {
  WaitingForInput,
  Value(i64),
  Halt
}

pub fn read_output(out: Output) -> Option<i64> {
  match out {
    Output::Value(value) => Some(value),
    _ => None,
  }
}

impl VM {

  // initialize vm
  pub fn new(program: &Vec<i64>) -> VM {
    let mut fixed_length_program = vec![0; MIN_PROGRAM_SIZE];
    for (index, instruction) in program.iter().enumerate() {
      fixed_length_program[index] = *instruction;
    };

    VM {
      program: fixed_length_program,
      pointer: 0,
      relative_base: 0,
    }
  }

  // helper to get param value based on mode
  fn get_read_param_value(&self, param_raw: i64, mode: i64) -> i64 {
    match mode {
      POSITION_MODE => self.program[param_raw as usize],
      IMMIDIATE_MODE => param_raw,
      RELATIVE_MODE => {
        let index = self.relative_base + param_raw;
        self.program[index as usize]
      },
      _ => panic!("Invalid mode {}", mode),
    }
  }

  // helper to get param value based on mode
  fn get_write_param_value(&self, param_raw: i64, mode: i64) -> i64 {
    match mode {
      POSITION_MODE => param_raw,
      RELATIVE_MODE => param_raw + self.relative_base,
      _ => panic!("Invalid mode {}", mode),
    }
  }

  // reads an input and executes until returning Some(output)
  // or halting and returning None
  pub fn read_input(&mut self, mut input: Option<i64>) -> Output {
    loop {
      let opcode = self.program[self.pointer] % 100;

      // early exit before accessing invalid indices
      if opcode == 99 {
        return Output::Halt;
      }

      let mode_1 = (self.program[self.pointer] / 100) % 10;
      let mode_2 = (self.program[self.pointer] / 1000) % 10;
      let mode_3 = (self.program[self.pointer] / 10000) % 10;

      // println!("exe opcode {} mode_1 {} mode_2 {} mode_3 {}", opcode, mode_1, mode_2, mode_3);
      match opcode {
        1 => {
          let param_1_raw = self.program[self.pointer + 1];
          let param_2_raw = self.program[self.pointer + 2];
          let param_3_raw = self.program[self.pointer + 3];

          let param_1 = self.get_read_param_value(param_1_raw, mode_1);
          let param_2 = self.get_read_param_value(param_2_raw, mode_2);
          let param_3 = self.get_write_param_value(param_3_raw, mode_3);

          self.program[param_3 as usize] = param_1 + param_2;

          self.pointer += 4;
        },
        2 => {
          let param_1_raw = self.program[self.pointer + 1];
          let param_2_raw = self.program[self.pointer + 2];
          let param_3_raw = self.program[self.pointer + 3];

          let param_1 = self.get_read_param_value(param_1_raw, mode_1);
          let param_2 = self.get_read_param_value(param_2_raw, mode_2);
          let param_3 = self.get_write_param_value(param_3_raw, mode_3);

          self.program[param_3 as usize] = param_1 * param_2;

          self.pointer += 4;
        },
        3 => {
          let param_1_raw = self.program[self.pointer + 1];
          let param_1 = self.get_write_param_value(param_1_raw, mode_1);

          if input.is_none() {
            return Output::WaitingForInput;
          } else {
            self.program[param_1 as usize] = input.unwrap();
            self.pointer += 2;
            // Use input only once
            input = None;
          }
        },
        4 => {
          let param_1_raw = self.program[self.pointer + 1];

          let output = self.get_read_param_value(param_1_raw, mode_1);
          self.pointer += 2;
          return Output::Value(output);
        },
        5 => {
          let param_1_raw = self.program[self.pointer + 1];
          let param_2_raw = self.program[self.pointer + 2];

          let param_1 = self.get_read_param_value(param_1_raw, mode_1);
          let param_2 = self.get_read_param_value(param_2_raw, mode_2);

          if param_1 != 0 {
            self.pointer = param_2 as usize;
          } else {
            self.pointer += 3;
          }
        },
        6 => {
          let param_1_raw = self.program[self.pointer + 1];
          let param_2_raw = self.program[self.pointer + 2];

          let param_1 = self.get_read_param_value(param_1_raw, mode_1);
          let param_2 = self.get_read_param_value(param_2_raw, mode_2);

          if param_1 == 0 {
            self.pointer = param_2 as usize;
          } else {
            self.pointer += 3;
          }
        },
        7 => {
          let param_1_raw = self.program[self.pointer + 1];
          let param_2_raw = self.program[self.pointer + 2];
          let param_3_raw = self.program[self.pointer + 3];

          let param_1 = self.get_read_param_value(param_1_raw, mode_1);
          let param_2 = self.get_read_param_value(param_2_raw, mode_2);
          let param_3 = self.get_write_param_value(param_3_raw, mode_3);

          self.program[param_3 as usize] = if param_1 < param_2 { 1 } else { 0 };

          self.pointer += 4;
        },
        8 => {
          let param_1_raw = self.program[self.pointer + 1];
          let param_2_raw = self.program[self.pointer + 2];
          let param_3_raw = self.program[self.pointer + 3];

          let param_1 = self.get_read_param_value(param_1_raw, mode_1);
          let param_2 = self.get_read_param_value(param_2_raw, mode_2);
          let param_3 = self.get_write_param_value(param_3_raw, mode_3);

          self.program[param_3 as usize] = if param_1 == param_2 { 1 } else { 0 };

          self.pointer += 4;
        },
        9 => {
          let param_1_raw = self.program[self.pointer + 1];
          let param_1 = self.get_read_param_value(param_1_raw, mode_1);

          self.relative_base += param_1;
          self.pointer += 2;
        },
        _ => panic!("Invalid operation code {}", opcode)
      };
    }
  }
}