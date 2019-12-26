mod intcode;

use std::io::{self, Read, BufRead, BufReader};
use std::fs::File;
use std::io::prelude::*;
use std::collections::{ HashSet };

extern crate itertools;
use itertools::Itertools;


fn main() {
  let mut input = String::new();
  //io::stdin().read_to_string(&mut input).unwrap();

  let mut file = File::open("input/input.txt").unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();

  part1(&contents);
  part2(&contents);
}

fn part1(input: &str) {
  let program: Vec<i64> = input.split(",").map(|s| s.parse().unwrap()).collect();

  let mut vm = intcode::VM::new(&program);

  let items: Vec<&str> = vec!(
    "tambourine",
    "hologram",
    "fuel cell",
    "wreath",
    "boulder",
    "fixed point",
    "manifold",
    "polygon",
  );



  // manually find items and go to pressure-sensitive floor
  // give bruteforce command to try all combinations of items
  loop {
    let out = vm.output_ascii();
    if out.is_some() {
      println!("{}", out.unwrap());
      break;
    }
    let mut command = String::new();
    io::stdin().read_line(&mut command).expect("Failed to read line");


    if &command == "bruteforce\n" {
      let mut combination_count = 7;

      while combination_count > 1 {
        for next_items in items.iter().combinations(combination_count) {
          // drop all items
          for item in items.iter() {
            vm.input_ascii(&format!("drop {}\n", item));
            vm.output_ascii();
          }

          for item in next_items.iter() {
            vm.input_ascii(&format!("take {}\n", item));
            vm.output_ascii();
          }

          vm.input_ascii("inv\n");
          vm.output_ascii();

          vm.input_ascii("north\n");
          let out = vm.output_ascii();

          if out.is_some() {
            // correct combination of items:
            // boulder, fixed point, manifold, polygon
            println!("{}", out.unwrap());
            return ();
          }
        }
        combination_count -= 1;
      }
    }

    vm.input_ascii(&command);
  }
}


// TODO: need rest of the stars
fn part2(input: &str) {
}