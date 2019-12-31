use std::io::{self, Read};
use std::collections::{HashMap, HashSet, VecDeque};
use std::mem;

fn main() {
  let mut input = String::new();
  io::stdin().read_to_string(&mut input).unwrap();

  part1(&input);
  part2(&input);
}

type Position = (usize, usize);

fn create_maze(input: &str) -> (
  HashMap<Position, char>,
  HashSet<char>,
  Position,
) {
  let mut maze: HashMap<Position, char> = HashMap::new();
  let mut start = (0, 0);
  let mut doors = HashSet::new();

  for (y, line) in input.lines().enumerate() {
    for (x, ch) in line.chars().enumerate() {
      maze.insert((x, y), ch);
      if ch == '@' {
        start = (x, y);
      }
      if ch.is_alphabetic() {
        doors.insert(ch.to_ascii_uppercase());
      }
    }
  }
  (
    maze,
    doors,
    start
  )
}

fn part1(input: &str) {

  let (maze, doors, start) = create_maze(&input);

  let shortest = shortest_path(
    &maze,
    &doors,
    start
  );
  println!("{:?}", shortest);
}

fn shortest_path(
  tiles: &HashMap<Position, char>,
  doors: &HashSet<char>,
  origin: Position,
) -> Option<usize> {

  let mut queue: VecDeque<(
    Position,
    usize,
    String
  )> = VecDeque::new();

  queue.push_back((origin, 0, String::new()));

  let mut visited: HashSet<(Position, String)> = HashSet::new();
  visited.insert((origin, String::new()));

  let adjacent: Vec<(isize, isize)> = vec!((0, 1), (0, -1), (1, 0), (-1, 0));

  while let Some((pos, distance, keys)) = queue.pop_front() {

    if keys.len() == doors.len() {
      return Some(distance);
    }

    for adj in adjacent.iter() {
      let adj_pos = ((pos.0 as isize + adj.0) as usize, (pos.1 as isize + adj.1) as usize);
      if !visited.contains(&(adj_pos, keys.clone())) {
        let tile = tiles.get(&adj_pos).unwrap_or(&'#');
        visited.insert((adj_pos, keys.clone()));

        if tile == &'.' || tile == &'@' {
          queue.push_back((adj_pos, distance + 1, keys.clone()));
        } else if tile.is_alphabetic() {
          let tile_upper = tile.to_ascii_uppercase();
          // is key
          if tile_upper != *tile {
            let mut new_keys: Vec<String> = keys.split("").map(|s| s.to_string()).collect();
            new_keys.push(tile_upper.to_string());
            new_keys.sort();
            new_keys.dedup();
            let new_key = new_keys.join("").to_string();
            queue.push_back((adj_pos, distance + 1, new_key));
          // is door, for unkown door pass through (part 2)
          } else if keys.contains(tile_upper) || !doors.contains(&tile_upper) {
            queue.push_back((adj_pos, distance + 1, keys.clone()));
          }
        }
      }
    }
  }

  None
}


fn part2(input: &str) {
  let (mut maze, doors, start) = create_maze(&input);

  // modify maze
  maze.insert((start.0, start.1), '#');
  maze.insert((start.0 + 1, start.1), '#');
  maze.insert((start.0 - 1, start.1), '#');
  maze.insert((start.0, start.1 + 1), '#');
  maze.insert((start.0, start.1 - 1), '#');

  // defined start position for corners
  let starts: Vec<Position> = vec!(
    (start.0 - 1, start.1 - 1),
    (start.0 + 1, start.1 - 1),
    (start.0 - 1, start.1 + 1),
    (start.0 + 1, start.1 + 1),
  );

  let mut sum = 0;
  for start in starts {
    // find all possible keys
    let keys = find_keys(
      &maze,
      start.clone()
    );
    // use part 1 solver with possible keys
    // this will ignore doors that you cannot open
    let shortest = shortest_path(
      &maze,
      &keys,
      start
    );
    sum += shortest.unwrap();
  }

  println!("{:?}", sum);
}

fn find_keys(
  tiles: &HashMap<Position, char>,
  origin: Position,
) -> HashSet<char> {

  let mut keys = HashSet::new();

  let mut queue: VecDeque<Position> = VecDeque::new();

  queue.push_back(origin);

  let mut visited: HashSet<Position> = HashSet::new();
  visited.insert(origin);

  let adjacent: Vec<(isize, isize)> = vec!((0, 1), (0, -1), (1, 0), (-1, 0));

  while let Some(pos) = queue.pop_front() {

    for adj in adjacent.iter() {
      let adj_pos = ((pos.0 as isize + adj.0) as usize, (pos.1 as isize + adj.1) as usize);
      if !visited.contains(&adj_pos) {
        let tile = tiles.get(&adj_pos).unwrap_or(&'#');
        visited.insert(adj_pos);

        if tile == &'.' || tile == &'@' {
          queue.push_back(adj_pos);
        } else if tile.is_alphabetic() {
          queue.push_back(adj_pos);
          let tile_upper = tile.to_ascii_uppercase();
          // is lowercase
          if tile_upper != *tile {
            keys.insert(tile_upper);
          }
        }
      }
    }
  }

  keys
}
