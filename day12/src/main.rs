use std::io::{self, Read};
use std::collections::HashSet;

extern crate regex;
use regex::Regex;

fn main() {
  let mut input = String::new();
  io::stdin().read_to_string(&mut input).unwrap();

  part1(&input);
  part2(&input);
}

#[derive(Debug, Clone)]
struct Point {
  x: i32,
  y: i32,
  z: i32,
}

#[derive(Debug, Clone)]
struct Moon {
  pos: Point,
  vel: Point,
}

fn get_moons(input: &str) -> Vec<Moon> {
  let mut moons = vec!();

  let re = Regex::new(r"x=(-?\d+).*y=(-?\d+).*z=(-?\d+)").unwrap();

  for cap in re.captures_iter(input) {

    let pos = Point {
      x: cap[1].parse().unwrap(),
      y: cap[2].parse().unwrap(),
      z: cap[3].parse().unwrap(),
    };

    let vel = Point { x: 0, y: 0, z: 0 };

    moons.push(Moon { pos, vel });
  }

  moons
}

fn vel_comparison(n_1: i32, n_2: i32) -> i32 {
  if n_1 < n_2 { return 1; }
  if n_1 > n_2 { return -1; }
  return 0;
}

fn calculate_velocities(moons: &Vec<Moon>) -> Vec<Moon> {
  let mut new_moons = moons.clone();

  for (i, moon_1) in moons.iter().enumerate() {
    for (j, moon_2) in moons.iter().enumerate().skip(i + 1) {

      let x = vel_comparison(moon_1.pos.x, moon_2.pos.x);
      let y = vel_comparison(moon_1.pos.y, moon_2.pos.y);
      let z = vel_comparison(moon_1.pos.z, moon_2.pos.z);

      let moon_1_vel = Point { x, y, z };
      let moon_2_vel = Point { x: x * -1, y: y * -1, z: z * -1 };

      new_moons[i].vel.x += moon_1_vel.x;
      new_moons[i].vel.y += moon_1_vel.y;
      new_moons[i].vel.z += moon_1_vel.z;

      new_moons[j].vel.x += moon_2_vel.x;
      new_moons[j].vel.y += moon_2_vel.y;
      new_moons[j].vel.z += moon_2_vel.z;
    }
  }

  new_moons
}

fn calculate_positions(moons: &Vec<Moon>) -> Vec<Moon> {
   moons.iter().cloned().map(|mut moon| {
    moon.pos.x += moon.vel.x;
    moon.pos.y += moon.vel.y;
    moon.pos.z += moon.vel.z;
    moon
  }).collect()
}

fn total_energy(moons: &Vec<Moon>) -> i32 {
  moons.iter().fold(0, |total, moon| {
    let potential = moon.pos.x.abs() +  moon.pos.y.abs() + moon.pos.z.abs();
    let kinetic = moon.vel.x.abs() +  moon.vel.y.abs() + moon.vel.z.abs();

    total + (potential * kinetic)
  })
}

fn part1(input: &str) {
  let mut moons = get_moons(input);

  let mut steps = 0;
  while steps < 1000 {

    let new_vel_moons = calculate_velocities(&moons);
    let new_pos_moons = calculate_positions(&new_vel_moons);

    moons = new_pos_moons;
    steps = steps + 1;
  }

  let energy = total_energy(&moons);
  println!("{}", energy);
}


fn part2(input: &str) {

  let mut x_period: Option<i32> = None;
  let mut y_period: Option<i32> = None;
  let mut z_period: Option<i32> = None;

  let mut x_states: HashSet<(i32,i32,  i32,i32,  i32,i32,  i32,i32)> = HashSet::new();
  let mut y_states: HashSet<(i32,i32,  i32,i32,  i32,i32,  i32,i32)> = HashSet::new();
  let mut z_states: HashSet<(i32,i32,  i32,i32,  i32,i32,  i32,i32)> = HashSet::new();

  let mut period = 0;

  let mut moons = get_moons(input);
  loop {
    let new_vel_moons = calculate_velocities(&moons);
    let new_pos_moons = calculate_positions(&new_vel_moons);
    moons = new_pos_moons;

    let x_state = (
      moons[0].pos.x, moons[0].vel.x,
      moons[1].pos.x, moons[1].vel.x,
      moons[2].pos.x, moons[2].vel.x,
      moons[3].pos.x, moons[3].vel.x
    );

    let y_state = (
      moons[0].pos.y, moons[0].vel.y,
      moons[1].pos.y, moons[1].vel.y,
      moons[2].pos.y, moons[2].vel.y,
      moons[3].pos.y, moons[3].vel.y
    );

    let z_state = (
      moons[0].pos.z, moons[0].vel.z,
      moons[1].pos.z, moons[1].vel.z,
      moons[2].pos.z, moons[2].vel.z,
      moons[3].pos.z, moons[3].vel.z
    );

    if x_states.contains(&x_state) && x_period.is_none() {
      x_period = Some(period);
    }

    if y_states.contains(&y_state) && y_period.is_none() {
      y_period = Some(period);
    }

    if z_states.contains(&z_state) && z_period.is_none() {
      z_period = Some(period);
    }

    if x_period.is_some() && y_period.is_some() && z_period.is_some() {
      break;
    }

    x_states.insert(x_state);
    y_states.insert(y_state);
    z_states.insert(z_state);

    period += 1;
  }

  // link that calculates least common multiple
  println!(
    "https://www.wolframalpha.com/input/?i=lcm%28{}%2C{}%2C{}%29",
    x_period.unwrap(),
    y_period.unwrap(),
    z_period.unwrap()
  );
}
