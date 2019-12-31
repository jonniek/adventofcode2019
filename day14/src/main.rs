use std::io::{self, Read};
use std::collections::HashMap;
use std::mem;

fn main() {
  let mut input = String::new();
  io::stdin().read_to_string(&mut input).unwrap();

  part1(&input);
  part2(&input);
}

type Ingredients<'a> = HashMap<&'a str, i64>;

fn part1(input: &str) {
  println!("{}", compute_ore(&input, 1));
}

fn compute_ore(input: &str, multiplier: i64) -> i64 {

  let mut reaction_map: HashMap<&str, Ingredients> = HashMap::new();
  let mut reaction_count_map: HashMap<&str, i64> = HashMap::new();

  for line in input.lines() {
    let mut split_iter = line.split(" => ");
    let mut ingredients: Ingredients = HashMap::new();

    for pair in split_iter.next().unwrap().split(", ") {
      let mut i = pair.split(" ");
      let value: i64 = i.next().unwrap().parse().unwrap();
      let name = i.next().unwrap();
      ingredients.insert(name, value);
    }

    let mut result_iter = split_iter.next().unwrap().split(" ");
    let result_amount: i64 = result_iter.next().unwrap().parse().unwrap();
    let result_type = result_iter.next().unwrap();

    reaction_count_map.insert(result_type, result_amount);
    reaction_map.insert(result_type, ingredients);
  }

  // multiply initial ingriedients
  let mut ingredients = reaction_map.remove("FUEL").unwrap().clone();
  for (_, v) in ingredients.iter_mut() {
    *v *= multiplier;
  }

  loop {
    let mut next_ingredients: Ingredients = HashMap::new();

    for (name, count) in ingredients.iter() {
      // early exit for ORE since it doesn't have ingredients
      if *name == "ORE" {
        let v = next_ingredients.entry(name).or_insert(0);
        *v += count;
        continue;
      }

      // If we still have a dependency, continue
      if reaction_map.iter().find(|(_, ing)| {
        ing.get(name).is_some() || next_ingredients.get(name).is_some()
      }).is_some() {
        let v = next_ingredients.entry(name).or_insert(0);
        *v += count;
        continue;
      }

      let result_count = reaction_count_map.get(name).unwrap();
      // divide and round up
      let times = (count - 1) / result_count + 1;
      let mut next_ing = reaction_map.remove(name).unwrap().clone();

      for (_, count) in next_ing.iter_mut() {
        *count *= times;
      }

      for (n, count) in next_ing.iter() {
        let v = next_ingredients.entry(n).or_insert(0);
        *v += count;
      }
    }
    if next_ingredients.len() == 1 {
      return *next_ingredients.get("ORE").unwrap();
    }
    mem::swap(&mut ingredients, &mut next_ingredients);
  }
}


fn part2(input: &str) {
  let mut n = 1;
  let mut result = 0;
  // Poor mans binary search
  while result < 1000000000000 {
    n += 10000;
    result = compute_ore(&input, n);
  }
  while result > 1000000000000 {
    n -= 1;
    result = compute_ore(&input, n);
  }
  println!("{}", n);
}