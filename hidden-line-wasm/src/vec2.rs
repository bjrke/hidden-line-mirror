use crate::float::*;
use std::cmp::*;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vector2 {
    pub x: Float,
    pub y: Float,
}

#[inline]
pub fn Vector2(x: Float, y: Float) -> Vector2 {
    Vector2 { x, y }
}

impl Vector2 {
    #[inline]
    pub fn sqrbetrag2d(&self) -> Float {
        self * self
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

impl std::fmt::Debug for Vector2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("").field(&self.x).field(&self.y).finish()
    }
}

impl Add for Vector2 {
    type Output = Vector2;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Vector2(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Add for &Vector2 {
    type Output = Vector2;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Vector2(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign for Vector2 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Vector2 {
    type Output = Vector2;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Vector2(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Sub for &Vector2 {
    type Output = Vector2;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Vector2(self.x - rhs.x, self.y - rhs.y)
    }
}

impl SubAssign for Vector2 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Mul for Vector2 {
    type Output = Float;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y
    }
}

impl Mul for &Vector2 {
    type Output = Float;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y
    }
}

impl Mul<Float> for Vector2 {
    type Output = Vector2;

    #[inline]
    fn mul(self, rhs: Float) -> Self::Output {
        Vector2(self.x * rhs, self.y * rhs)
    }
}

impl Mul<Float> for &Vector2 {
    type Output = Vector2;

    #[inline]
    fn mul(self, rhs: Float) -> Self::Output {
        Vector2(self.x * rhs, self.y * rhs)
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
    fn div(self, rhs: Float) -> Self::Output {
        Vector2(self.x / rhs, self.y / rhs)
    }
}

impl Div<Float> for &Vector2 {
    type Output = Vector2;

    #[inline]
    fn div(self, rhs: Float) -> Self::Output {
        Vector2(self.x / rhs, self.y / rhs)
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
    fn neg(self) -> Self::Output {
        Vector2(-self.x, -self.y)
    }
}

impl Neg for &Vector2 {
    type Output = Vector2;

    #[inline]
    fn neg(self) -> Self::Output {
        Vector2(-self.x, -self.y)
    }
}

impl Eq for Vector2 {}

impl Ord for Vector2 {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl std::fmt::Display for Vector2 {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[inline]
pub fn colinear(p1: &Vector2, p2: &Vector2, p3: &Vector2) -> bool {
    ((p1.y - p2.y) * (p3.x - p2.x) - (p1.x - p2.x) * (p3.y - p2.y)).abs() < epsilon2
}
