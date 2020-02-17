use crate::float::*;
use std::cmp::Ordering;

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vector2 {
  pub x: Float,
  pub y: Float,
}

pub type vector2d = Vector2;

impl Vector2 {
  pub fn new(x: Float, y: Float) -> Vector2 {
    Vector2 { x, y }
  }

  pub fn mul2d(&self, f: Float) -> Vector2 {
    Vector2::new(self.x * f, self.y * f)
  }

  pub fn div2d(&self, d: Float) -> Vector2 {
    Vector2::new(self.x / d, self.y / d)
  }

  pub fn sub2d(&self, v: &Vector2) -> Vector2 {
    Vector2::new(self.x - v.x, self.y - v.y)
  }

  pub fn add2d(&self, v: &Vector2) -> Vector2 {
    Vector2::new(self.x + v.x, self.y + v.y)
  }

  pub fn sqrbetrag2d(&self) -> Float {
    self.x.sqr() + self.y.sqr()
  }

  pub fn betrag2d(&self) -> Float {
    self.sqrbetrag2d().sqrt()
  }
}

impl Eq for Vector2 {}

impl Ord for Vector2 {
  fn cmp(&self, other: &Self) -> Ordering {
    self.partial_cmp(other).unwrap()
  }
}

impl std::fmt::Display for Vector2 {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "({}, {})", self.x, self.y)
  }
}

pub fn colinear(p1: &Vector2, p2: &Vector2, p3: &Vector2) -> bool {
  ((p1.y - p2.y) * (p3.x - p2.x) - (p1.x - p2.x) * (p3.y - p2.y)).abs() < epsilon2
}
