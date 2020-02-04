use crate::float::*;

#[derive(Clone, Copy)]
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

impl std::fmt::Display for Vector2 {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "({}, {})", self.x, self.y)
  }
}
