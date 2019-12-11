use std::io::{self, Read};
use std::collections::HashSet;
use std::collections::BTreeMap;

// helper structs with some comparison, sort and hash implementations
mod utils;
use utils::{ Angle, Coord };

fn main() {
  let mut input = String::new();
  io::stdin().read_to_string(&mut input).unwrap();

  let base = part1(&input);
  part2(&input, base);
}

fn get_asteroids(input: &str) -> Vec<Coord> {
  let mut asteroids: Vec<Coord> = vec!();
  for (y, line) in input.lines().enumerate() {
    for (x, ch) in line.chars().enumerate() {
      if ch == '#' {
        asteroids.push(Coord { x: x as i32, y: y as i32 });
      }
    }
  }
  asteroids
}

fn azimuth_angle(a: &Coord, b: &Coord) -> f32 {
  // y: a.y - b.y is flipped here because our Y axis is flipped
  let vector = Coord { x: b.x - a.x, y: a.y - b.y };
  let theta = (vector.x as f32).atan2(vector.y as f32);
  let deg = theta.to_degrees();

  match deg < 0.0 {
    true => deg + 360.0,
    false => deg,
  }
}

fn calculate_visible_asteroids(from: &Coord, asteroids: &Vec<Coord>) -> usize {
  let mut blocked_angles: HashSet<Angle> = HashSet::new();

  for to in asteroids.iter() {
    if from == to {
      continue;
    }

    let angle = Angle { angle: azimuth_angle(from, to) };
    blocked_angles.insert(angle);
  }

  blocked_angles.len()
}

fn manhattan_distance(a: &Coord, b: &Coord) -> i32 {
  (a.x - b.x).abs() + (a.y - b.y).abs()
}

fn part1(input: &str) -> Coord {
  let asteroids = get_asteroids(input);

  let result: (Option<Coord>, usize) =
    asteroids
      .iter()
      .fold((None, 0), |state, asteroid| {
        let visible = calculate_visible_asteroids(asteroid, &asteroids);
        if visible > state.1 {
          return (Some(asteroid.clone()), visible);
        }
        state
      });

  let base = result.0.unwrap();
  let visible_asteroids = result.1;

  println!("{:?}", visible_asteroids);

  base
}

// base comes from part1
fn part2(input: &str, base: Coord) {
  let asteroids = get_asteroids(input);

  // use BTree to keep angles of asteroids in sorted order
  let mut line_of_sight_tree: BTreeMap<Angle, Vec<Coord>> = BTreeMap::new();

  // iterate and insert asteroids to our Btree
  for to in asteroids.iter() {
    if &base == to {
      continue;
    }

    let radian = azimuth_angle(&base, to);
    let angle = Angle { angle: radian };

    let mut list = line_of_sight_tree.entry(angle).or_insert(Vec::new());
    list.push(to.clone());
  }

  // sort our line of sight vectors by distance from base
  for (_, line_of_sight) in line_of_sight_tree.iter_mut() {
    line_of_sight.sort_by(|a, b| {
      let dis_a = manhattan_distance(&base, a);
      let dis_b = manhattan_distance(&base, b);
      dis_a.partial_cmp(&dis_b).unwrap()
    });
  }

  let mut destroyed_asteroids: HashSet<Coord> = HashSet::new();

  // keep looping our unique angles
  loop {
    for (_, line_of_sight) in line_of_sight_tree.iter() {

      // Find the closest asteroid in line of sight that is not destroyed
      let asteroid = line_of_sight.iter().find(|c| !destroyed_asteroids.contains(c));

      // destroy asteroid
      if asteroid.is_some() {
        let coord = asteroid.unwrap();
        destroyed_asteroids.insert(coord.clone());

        if destroyed_asteroids.len() == 200 {
          let x = (coord.x * 100) + coord.y;
          println!("{}", x);
          return ();
        }
      }
    }
  }
}
