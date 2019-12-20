mod intcode;

use std::io::{self, Read};
use std::collections::{HashMap,VecDeque,HashSet};

fn main() {
  let mut input = String::new();
  io::stdin().read_to_string(&mut input).unwrap();

  part1_2(&input);
}

#[derive(PartialEq)]
enum Tile {
  Wall,
  Empty,
  Oxygen,
  Unknown,
}

#[repr(u8)]
#[derive(Clone)]
#[allow(dead_code)]
enum Dir {
  North = 0,
  West = 1,
  South = 2,
  East = 3,
}

fn turn(dir: &Dir, amount: u8) -> Dir {
  let current_dir = dir.clone();
  let current = current_dir as u8;
  let next_dir: Dir = unsafe { ::std::mem::transmute((current + amount) % 4) };
  next_dir
}

type Position = (i64 , i64);

fn next_position(pos: &Position, dir: &Dir) -> Position {
  match dir {
    Dir::North => (pos.0, pos.1 + 1),
    Dir::South => (pos.0, pos.1 - 1),
    Dir::West => (pos.0 - 1, pos.1),
    Dir::East => (pos.0 + 1, pos.1),
  }
}

fn dir_to_input(dir: &Dir) -> i64 {
  match *dir {
    Dir::North => 1,
    Dir::South => 2,
    Dir::West => 3,
    Dir::East => 4,
  }
}

fn part1_2(input: &str) {
  let program: Vec<i64> = input.split(",").map(|s| s.parse().unwrap()).collect();

  let mut robot = intcode::VM::new(&program);

  let mut map: HashMap<Position, Tile> = HashMap::new();

  let mut position: Position = (0, 0);

  map.insert(position.clone(), Tile::Empty);

  let turn_order: [u8; 4] = [1, 0, 3, 2];

  let mut current_dir: Dir = Dir::North;

  let mut oxygen: Position = (0, 0);

  loop {

    for turn_amount in turn_order.iter() {
      let new_dir = turn(&current_dir, *turn_amount);
      let input_dir = dir_to_input(&new_dir);
      let output = intcode::read_output(robot.read_input(Some(input_dir))).unwrap();
      match output {
        0 => {
          let wall_position = next_position(&position, &new_dir);
          map.entry(wall_position.clone()).or_insert(Tile::Wall);
        },
        1 => {
          map.entry(position.clone()).or_insert(Tile::Empty);
          position = next_position(&position, &new_dir);
          current_dir = new_dir;
          break;
        },
        2 => {
          map.entry(position.clone()).or_insert(Tile::Oxygen);
          oxygen = position.clone();
          position = next_position(&position, &new_dir);
          current_dir = new_dir;
          break;
        },
        _ => panic!("Unexpected output {:?}", output),
      };
    }

    if position == (0, 0) {
      break;
    }
  }

  // part 1
  let shortest = shortest_path_to_oxygen(&map, (0, 0));
  println!("{:?}", shortest.unwrap());

  // part 2
  let longest = longest_path_from_oxygen(&map, oxygen);
  println!("{}", longest);
}

fn shortest_path_to_oxygen(tiles: &HashMap<Position, Tile>, origin: Position) -> Option<usize> {
  let mut queue: VecDeque<(Position, usize)> = VecDeque::new();
  queue.push_back((origin, 1));

  let mut visited: HashSet<Position> = HashSet::new();
  visited.insert(origin);

  let adjacent: Vec<(i64, i64)> = vec!((0, 1), (0, -1), (1, 0), (-1, 0));

  while let Some((pos, distance)) = queue.pop_front() {
    for adj in adjacent.iter() {
      let adj_pos = (pos.0 + adj.0, pos.1 + adj.1);
      if !visited.contains(&adj_pos) {
        let tile = tiles.get(&adj_pos).unwrap_or(&Tile::Unknown);
        visited.insert(adj_pos);

        if *tile == Tile::Oxygen {
          return Some(distance + 1);
        }

        if *tile == Tile::Empty {
          queue.push_back((adj_pos, distance + 1));
        }
      }
    }
  }

  None
}

fn longest_path_from_oxygen(tiles: &HashMap<Position, Tile>, origin: Position) -> usize {
  let mut queue: VecDeque<(Position, usize)> = VecDeque::new();
  queue.push_back((origin, 1));

  let mut visited: HashSet<Position> = HashSet::new();
  visited.insert(origin);

  let adjacent: Vec<(i64, i64)> = vec!((0, 1), (0, -1), (1, 0), (-1, 0));

  let mut last_distance = 0;

  while let Some((pos, distance)) = queue.pop_front() {
    last_distance = distance;
    for adj in adjacent.iter() {
      let adj_pos = (pos.0 + adj.0, pos.1 + adj.1);
      if !visited.contains(&adj_pos) {
        let tile = tiles.get(&adj_pos).unwrap_or(&Tile::Unknown);
        visited.insert(adj_pos);

        if *tile == Tile::Empty {
          queue.push_back((adj_pos, distance + 1));
        }
      }
    }
  }

  last_distance
}
