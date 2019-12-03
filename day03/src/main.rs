use std::io::{self, Read};
use std::collections::HashSet;
use std::collections::HashMap;
use std::cmp;

fn main() {
  let mut input = String::new();
  io::stdin().read_to_string(&mut input).unwrap();

  part1and2(&input);
}

fn token_split(s: &str) -> (&str, &str) {
  match s.chars().next() {
    Some(c) => s.split_at(c.len_utf8()),
    None => s.split_at(0),
  }
}

fn y_multiplier(direction: &str) -> i32 {
  match direction {
    "U" => 1,
    "D" => -1,
    _ => 0
  }
}

fn x_multiplier(direction: &str) -> i32 {
  match direction {
    "R" => 1,
    "L" => -1,
    _ => 0
  }
}

const CENTER: (i32, i32) = (0, 0);

fn manhattan_distance_from_center(a: (i32, i32)) -> i32 {
  manhattan_distance(CENTER, a)
}

fn manhattan_distance(a: (i32, i32), b: (i32, i32)) -> i32 {
  (a.0 - b.0).abs() + (a.1 - b.1).abs()
}


fn line_visits(line: &str) -> HashMap<(i32, i32), i32> {
  let mut visits: HashMap<(i32, i32), i32> = HashMap::new();
  let mut position: (i32, i32) = CENTER;

  // distance from intial position
  let mut distance = 0;

  for command in line.split(",") {

    let (direction, range_str) = token_split(command);
    let range: i32 = range_str.parse().unwrap();

    for _ in 0..range {
      distance += 1;

      position.0 += x_multiplier(direction);
      position.1 += y_multiplier(direction);

      visits.entry(position).or_insert(distance);
    }
  }

  visits
}

fn part1and2(input: &str) {
  let mut lines = input.lines();

  // common computations
  let first_visits = line_visits(lines.next().unwrap());
  let second_visits = line_visits(lines.next().unwrap());

  let first_set: HashSet<(i32, i32)> = first_visits.iter().map(|(k, _)| *k).collect();
  let second_set: HashSet<(i32, i32)> = second_visits.iter().map(|(k, _)| *k).collect();

  let intersection: HashSet<&(i32, i32)> = first_set.intersection(&second_set).collect();

  // part 1
  let smallest_distance: i32 =
    intersection
      .iter()
      .fold(i32::max_value(), |min, pos| {
        let dis = manhattan_distance_from_center(**pos);
        cmp::min(dis, min)
      });

  println!("{:?}", smallest_distance);

  // part 2
  let shortest_lines: i32 =
    intersection
      .iter()
      .fold(i32::max_value(), |min, pos| {
        let dis = first_visits.get(*pos).unwrap() + second_visits.get(*pos).unwrap();
        cmp::min(dis, min)
      });

  println!("{:?}", shortest_lines);
}
