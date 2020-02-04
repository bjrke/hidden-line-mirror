use crate::float::*;
use crate::vec2::Vector2;

#[derive(Clone, Copy)]
pub struct Matrix2 {
    pub x: Vector2,
    pub y: Vector2,
}

impl Matrix2 {
    pub fn new(x: Vector2, y: Vector2) -> Matrix2 {
        Matrix2 { x, y }
    }

    pub fn withX(&self, v: Vector2) -> Matrix2 {
        Matrix2::new(v, self.y)
    }
    pub fn withY(&self, v: Vector2) -> Matrix2 {
        Matrix2::new(self.x, v)
    }

    pub fn det2d(&self) -> Float {
        self.x.x * self.y.y - self.x.y * self.y.x
    }
}

impl std::fmt::Display for Matrix2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

type matrix2d = Matrix2;
