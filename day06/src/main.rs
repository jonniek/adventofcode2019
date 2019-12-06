use std::io::{self, Read};
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
  let mut input = String::new();
  io::stdin().read_to_string(&mut input).unwrap();

  part1(&input);
  part2(&input);
}

fn parse_orbits(input: &str) -> Vec<(&str, &str)> {
  return input.lines().map(|line| {
    let mut orbit = line.split(")");
    (orbit.next().unwrap(), orbit.next().unwrap())
  }).collect();
}

fn part1(input: &str) {
  let orbits = parse_orbits(input);

  let mut orbit_map: HashMap<&str, HashSet<&str>> = HashMap::new();

  for (a, b) in orbits.iter() {
    let mut empty_set: HashSet<&str> = HashSet::new();
    let mut set = orbit_map.entry(a).or_insert(empty_set);
    set.insert(b);
  }

  let orbit_count: u32 = orbit_count(&orbit_map, "COM", 0);

  println!("{:?}", orbit_count);
}

fn orbit_count(orbit_map: &HashMap<&str, HashSet<&str>>, orbit: &str, count: u32) -> u32 {
  let set_get = orbit_map.get(orbit);

  if set_get.is_none() {
    return count;
  }

  let set = set_get.unwrap();

  let mut inner_orbit_count = 0;
  for inner_orbit in set.iter() {
    inner_orbit_count += orbit_count(orbit_map, inner_orbit, count + 1);
  }

  return count + inner_orbit_count;
}


fn part2(input: &str) {
  let orbits = parse_orbits(input);

  let mut orbit_map: HashMap<&str, &str> = HashMap::new();

  // Map the orbits in reverse compared to part 1
  // So we can traverse the path YOU and SAN take down to COM
  for (a, b) in orbits.iter() {
    orbit_map.insert(b, a);
  }

  let mut you_orbits: HashSet<&str> = HashSet::new();
  let mut orbit = "YOU";
  while orbit != "COM" {
    orbit = orbit_map.get(orbit).unwrap();
    you_orbits.insert(orbit);
  }

  let mut santa_orbits: HashSet<&str> = HashSet::new();
  let mut orbit = "SAN";
  while orbit != "COM" {
    orbit = orbit_map.get(orbit).unwrap();
    santa_orbits.insert(orbit);
  }

  let you_path: HashSet<_> = you_orbits.difference(&santa_orbits).collect();
  let santa_path: HashSet<_> = santa_orbits.difference(&you_orbits).collect();

  let sum = you_path.len() + santa_path.len();
  println!("{:?}", sum);
}
