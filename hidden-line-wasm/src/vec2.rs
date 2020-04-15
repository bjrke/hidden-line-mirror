use crate::float::*;
use std::cmp::*;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
pub struct Vector2(pub Float, pub Float);

impl Vector2 {
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

impl Add for Vector2 {
    type Output = Vector2;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        let Vector2(sx, sy) = self;
        let Vector2(rx, ry) = rhs;
        Vector2(sx + rx, sy + ry)
    }
}

impl Add for &Vector2 {
    type Output = Vector2;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        let Vector2(sx, sy) = *self;
        let Vector2(rx, ry) = *rhs;
        Vector2(sx + rx, sy + ry)
    }
}

impl AddAssign for Vector2 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        let Vector2(ref mut sx, ref mut sy) = self;
        let Vector2(rx, ry) = rhs;
        *sx += rx;
        *sy += ry;
    }
}

impl Sub for Vector2 {
    type Output = Vector2;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        let Vector2(sx, sy) = self;
        let Vector2(rx, ry) = rhs;
        Vector2(sx - rx, sy - ry)
    }
}

impl Sub for &Vector2 {
    type Output = Vector2;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        let Vector2(sx, sy) = *self;
        let Vector2(rx, ry) = *rhs;
        Vector2(sx - rx, sy - ry)
    }
}

impl SubAssign for Vector2 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        let Vector2(ref mut sx, ref mut sy) = self;
        let Vector2(rx, ry) = rhs;
        *sx -= rx;
        *sy -= ry;
    }
}

impl Mul for Vector2 {
    type Output = Float;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        let Vector2(sx, sy) = self;
        let Vector2(rx, ry) = rhs;
        sx * rx + sy * ry
    }
}

impl Mul for &Vector2 {
    type Output = Float;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        let Vector2(sx, sy) = *self;
        let Vector2(rx, ry) = *rhs;
        sx * rx + sy * ry
    }
}

impl Mul<Float> for Vector2 {
    type Output = Vector2;

    #[inline]
    fn mul(self, rhs: Float) -> Self::Output {
        let Vector2(sx, sy) = self;
        Vector2(sx * rhs, sy * rhs)
    }
}

impl Mul<Float> for &Vector2 {
    type Output = Vector2;

    #[inline]
    fn mul(self, rhs: Float) -> Self::Output {
        let Vector2(sx, sy) = *self;
        Vector2(sx * rhs, sy * rhs)
    }
}

impl MulAssign<Float> for Vector2 {
    #[inline]
    fn mul_assign(&mut self, rhs: Float) {
        let Vector2(ref mut sx, ref mut sy) = self;
        *sx *= rhs;
        *sy *= rhs;
    }
}

impl Div<Float> for Vector2 {
    type Output = Vector2;

    #[inline]
    fn div(self, rhs: Float) -> Self::Output {
        let Vector2(sx, sy) = self;
        Vector2(sx / rhs, sy / rhs)
    }
}

impl Div<Float> for &Vector2 {
    type Output = Vector2;

    #[inline]
    fn div(self, rhs: Float) -> Self::Output {
        let Vector2(sx, sy) = *self;
        Vector2(sx / rhs, sy / rhs)
    }
}

impl DivAssign<Float> for Vector2 {
    #[inline]
    fn div_assign(&mut self, rhs: Float) {
        let Vector2(ref mut sx, ref mut sy) = self;
        *sx /= rhs;
        *sy /= rhs;
    }
}

impl Neg for Vector2 {
    type Output = Vector2;

    #[inline]
    fn neg(self) -> Self::Output {
        let Vector2(sx, sy) = self;
        Vector2(-sx, -sy)
    }
}

impl Neg for &Vector2 {
    type Output = Vector2;

    #[inline]
    fn neg(self) -> Self::Output {
        let Vector2(sx, sy) = *self;
        Vector2(-sx, -sy)
    }
}

#[inline]
pub fn colinear(p1: &Vector2, p2: &Vector2, p3: &Vector2) -> bool {
    let Vector2(x1, y1) = *p1;
    let Vector2(x2, y2) = *p2;
    let Vector2(x3, y3) = *p3;
    ((y1 - y2) * (x3 - x3) - (x1 - x2) * (y3 - y2)).abs() < EPSILON2
}
