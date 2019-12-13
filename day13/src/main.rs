mod intcode;

use std::io::{self, Read};
use std::collections::{ HashMap };
use std::cmp;

fn main() {
  let mut input = String::new();
  io::stdin().read_to_string(&mut input).unwrap();

  part1(&input);
  part2(&input);
}

#[derive(PartialEq)]
enum Tile {
  Empty,
  Wall,
  Block,
  Paddle,
  Ball
}

fn int_to_tile(input: i64) -> Tile {
  match input {
    0 => Tile::Empty,
    1 => Tile::Wall,
    2 => Tile::Block,
    3 => Tile::Paddle,
    4 => Tile::Ball,
    _ => panic!("Unexpected tile id {}", input),
  }
}

fn part1(input: &str) {
  let program: Vec<i64> = input.split(",").map(|s| s.parse().unwrap()).collect();

  let mut vm = intcode::VM::new(&program);

  let mut tiles: HashMap<(i64, i64), Tile> = HashMap::new();

  loop {

    let x = intcode::read_output(vm.read_input(None));
    let y = intcode::read_output(vm.read_input(None));
    let tile_int = intcode::read_output(vm.read_input(None));

    if x.is_none() || y.is_none() || tile_int.is_none() {
      break;
    }

    let pos = (x.unwrap(), y.unwrap());
    let tile = int_to_tile(tile_int.unwrap());

    tiles.insert(pos, tile);
  }

  let blocks: u32 = tiles.iter().fold(0, |total, (_, tile)| {
    match tile {
      Tile::Block => total + 1,
      _ => total,
    }
  });

  println!("{}", blocks);
}

fn render(tiles: &HashMap<(i64, i64), Tile>) {
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
      let tile: &Tile = tiles.get(&(x as i64, y as i64)).unwrap_or(&Tile::Empty);
      match tile {
        Tile::Empty => print!(" "),
        Tile::Wall => print!("█"),
        Tile::Block => print!("▒"),
        Tile::Paddle => print!("-"),
        Tile::Ball => print!("⏺"),
      }
    }
    println!("");
  }
}

fn part2(input: &str) {

  let mut program: Vec<i64> = input.split(",").map(|s| s.parse().unwrap()).collect();
  program[0] = 2;

  let mut vm = intcode::VM::new(&program);

  let mut tiles: HashMap<(i64, i64), Tile> = HashMap::new();
  let mut score = 0;

  let mut next_input = 0;
  let mut paddle_x: i64 = 21;

  loop {

    let x = intcode::read_output(vm.read_input(Some(next_input)));
    let y = intcode::read_output(vm.read_input(Some(next_input)));
    let tile_int = intcode::read_output(vm.read_input(Some(next_input)));

    if x.is_none() || y.is_none() || tile_int.is_none() {
      break;
    }

    let pos = (x.unwrap(), y.unwrap());
    if pos == (-1, 0) {
      score = tile_int.unwrap();
    } else {
      tiles.insert(pos, int_to_tile(tile_int.unwrap()));
      let tile = int_to_tile(tile_int.unwrap());

      if tile == Tile::Paddle {
        paddle_x = pos.0;
      }

      if tile == Tile::Ball {
        if paddle_x < pos.0 {
          next_input = 1;
        } else if paddle_x > pos.0 {
          next_input = -1;
        } else {
          next_input = 0;
        }
      }
    }
  }

  render(&tiles);
  println!("Score: {}", score);
}