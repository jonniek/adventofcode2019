use std::io::{self, Read};
use std::collections::{HashSet,HashMap,BTreeMap};
use std::cmp;

#[macro_use]
extern crate lazy_static;

fn main() {
  let mut input = String::new();
  io::stdin().read_to_string(&mut input).unwrap();

  part1(&input);
  part2(&input);
}

type BugMap = HashMap<Pos, bool>;
type Pos = (i32, i32);

fn part1(input: &str) {

  let mut map: BugMap = HashMap::new();


  let mut seen_states: HashSet<String> = HashSet::new();

  for (y, line) in input.lines().enumerate() {
    for (x, ch) in line.chars().enumerate() {
      map.insert((x as i32, y as i32), ch == '#');
    }
  }

  loop {

    // check for seen state
    let map_hash = map_to_string(&map);
    if seen_states.contains(&map_hash) {

      let biodiversity = map_hash.chars().enumerate().fold(0, |sum, (n, ch)| {
        if ch == '#' {
          let base: i32 = 2;
          return sum + (base.pow(n as u32 + 1) / 2);
        }
        sum
      });
      println!("{}", biodiversity);
      break;
    }
    seen_states.insert(map_hash);

    // calculate next state
    let old_map = map.clone();
    for (pos, has_bug) in old_map.iter() {
      let neighbours = neighbours(pos, &old_map);
      if *has_bug {
        if neighbours == 1 {
          map.insert(*pos, true);
        } else {
          map.insert(*pos, false);
        }
      } else {
        if neighbours == 1 || neighbours == 2 {
          map.insert(*pos, true);
        }
      }
    }
  }
}

fn map_to_string(map: &BugMap) -> String {
  let (min_x, max_x, min_y, max_y) = map.iter().fold((0,0,0,0), |minmax, (pos, _)| {
    (
      cmp::min(minmax.0, pos.0),
      cmp::max(minmax.1, pos.0),
      cmp::min(minmax.2, pos.1),
      cmp::max(minmax.3, pos.1),
    )
  });

  let mut output: Vec<&str> = vec!();

  for y in min_y..max_y+1 {
    for x in min_x..max_x+1 {
      let has_bug = map.get(&(x as i32, y as i32)).unwrap_or(&false);
      if *has_bug {
        output.push("#");
      } else {
        output.push(".");
      }
    }
  }

  output.join("").to_string()
}

const DIRECTIONS: [Pos; 4] = [
  (0, 1), (1, 0),
  (0, -1), (-1, 0)
];

fn neighbours(
  pos: &Pos,
  map: &BugMap
) -> i32 {

  let mut sum = 0;

  for mult in DIRECTIONS.iter() {
    let next_pos = (pos.0 + mult.0, pos.1 + mult.1);

    let is_bug = map.get(&next_pos).unwrap_or(&false);
    if *is_bug {
      sum += 1;
    }
  }

  sum
}


fn part2(input: &str) {

  let mut levels: BTreeMap<i32, BugMap> = BTreeMap::new();

  let mut first_level: BugMap = HashMap::new();
  for (y, line) in input.lines().enumerate() {
    for (x, ch) in line.chars().enumerate() {
      if y != 2 || x != 2 {
        first_level.insert((x as i32, y as i32), ch == '#');
      }
    }
  }
  levels.insert(0, first_level);

  let minutes = 200;

  for n in 1..minutes + 1 {

    let mut empty_map_1: BugMap = HashMap::new();
    let mut empty_map_2: BugMap = HashMap::new();
    for y in 0..5 {
      for x in 0..5 {
        if y != 2 || x != 2 {
          empty_map_1.insert((x as i32, y as i32), false);
          empty_map_2.insert((x as i32, y as i32), false);
        }
      }
    }
    levels.insert(n, empty_map_1);
    levels.insert(n * -1, empty_map_2);


    levels = next_levels_state(levels);
  }

  let total_bugs = levels.iter().fold(0, |total, (_, map)| {
    let inner_bugs = map.iter().fold(0, |inner_total, (_, bug)| {
      if *bug {
        return inner_total + 1;
      }
      inner_total
    });

    total + inner_bugs
  });

  println!("{}", total_bugs);
}

fn next_levels_state(levels: BTreeMap<i32, BugMap>) -> BTreeMap<i32, BugMap> {
  let mut new_map = BTreeMap::new();

  for (level_index, level_map) in levels.iter() {

    let mut new_level = level_map.clone();
    let inner_levels = levels.get(&(level_index + 1));
    let outer_levels = levels.get(&(level_index - 1));

    for (pos, has_bug) in level_map.iter() {
      let neighbours = neighbours_nested(pos, level_map, inner_levels, outer_levels);
      if *has_bug {
        if neighbours == 1 {
          new_level.insert(*pos, true);
        } else {
          new_level.insert(*pos, false);
        }
      } else {
        if neighbours == 1 || neighbours == 2 {
          new_level.insert(*pos, true);
        }
      }
    }
    new_map.insert(*level_index, new_level);
  }

  new_map
}

lazy_static!{
  static ref outer_positions: HashMap<Pos, Pos> = {
    let mut m = HashMap::new();
    m.insert((0, -1), (2, 1));
    m.insert((0, 1), (2, 3));
    m.insert((1, 0), (3, 2));
    m.insert((-1, 0), (1, 2));
    m
  };

  static ref inner_positions: HashMap<Pos, Vec<Pos>> = {
    let mut m = HashMap::new();
    m.insert((2, 1), vec!((0, 0), (1, 0), (2, 0), (3, 0), (4, 0)));
    m.insert((2, 3), vec!((0, 4), (1, 4), (2, 4), (3, 4), (4, 4)));
    m.insert((3, 2), vec!((4, 0), (4, 1), (4, 2), (4, 3), (4, 4)));
    m.insert((1, 2), vec!((0, 0), (0, 1), (0, 2), (0, 3), (0, 4)));
    m
  };
}


fn neighbours_nested(
  pos: &Pos,
  map: &BugMap,
  inner: Option<&BugMap>,
  outer: Option<&BugMap>,
) -> i32 {

  let mut sum = 0;

  for mult in DIRECTIONS.iter() {
    let next_pos = (pos.0 + mult.0, pos.1 + mult.1);

    let mut is_bug = false;

    let inner_option = inner_positions.get(&pos);
    if inner_option.is_some() && next_pos == (2, 2) {
      for adj in inner_option.unwrap().iter() {
        match inner {
          Some(map) => {
            let had_bug = map.get(adj).unwrap_or(&false);
            if *had_bug {
              sum += 1;
            }
          },
          None => (),
        }
      }
    } else if next_pos.0 < 0 || next_pos.0 > 4 || next_pos.1 < 0 || next_pos.1 > 4{
      let outer_pos = outer_positions.get(mult).unwrap();
      match outer {
        Some(map) => {
          is_bug = *map.get(outer_pos).unwrap_or(&false);
        },
        None => {
          is_bug = false;
        }
      }
    } else {
      is_bug = *map.get(&next_pos).unwrap_or(&false);
    }
    if is_bug {
      sum += 1;
    }
  }

  sum
}