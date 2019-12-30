use std::io::{self, Read};
use std::collections::{HashSet,HashMap,VecDeque};
use std::cmp;

fn main() {
  let mut input = String::new();
  io::stdin().read_to_string(&mut input).unwrap();

  part1_2(&input);
}

#[derive(Debug, PartialEq)]
enum Type {
  Wall,
  Empty,
}

fn is_letter(ch: char) -> bool {
  ch != '#' && ch != ' ' && ch != '.'
}

type Pos = (usize, usize);

fn get_portals(input: &str) -> (HashMap<Pos, Pos>, Pos, Pos) {

  let mut portals: HashMap<Pos, Pos> = HashMap::new();

  let mut partial_portals: HashMap<String, Pos> = HashMap::new();

  let vec_lines: Vec<&str> = input.lines().collect();
  let vec_chars: Vec<Vec<char>> = vec_lines.iter().map(|line| line.chars().collect()).collect();

  let mut used_positions: HashSet<Pos> = HashSet::new();
  let mut used_names: HashSet<String> = HashSet::new();

  let mut start = (0, 0);
  let mut end = (0, 0);

  for (y, line) in vec_chars.iter().enumerate() {
    for (x, ch) in line.iter().enumerate() {
      let position = (x, y);
      if !is_letter(*ch) || used_positions.contains(&position) {
        continue;
      }

      let below = vec_chars[y+1][x];
      let right = vec_chars[y][x+1];

      let (portal_name, portal_pos) = match (is_letter(right), is_letter(below)) {
        (true, _) => {
          used_positions.insert(position);
          used_positions.insert((x + 1, y));

          let name = format!("{}{}", ch, right);
          let safe_x= if x == 0 { 0 } else { x-1 };
          let pos = if vec_chars[y][safe_x] == '.' { (x-1, y) } else { (x+2, y) };
          (name, pos)
        },
        (_, true) => {
          used_positions.insert(position);
          used_positions.insert((x, y + 1));

          let name = format!("{}{}", ch, below);
          let safe_y = if y == 0 { 0 } else { y-1 };
          let pos = if vec_chars[safe_y][x] == '.' { (x, y-1) } else { (x, y+2) };
          (name, pos)
        },
        (_, _) => panic!("No second letter found for {}", ch),
      };

      if portal_name == "ZZ" {
        end = portal_pos;
      }
      if portal_name == "AA" {
        start = portal_pos;
      }

      if used_names.contains(&portal_name) {
        let p = partial_portals.get(&portal_name).unwrap();
        portals.insert(portal_pos, *p);
        portals.insert(*p, portal_pos);
      } else {
        used_names.insert(portal_name.clone());
        partial_portals.insert(portal_name, portal_pos);
      }

    }
  }

  (portals, start, end)
}

fn part1_2(input: &str) {

  let (portals, start, end) = get_portals(&input);

  let mut map: HashMap<(usize, usize), Type> = HashMap::new();

  let mut max_y = 0;
  let mut max_x = 0;
  for (y, line) in input.lines().enumerate() {
    max_y = cmp::max(max_y, y);
    for (x, ch) in line.chars().enumerate() {
      max_x = cmp::max(max_x, x);
      match ch {
        '.' => {
          map.insert((x, y), Type::Empty);
        },
        _ => {
          map.insert((x, y), Type::Wall);
        }
      };
    }
  }

  let shortest = shortest_path(&portals, &map, start.clone(), end.clone());
  println!("{:?}", shortest.unwrap());

  let shortest2 = shortest_recursive_path(&portals, &map, start, end, max_x, max_y);
  println!("{:?}", shortest2.unwrap());
}


fn shortest_path(portals: &HashMap<Pos, Pos>, tiles: &HashMap<Pos, Type>, origin: Pos, end: Pos) -> Option<usize> {
  let mut queue: VecDeque<(Pos, usize)> = VecDeque::new();
  queue.push_back((origin, 0));

  let mut visited: HashSet<Pos> = HashSet::new();
  visited.insert(origin);

  let adjacent: Vec<(isize, isize)> = vec!((0, 1), (0, -1), (1, 0), (-1, 0));

  while let Some((pos, distance)) = queue.pop_front() {

    let portal_option = portals.get(&pos);
    if portal_option.is_some() {
      let p = portal_option.unwrap();
      queue.push_back((*p, distance + 1));
      visited.insert(*p);
    }

    for adj in adjacent.iter() {
      let adj_pos = ((pos.0 as isize + adj.0) as usize, (pos.1 as isize + adj.1) as usize);
      if !visited.contains(&adj_pos) {
        let tile = tiles.get(&adj_pos).unwrap_or(&Type::Wall);
        visited.insert(adj_pos);

        if adj_pos == end {
          return Some(distance + 1);
        }

        if *tile == Type::Empty {
          queue.push_back((adj_pos, distance + 1));
        }
      }
    }
  }

  None
}

fn shortest_recursive_path(
  portals: &HashMap<Pos, Pos>,
  tiles: &HashMap<Pos, Type>,
  origin: Pos,
  end: Pos,
  max_x: usize,
  max_y: usize,
) -> Option<usize> {

  let mut queue: VecDeque<(Pos, usize, usize)> = VecDeque::new();
  queue.push_back((origin, 0, 0));

  let mut visited: HashSet<(Pos, usize)> = HashSet::new();
  visited.insert((origin, 0));

  let adjacent: Vec<(isize, isize)> = vec!((0, 1), (0, -1), (1, 0), (-1, 0));

  while let Some((pos, distance, level)) = queue.pop_front() {

    let portal_option = portals.get(&pos);
    if portal_option.is_some() {
      let p = portal_option.unwrap();
      // Inner portal
      if pos.0 > 4 && pos.0 < max_x - 4 && pos.1 > 4 && pos.1 < max_y - 4 {
        if level < portals.len() && !visited.contains(&(*p, level + 1)) {
          queue.push_back((*p, distance + 1, level + 1));
          visited.insert((*p, level + 1));
        }
      // Outer portal
      } else if level > 0 && !visited.contains(&(*p, level - 1)) {
        queue.push_back((*p, distance + 1, level - 1));
        visited.insert((*p, level - 1));
      }
    }

    for adj in adjacent.iter() {
      let adj_pos = ((pos.0 as isize + adj.0) as usize, (pos.1 as isize + adj.1) as usize);
      if !visited.contains(&(adj_pos, level)) {
        let tile = tiles.get(&adj_pos).unwrap_or(&Type::Wall);
        visited.insert((adj_pos, level));

        if adj_pos == end && level == 0 {
          return Some(distance + 1);
        }

        if *tile == Type::Empty {
          queue.push_back((adj_pos, distance + 1, level));
        }
      }
    }
  }

  None
}