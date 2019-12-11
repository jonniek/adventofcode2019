use std::io::{self, Read};
use std::str;
use std::collections::HashMap;

fn main() {
  let mut input = String::new();
  io::stdin().read_to_string(&mut input).unwrap();

  part1(&input);
  part2(&input);
}

const HEIGHT: usize = 6;
const WIDTH: usize = 25;

const PIXELS: usize = 150;

const LAYER_LENGTH: usize = HEIGHT * WIDTH; 

fn get_layers(image: &str) -> Vec<&str> {
  image.as_bytes()
    .chunks(LAYER_LENGTH as usize)
    .map(str::from_utf8)
    .collect::<Result<Vec<&str>, _>>()
    .unwrap()
}

fn layer_digits(layer: &str) -> HashMap<char, i32> {
  let mut map = HashMap::new();
  for ch in layer.chars() {
    let mut count = map.entry(ch).or_insert(0);
    *count += 1;
  }
  map
}

fn part1(input: &str) {
  let layers = get_layers(input);

  let mut final_map: HashMap<char, i32> = HashMap::new();
  let mut fewest_zeros = i32::max_value();
  for layer in layers.iter() {
    let map = layer_digits(layer);
    let zero_count = map.get(&'0');

    let zero_count = match map.get(&'0') {
      Some(count) => count,
      None => &0,
    };

    if zero_count < &fewest_zeros {
      fewest_zeros = *zero_count;
      final_map = layer_digits(layer);
    }
  }

  let ones = match final_map.get(&'1') {
    Some(count) => count,
    None => &0,
  };
  let twos = match final_map.get(&'2') {
    Some(count) => count,
    None => &0,
  };
  println!("{:?}", ones * twos);
}

fn part2(input: &str) {
  let layers = get_layers(input);

  let mut message: [i32; PIXELS] = [2; PIXELS];
  
  for layer in layers.iter() {
    for (index, pixel) in layer.chars().enumerate() {
      match (pixel, message[index]) {
        ('0', 2) => {
          message[index] = 0;
        },
        ('1', 2) => {
          message[index] = 1;
        },
        _ => (),
      }
    }
  }

  let message_vec: Vec<i32> = message.iter().cloned().collect();
  for chunk in message_vec.chunks(WIDTH) {
    for value in chunk {
      let c = match value {
        1 => "◼",
        _ => " ",
      };
      print!("{}", c);
    }
    println!("");
  }
}
