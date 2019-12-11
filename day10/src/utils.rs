use std;
use std::hash::Hash;
use std::hash::Hasher;
use std::cmp::Ordering;

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct Coord {
  pub x: i32,
  pub y: i32,
}

#[derive(Debug)]
pub struct Angle {
  pub angle: f32,
}

fn f32_bits(a: f32) -> i32 { unsafe { std::mem::transmute(a) } }
fn cmp_total_order(a: f32, b: f32) -> std::cmp::Ordering {
  let mut a = f32_bits(a);
  let mut b = f32_bits(b);
  if a < 0 { a ^= 0x7fffffff; }
  if b < 0 { b ^= 0x7fffffff; }
  a.cmp(&b)
}

impl PartialEq for Angle {
  fn eq(&self, other: &Self) -> bool {
    self.angle == other.angle
  }
}

impl Ord for Angle {
  fn cmp(&self, other: &Self) -> Ordering {
    cmp_total_order(self.angle, other.angle)
  }
}

impl PartialOrd for Angle {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(cmp_total_order(self.angle, other.angle))
  }
}

impl Eq for Angle {}

impl Hash for Angle {
  fn hash<H>(&self, state: &mut H)
    where H: Hasher,
  {
    state.write_i32(self.angle as i32);
    state.finish();
  }
}