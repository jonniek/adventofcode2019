mod intcode;

use std::io::{self, Read};
use std::collections::{ HashMap, HashSet };
use std::cmp;

fn main() {
  let mut input = String::new();
  io::stdin().read_to_string(&mut input).unwrap();

  part1(&input);
  part2(&input);
}

#[derive(Debug)]
enum Direction {
  Up, Right, Down, Left,
}

#[derive(Debug)]
enum Turn {
  Left = 0,
  Right = 1,
}

fn get_turn_enum(value: i64) -> Turn {
  match value {
    0 => Turn::Left,
    1 => Turn::Right,
    _ => panic!("Unexpected turn value {:?}", value),
  }
}

fn next_direction(current: &Direction, turn: Turn) -> Direction {
  match (current, turn) {
    (Direction::Up, Turn::Left) => Direction::Left,
    (Direction::Up, Turn::Right) => Direction::Right,

    (Direction::Right, Turn::Left) => Direction::Up,
    (Direction::Right, Turn::Right) => Direction::Down,

    (Direction::Down, Turn::Left) => Direction::Right,
    (Direction::Down, Turn::Right) => Direction::Left,

    (Direction::Left, Turn::Left) => Direction::Down,
    (Direction::Left, Turn::Right) => Direction::Up,
  }
}

fn next_position(pos: (i64, i64), direction: &Direction) -> (i64, i64) {
  match direction {
    Direction::Up => (pos.0, pos.1 + 1),
    Direction::Right => (pos.0 + 1, pos.1),
    Direction::Down => (pos.0, pos.1 - 1),
    Direction::Left => (pos.0 - 1, pos.1),
  }
}

fn part1(input: &str) {
  let program: Vec<i64> = input.split(",").map(|s| s.parse().unwrap()).collect();

  let mut robot = intcode::VM::new(&program);

  let mut colors: HashMap<(i64, i64), i64> = HashMap::new();

  let mut position: (i64, i64) = (0, 0);
  let mut direction = Direction::Up;

  let mut colored_positions: HashSet<(i64, i64)> = HashSet::new();

  loop {
    let color = colors.entry(position).or_insert(0);
    let next_color = robot.read_input(Some(*color));

    match next_color {
      intcode::Output::Value(v) => {
        *color = v;
        colored_positions.insert(position);
      },
      intcode::Output::Halt => break,
      _ => panic!("Robot was waiting for input when we expected output"),
    };

    let turn = match robot.read_input(None) {
      intcode::Output::Value(v) => v,
      intcode::Output::Halt => break,
      _ => panic!("Robot was waiting for input when we expected output"),
    };

    let turn_enum = get_turn_enum(turn);
    direction = next_direction(&direction, turn_enum);
    position = next_position(position, &direction);
  }

  println!("{}", colored_positions.len());
}

fn part2(input: &str) {
  let program: Vec<i64> = input.split(",").map(|s| s.parse().unwrap()).collect();

  let mut robot = intcode::VM::new(&program);

  let mut colors: HashMap<(i64, i64), i64> = HashMap::new();
  colors.insert((0,0), 1);

  let mut position: (i64, i64) = (0, 0);
  let mut direction = Direction::Up;

  loop {
    let color = colors.entry(position).or_insert(0);
    let next_color = robot.read_input(Some(*color));

    match next_color {
      intcode::Output::Value(v) => {
        *color = v;
      },
      intcode::Output::Halt => break,
      _ => panic!("Robot was waiting for input when we expected output"),
    };

    let turn = match robot.read_input(None) {
      intcode::Output::Value(v) => v,
      intcode::Output::Halt => break,
      _ => panic!("Robot was waiting for input when we expected output"),
    };

    let turn_enum = get_turn_enum(turn);
    direction = next_direction(&direction, turn_enum);
    position = next_position(position, &direction);
  }

  let white_colors: Vec<(i64, i64)> = colors.iter().filter_map(|(pos, color)| {
    if *color == 1 {
      return Some(*pos);
    }
    None
  }).collect();

  let (min_x, max_x, min_y, max_y) = white_colors.iter().fold((0, 0, 0, 0), |total, pos| {
    (
      cmp::min(pos.0, total.0),
      cmp::max(pos.0, total.1),
      cmp::min(pos.1, total.2),
      cmp::max(pos.1, total.3),
    )
  });

  for y in (min_y..max_y + 1).rev() {
    for x in min_x..max_x + 1 {
      match colors.get(&(x as i64, y as i64)).unwrap_or(&0) {
        &1 => print!("█"),
        _ => print!(" "),
      };
    }
    println!("");
  }
}