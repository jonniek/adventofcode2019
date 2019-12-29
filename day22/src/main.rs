use std::io::{self, Read};

fn main() {
  let mut input = String::new();
  io::stdin().read_to_string(&mut input).unwrap();

  part1(&input);
  part2(&input);
}


fn cut(vec: Vec<usize>, n: isize) -> Vec<usize> {

  let cut_length: usize = if n < 0 { vec.len() - n.abs() as usize } else { n as usize };

  let head: Vec<usize> = vec.iter().cloned().take(cut_length).collect();
  let mut tail: Vec<usize> = vec.iter().cloned().skip(cut_length).collect();

  tail.extend(head);

  tail
}

fn deal_with_increment(vec: Vec<usize>, n: usize) -> Vec<usize> {
  let mut output = vec.clone();

  for (m, value) in (0..vec.len()).zip(vec.iter()) {
    let index = (m * n) % vec.len();
    output[index] = *value;
  }

  output
}

fn create_deck(cards: usize) -> Vec<usize> {
  let mut vec = vec!();

  for i in 0..cards {
    vec.push(i)
  }

  vec
}

fn deal_into_new_stack(vec: Vec<usize>) -> Vec<usize> {
  vec.into_iter().rev().collect()
}

fn part1(input: &str) {
  let mut cards = create_deck(10007);

  for command in input.lines() {

    if command.contains("cut") {
      let n: isize = command.split(" ").last().unwrap().parse().unwrap();
      cards = cut(cards, n);
    } else if command.contains("deal with increment") {
      let n: usize = command.split(" ").last().unwrap().parse().unwrap();
      cards = deal_with_increment(cards, n);
    } else if command.contains("deal into new stack") {
      cards = deal_into_new_stack(cards);
    }
  }

  for (index, card) in cards.iter().enumerate() {
    if *card == 2019 as usize {
      println!("{}", index);
      break;
    }
  }
}

fn modp(b: i128, exp: i128, base: i128) -> i128 {
  let mut x = 1;
  let mut p = b % base;

  for i in 0..128 {
    if 1 & (exp >> i) == 1 {
      x = x * p % base;
    }

    p = p * p % base;
  }

  x
}

fn modinv(mut a: i128, mut base: i128) -> i128 {
  if base == 1 {
    return 0;
  }

  let orig = base;

  let mut x = 1;
  let mut y = 0;

  while a > 1 {
    let q = a / base;
    let tmp = base;
    base = a % base;
    a = tmp;
    let tmp = y;
    y = x - q * y;
    x = tmp;
  }

  if x < 0 {
    x + orig
  } else {
    x
  }
}

fn part2(input: &str) {

  let repetitions: i128 = 101_741_582_076_661;
  let modulo: i128 = 119_315_717_514_047;

  let mut inc: i128 = 1;
  let mut offset: i128 = 0;

  for command in input.lines() {
    if command.contains("cut") {
      let n: i128 = command.split(" ").last().unwrap().parse().unwrap();
      offset = (offset + n * inc) % modulo;
    } else if command.contains("deal with increment") {
      let n: i128 = command.split(" ").last().unwrap().parse().unwrap();
      let inverse = modinv(n, modulo);
      inc = (inc * inverse) % modulo;
    } else if command.contains("deal into new stack") {
      inc = (inc * -1) % modulo;
      offset = (offset + inc) % modulo;
    }
  }

  let final_inc = modp(inc, repetitions, modulo);

  let geo_inv = modinv((1 - inc) % modulo, modulo);
  let final_offset = ((offset * (1 - final_inc) % modulo) * geo_inv) % modulo;

  let result = (final_offset + final_inc * 2020) % modulo;

  if result < 0 {
    println!("{}", result + modulo);
  } else {
    println!("{}", result);
  }
}