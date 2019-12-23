mod intcode;

use std::io::{self, Read};
use std::collections::{ HashSet };

fn main() {
  let mut input = String::new();
  io::stdin().read_to_string(&mut input).unwrap();

  part1(&input);
  part2(&input);
}

fn part1(input: &str) {
  let program: Vec<i64> = input.split(",").map(|s| s.parse().unwrap()).collect();

  let mut computers: Vec<intcode::VM> = vec!();
  let mut queues: Vec<Vec<i64>> = vec!();

  let mut outputs: Vec<Vec<i64>> = vec!();

  // initialize computers and queues
  for n in 0..50 {
    let mut computer = intcode::VM::new(&program);
    computer.read_input(Some(n));
    computers.push(computer);
    queues.push(Vec::new());
    outputs.push(Vec::new());
  }

  // start network
  loop {

    for (index, computer) in computers.iter_mut().enumerate() {

      if queues[index as usize].len() > 0 {
        let x = queues[index as usize].pop().unwrap();
        let y = queues[index as usize].pop().unwrap();
        // println!("Computer {} receive packet {} {}", index, x, y);
        match computer.read_input(Some(x)) {
          intcode::Output::Value(value) => {
            outputs[index as usize].insert(0, value);
          },
          _ => (),
        };
        match computer.read_input(Some(y)) {
          intcode::Output::Value(value) => {
            outputs[index as usize].insert(0, value);
          },
          _ => (),
        };
      }


      while outputs[index as usize].len() < 3 {
        let out = intcode::read_output(computer.read_input(Some(-1)));
        if out.is_none() {
          break;
        }
        outputs[index as usize].insert(0, out.unwrap());
      }

      if outputs[index as usize].len() > 2 {
        let packet = outputs[index as usize].pop().unwrap();
        let x = outputs[index as usize].pop().unwrap();
        let y = outputs[index as usize].pop().unwrap();

        if packet == 255 {
          println!("{}", y);
          return ();
        }

        // println!("Sending from {} to {} x {} y {}", index, packet, x, y);
        queues[packet as usize].insert(0, x);
        queues[packet as usize].insert(0, y);
      }
    }
  }
}

fn part2(input: &str) {
  let program: Vec<i64> = input.split(",").map(|s| s.parse().unwrap()).collect();

  let mut computers: Vec<intcode::VM> = vec!();
  let mut queues: Vec<Vec<i64>> = vec!();

  let mut nat_y: HashSet<i64> = HashSet::new();
  let mut nat: (i64, i64) = (0, 0);

  let mut outputs: Vec<Vec<i64>> = vec!();

  // initialize computers and queues
  for n in 0..50 {
    let mut computer = intcode::VM::new(&program);
    computer.read_input(Some(n));
    computers.push(computer);
    queues.push(Vec::new());
    outputs.push(Vec::new());
  }

  // start network
  loop {

    // check for network idle
    if nat.0 != 0 && nat.1 != 0 {
      let len = queues.iter().fold(0, |total, output_vec| total + output_vec.len());
      if len == 0 {

        if nat_y.contains(&nat.1) {
          println!("{}", nat.1);
          return ();
        }
        nat_y.insert(nat.1);

        queues[0].insert(0, nat.0);
        queues[0].insert(0, nat.1);
      }
    }

    for (index, computer) in computers.iter_mut().enumerate() {

      if queues[index as usize].len() > 0 {
        let x = queues[index as usize].pop().unwrap();
        let y = queues[index as usize].pop().unwrap();
        // println!("Computer {} receive packet {} {}", index, x, y);
        match computer.read_input(Some(x)) {
          intcode::Output::Value(value) => {
            outputs[index as usize].insert(0, value);
          },
          _ => (),
        };
        match computer.read_input(Some(y)) {
          intcode::Output::Value(value) => {
            outputs[index as usize].insert(0, value);
          },
          _ => (),
        };
      }


      while outputs[index as usize].len() < 3 {
        let out = intcode::read_output(computer.read_input(Some(-1)));
        if out.is_none() {
          break;
        }
        outputs[index as usize].insert(0, out.unwrap());
      }

      if outputs[index as usize].len() > 2 {
        let packet = outputs[index as usize].pop().unwrap();
        let x = outputs[index as usize].pop().unwrap();
        let y = outputs[index as usize].pop().unwrap();

        if packet == 255 {
          nat = (x, y);
        } else {

          // println!("Sending from {} to {} x {} y {}", index, packet, x, y);
          queues[packet as usize].insert(0, x);
          queues[packet as usize].insert(0, y);
        }

      }
    }
  }
}