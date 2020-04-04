use crate::float::*;
use std::cmp::*;

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vector2 {
    pub x: Float,
    pub y: Float,
}

impl Vector2 {
    #[inline]
    pub fn new(x: Float, y: Float) -> Vector2 {
        Vector2 { x, y }
    }

    #[inline]
    pub fn mul2d(&self, f: Float) -> Vector2 {
        Vector2::new(self.x * f, self.y * f)
    }

    #[inline]
    pub fn div2d(&self, d: Float) -> Vector2 {
        Vector2::new(self.x / d, self.y / d)
    }

    #[inline]
    pub fn sub2d(&self, v: &Vector2) -> Vector2 {
        Vector2::new(self.x - v.x, self.y - v.y)
    }

    #[inline]
    pub fn add2d(&self, v: &Vector2) -> Vector2 {
        Vector2::new(self.x + v.x, self.y + v.y)
    }

    #[inline]
    pub fn sqrbetrag2d(&self) -> Float {
        self.x.sqr() + self.y.sqr()
    }

    #[inline]
    pub fn betrag2d(&self) -> Float {
        self.sqrbetrag2d().sqrt()
    }

    #[inline]
    pub fn swap_xy(&self) -> Vector2 {
        Vector2::new(self.y, self.x)
    }

    #[inline]
    pub fn mix(&self, other: &Vector2, t: Float) -> Vector2 {
        let s = 1.0 - t;
        self.mul2d(s).add2d(&other.mul2d(t))
    }

    #[inline]
    pub fn nearly_equals(&self, o: &Self) -> bool {
        self.x.nearly_equals(&o.x) && self.y.nearly_equals(&o.y)
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
