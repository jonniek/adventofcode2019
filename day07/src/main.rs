use std::io::{self, Read};
use std::collections::HashSet;

mod intcode;

fn main() {
  let mut input = String::new();
  io::stdin().read_to_string(&mut input).unwrap();

  part1(&input);
  part2(&input);
}

fn permutations(values: [i64; 5]) -> Vec<[i64; 5]> {
  let mut perms = vec!();

  let sequence_digits: HashSet<i64> = values.iter().cloned().collect();
  for n_1 in sequence_digits.iter() {
    let used: HashSet<i64> = [*n_1].iter().cloned().collect();
    for n_2 in sequence_digits.difference(&used) {
      let used: HashSet<i64> = [*n_1, *n_2].iter().cloned().collect();
      for n_3 in sequence_digits.difference(&used) {
        let used: HashSet<i64> = [*n_1, *n_2, *n_3].iter().cloned().collect();
        for n_4 in sequence_digits.difference(&used) {
          let used: HashSet<i64> = [*n_1, *n_2, *n_3, *n_4].iter().cloned().collect();
          let n_5 = sequence_digits.difference(&used).next().unwrap();
          let seq = [*n_1, *n_2, *n_3, *n_4, *n_5];
          perms.push(seq);
        }
      }
    }
  }

  perms
}

fn part1(input: &str) {
  let program: Vec<i64> = input.split(",").map(|s| s.parse().unwrap()).collect();

  let permutations = permutations([0, 1, 2, 3, 4]);
  let max = permutations.iter().map(|p| compute_sequence(&program, *p)).max();

  println!("{}", max.unwrap());
}

fn compute_sequence(program: &Vec<i64>, sequence: [i64; 5]) -> i64 {
  let mut vm_1 = intcode::VM::new(&program);
  let mut vm_2 = intcode::VM::new(&program);
  let mut vm_3 = intcode::VM::new(&program);
  let mut vm_4 = intcode::VM::new(&program);
  let mut vm_5 = intcode::VM::new(&program);

  vm_1.read_input(Some(sequence[0]));
  vm_2.read_input(Some(sequence[1]));
  vm_3.read_input(Some(sequence[2]));
  vm_4.read_input(Some(sequence[3]));
  vm_5.read_input(Some(sequence[4]));

  let out_1 = vm_1.read_input(Some(0));
  let out_2 = vm_2.read_input(intcode::read_output(out_1));
  let out_3 = vm_3.read_input(intcode::read_output(out_2));
  let out_4 = vm_4.read_input(intcode::read_output(out_3));
  let out_5 = vm_5.read_input(intcode::read_output(out_4));

  intcode::read_output(out_5).unwrap()
}

fn compute_sequence_loop(program: &Vec<i64>, sequence: [i64; 5]) -> i64 {
  let mut vm_1 = intcode::VM::new(&program);
  let mut vm_2 = intcode::VM::new(&program);
  let mut vm_3 = intcode::VM::new(&program);
  let mut vm_4 = intcode::VM::new(&program);
  let mut vm_5 = intcode::VM::new(&program);

  vm_1.read_input(Some(sequence[0]));
  vm_2.read_input(Some(sequence[1]));
  vm_3.read_input(Some(sequence[2]));
  vm_4.read_input(Some(sequence[3]));
  vm_5.read_input(Some(sequence[4]));

  let mut input = Some(0);
  loop {
    let out_1 = vm_1.read_input(input);
    let out_2 = vm_2.read_input(intcode::read_output(out_1));
    let out_3 = vm_3.read_input(intcode::read_output(out_2));
    let out_4 = vm_4.read_input(intcode::read_output(out_3));
    let out_5 = vm_5.read_input(intcode::read_output(out_4));
    match intcode::read_output(out_5) {
      Some(v) => {
        input = Some(v)
      },
      None => {
        break;
      },
    };
  }

  input.unwrap()
}


fn part2(input: &str) {
  let program: Vec<i64> = input.split(",").map(|s| s.parse().unwrap()).collect();

  let permutations = permutations([5, 6, 7, 8, 9]);
  let max = permutations.iter().map(|p| compute_sequence_loop(&program, *p)).max();
  
  println!("{}", max.unwrap());
}