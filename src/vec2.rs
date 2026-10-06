use crate::float::*;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
pub struct Vector2 {
    pub x: Float,
    pub y: Float,
}

impl Vector2 {
    #[inline]
    pub const fn new(x: Float, y: Float) -> Vector2 {
        Vector2 { x, y }
    }

    #[inline]
    pub fn len(&self) -> Float {
        self.len_sq().sqrt()
    }

    #[inline]
    pub fn len_sq(&self) -> Float {
        self * self
    }

    #[inline]
    pub fn mix(&self, other: &Self, t: Float) -> Vector2 {
        (self * t) + (other * (1.0 - t))
    }
}

impl From<(Float, Float)> for Vector2 {
    #[inline]
    fn from((x, y): (Float, Float)) -> Vector2 {
        Vector2::new(x, y)
    }
}

impl From<Vector2> for (Float, Float) {
    #[inline]
    fn from(v: Vector2) -> (Float, Float) {
        (v.x, v.y)
    }
}

impl Add for Vector2 {
    type Output = Vector2;

    #[inline]
    fn add(self, rhs: Vector2) -> Vector2 {
        Vector2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Add for &Vector2 {
    type Output = Vector2;

    #[inline]
    fn add(self, rhs: &Vector2) -> Vector2 {
        Vector2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign for Vector2 {
    #[inline]
    fn add_assign(&mut self, rhs: Vector2) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Vector2 {
    type Output = Vector2;

    #[inline]
    fn sub(self, rhs: Vector2) -> Vector2 {
        Vector2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Sub for &Vector2 {
    type Output = Vector2;

    #[inline]
    fn sub(self, rhs: &Vector2) -> Vector2 {
        Vector2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl SubAssign for Vector2 {
    #[inline]
    fn sub_assign(&mut self, rhs: Vector2) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Mul for Vector2 {
    type Output = Float;

    #[inline]
    fn mul(self, rhs: Vector2) -> Float {
        self.x * rhs.x + self.y * rhs.y
    }
}

impl Mul for &Vector2 {
    type Output = Float;

    #[inline]
    fn mul(self, rhs: &Vector2) -> Float {
        self.x * rhs.x + self.y * rhs.y
    }
}

impl Mul<Float> for Vector2 {
    type Output = Vector2;

    #[inline]
    fn mul(self, rhs: Float) -> Vector2 {
        Vector2::new(self.x * rhs, self.y * rhs)
    }
}

impl Mul<Float> for &Vector2 {
    type Output = Vector2;

    #[inline]
    fn mul(self, rhs: Float) -> Vector2 {
        Vector2::new(self.x * rhs, self.y * rhs)
    }
}

impl MulAssign<Float> for Vector2 {
    #[inline]
    fn mul_assign(&mut self, rhs: Float) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl Div<Float> for Vector2 {
    type Output = Vector2;

    #[inline]
    fn div(self, rhs: Float) -> Vector2 {
        Vector2::new(self.x / rhs, self.y / rhs)
    }
}

impl Div<Float> for &Vector2 {
    type Output = Vector2;

    #[inline]
    fn div(self, rhs: Float) -> Vector2 {
        Vector2::new(self.x / rhs, self.y / rhs)
    }
}

impl DivAssign<Float> for Vector2 {
    #[inline]
    fn div_assign(&mut self, rhs: Float) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl Neg for Vector2 {
    type Output = Vector2;

    #[inline]
    fn neg(self) -> Vector2 {
        Vector2::new(-self.x, -self.y)
    }
}

impl Neg for &Vector2 {
    type Output = Vector2;

    #[inline]
    fn neg(self) -> Vector2 {
        Vector2::new(-self.x, -self.y)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn len_sq() {
        assert_eq!(Vector2::new(3.0, 4.0).len_sq(), 25.0);
    }

    #[test]
    fn len() {
        assert_eq!(Vector2::new(3.0, 4.0).len(), 5.0);
    }

    #[test]
    fn mix_midpoint() {
        assert_eq!(
            Vector2::new(0.0, 0.0).mix(&Vector2::new(10.0, 20.0), 0.5),
            Vector2::new(5.0, 10.0)
        );
    }

    #[test]
    fn mix_at_start() {
        assert_eq!(
            Vector2::new(0.0, 0.0).mix(&Vector2::new(10.0, 20.0), 0.0),
            Vector2::new(10.0, 20.0)
        );
    }

    #[test]
    fn mix_at_end() {
        assert_eq!(
            Vector2::new(0.0, 0.0).mix(&Vector2::new(10.0, 20.0), 1.0),
            Vector2::new(0.0, 0.0)
        );
    }

    #[test]
    fn add() {
        assert_eq!(
            Vector2::new(1.0, 2.0) + Vector2::new(3.0, 4.0),
            Vector2::new(4.0, 6.0)
        );
    }

    #[test]
    fn add_ref() {
        assert_eq!(
            &Vector2::new(1.0, 2.0) + &Vector2::new(3.0, 4.0),
            Vector2::new(4.0, 6.0)
        );
    }

    #[test]
    fn sub() {
        assert_eq!(
            Vector2::new(1.0, 2.0) - Vector2::new(3.0, 5.0),
            Vector2::new(-2.0, -3.0)
        );
    }

    #[test]
    fn sub_ref() {
        assert_eq!(
            &Vector2::new(1.0, 2.0) - &Vector2::new(3.0, 5.0),
            Vector2::new(-2.0, -3.0)
        );
    }

    #[test]
    fn dot() {
        assert_eq!(Vector2::new(1.0, 2.0) * Vector2::new(3.0, 4.0), 11.0);
    }

    #[test]
    fn mul_scalar() {
        assert_eq!(Vector2::new(1.0, 2.0) * 2.0, Vector2::new(2.0, 4.0));
    }

    #[test]
    fn div_scalar() {
        assert_eq!(Vector2::new(2.0, 4.0) / 2.0, Vector2::new(1.0, 2.0));
    }

    #[test]
    fn neg() {
        assert_eq!(-Vector2::new(1.0, -2.0), Vector2::new(-1.0, 2.0));
    }

    #[test]
    fn neg_ref() {
        assert_eq!(-&Vector2::new(1.0, -2.0), Vector2::new(-1.0, 2.0));
    }
}
