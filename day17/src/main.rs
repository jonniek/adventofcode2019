mod intcode;

use std::io::{self, Read};
use std::collections::{ HashMap };
use std::cmp;
use std::char;

fn main() {
  let mut input = String::new();
  io::stdin().read_to_string(&mut input).unwrap();

  let map = part1(&input);
  part2(&input, map);
}

fn part1(input: &str) -> HashMap<(i64, i64), char> {
  let program: Vec<i64> = input.split(",").map(|s| s.parse().unwrap()).collect();

  let mut vm = intcode::VM::new(&program);

  let mut map: HashMap<(i64, i64), char> = HashMap::new();

  let mut y = 0;
  let mut x = 0;

  loop {
    let output = intcode::read_output(vm.read_input(None));

    if output.is_none() {
      break;
    }

    match output.unwrap() {
      10 => {
        y += 1;
        x = -1;
      },
      other => {
        let c: char = (other as u8).into();
        map.insert((x, y), c);
      },
    }
    x += 1;
  }

  let alignment_sum: i64 = map.clone().iter().fold(0, |total, (pos, ch)| {
    if ch == &'#' && is_crossroad(&map, pos) {
      let alignment_param = pos.0 * pos.1;
      return total + alignment_param;
    }
    return total;
  });

  render(&map);
  println!("{}", alignment_sum);

  map
}

fn is_crossroad(map: &HashMap<(i64, i64), char>, position: &(i64, i64)) -> bool {
  let road_char = &'#';

  let above = map.get(&(position.0, position.1 - 1)).unwrap_or(&'.');
  let below = map.get(&(position.0, position.1 + 1)).unwrap_or(&'.');
  let right = map.get(&(position.0 + 1, position.1)).unwrap_or(&'.');
  let left = map.get(&(position.0 - 1, position.1)).unwrap_or(&'.');

  above == road_char && below == road_char && right == road_char && left == road_char
}

fn render(tiles: &HashMap<(i64, i64), char>) {
  let (min_x, max_x, min_y, max_y) = tiles.iter().fold((0,0,0,0), |minmax, (pos, _)| {
    (
      cmp::min(minmax.0, pos.0),
      cmp::max(minmax.1, pos.0),
      cmp::min(minmax.2, pos.1),
      cmp::max(minmax.3, pos.1),
    )
  });

  for y in min_y..max_y+1 {
    for x in min_x..max_x+1 {
      let mut tile: &char = tiles.get(&(x as i64, y as i64)).unwrap_or(&' ');
      if is_crossroad(&tiles, &(x as i64, y as i64)) {
        tile = &'O';
      }
      print!("{}", tile);
    }
    println!("");
  }
}

#[repr(u8)] #[derive(Clone)]
enum Dir {
  North = 0,
  East = 1,
  South = 2,
  West = 3,
}

fn left(dir: &Dir) -> Dir {
  let val = ((dir.clone() as u8) + 3) % 4;
  let new_dir: Dir = unsafe { ::std::mem::transmute(val) };
  new_dir
}

fn right(dir: &Dir) -> Dir {
  let val = ((dir.clone() as u8) + 1) % 4;
  let new_dir: Dir = unsafe { ::std::mem::transmute(val) };
  new_dir
}

fn next_position(position: &(i64, i64), dir: &Dir) -> (i64, i64) {
  match dir {
    Dir::North => (position.0, position.1 - 1),
    Dir::East => (position.0 + 1, position.1),
    Dir::South => (position.0, position.1 + 1),
    Dir::West => (position.0 - 1, position.1),
  }
}

fn get_rotation(map: &HashMap<(i64, i64), char>, pos: &(i64, i64), dir: &Dir) -> Option<(char, Dir)> {
  let road_char = &'#';

  let left = left(&dir);
  let right = right(&dir);

  let left_pos = next_position(&pos, &left);
  let right_pos = next_position(&pos, &right);

  let left_char = map.get(&left_pos).unwrap_or(&'.');
  let right_char = map.get(&right_pos).unwrap_or(&'.');

  if left_char == road_char { return Some(('L', left)); }
  if right_char == road_char { return Some(('R', right)); }

  None
}

fn print_until_next_input(mut vm: intcode::VM) -> intcode::VM {
  let mut out = vm.read_input(None);
  loop {
    match out {
      intcode::Output::Value(v) => {
        let c: char = (v as u8).into();
        print!("{}", c);
      },
      _ => {
        break;
      },
    };
    out = vm.read_input(None);
  }

  vm
}

fn part2(input: &str, map: HashMap<(i64, i64), char>) {

  // harcode the starting position from observing part 1 rendering
  let mut position: (i64, i64) = (24, 26);
  // hardcode the first rotation to Left from observing part 1 rendering
  // This simplifies the rotating logic
  let mut trail: Vec<char> = vec!('L');
  let mut direction = Dir::West;

  let mut distance: u32 = 0;

  loop {
    let next_pos = next_position(&position, &direction);
    let next_char = map.get(&next_pos).unwrap_or(&'.');

    match next_char {
      &'#' => {
        distance += 1;
        position = next_pos;
      },
      &'.' => {
        let input = distance.to_string();

        for ch in input.chars() {
          trail.push(ch);
        }

        let rotation = get_rotation(&map, &position, &direction);

        match rotation {
          Some(values) => {
            trail.push(values.0);
            direction = values.1;
            distance = 0;
          },
          None => {
            break;
          }
        };
      },
      _ => panic!("Unexpected character {}", next_char),
    }
  }

  /* Output of our trail, copy paste to new file and look for repeating patterns
  for n in trail.iter() {
    print!("{}", n);
  }
  println!("");
  */

  // Result of repeating patterns
  let function_calls = "A,B,A,C,A,A,C,B,C,B\n";
  let A = "L,12,L,8,R,12\n";
  let B = "L,10,L,8,L,12,R,12\n";
  let C = "R,12,L,8,L,10\n";

  let mut program: Vec<i64> = input.split(",").map(|s| s.parse().unwrap()).collect();
  program[0] = 2;
  let mut vm = intcode::VM::new(&program);

  vm = print_until_next_input(vm);

  for ch in function_calls.chars() {
    vm.read_input(Some(ch as i64));
  }

  vm = print_until_next_input(vm);

  for ch in A.chars() {
    vm.read_input(Some(ch as i64));
  }

  vm = print_until_next_input(vm);

  for ch in B.chars() {
    vm.read_input(Some(ch as i64));
  }

  vm = print_until_next_input(vm);

  for ch in C.chars() {
    vm.read_input(Some(ch as i64));
  }

  vm = print_until_next_input(vm);

  vm.read_input(Some('n' as i64));
  // Start reading output after submitting the final newline
  let mut output = vm.read_input(Some(10));

  let mut prev: i64 = 0;
  loop {
    match output {
      intcode::Output::Value(v) => {
        // store previous output in variable
        prev = v;
      },
      intcode::Output::WaitingForInput => panic!("Shouldnt need input"),
      _ => {
        break;
      },
    };
    output = vm.read_input(None);
  }

  // print only the finally outputted value
  println!("{:?}", prev);
}