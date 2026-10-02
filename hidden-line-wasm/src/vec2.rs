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

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn len_sq() {
        assert_eq!(Vector2(3.0, 4.0).len_sq(), 25.0);
    }

    #[test]
    fn len() {
        assert_eq!(Vector2(3.0, 4.0).len(), 5.0);
    }

    #[test]
    fn mix_midpoint() {
        assert_eq!(
            Vector2(0.0, 0.0).mix(&Vector2(10.0, 20.0), 0.5),
            Vector2(5.0, 10.0)
        );
    }

    #[test]
    fn mix_at_start() {
        assert_eq!(
            Vector2(0.0, 0.0).mix(&Vector2(10.0, 20.0), 0.0),
            Vector2(10.0, 20.0)
        );
    }

    #[test]
    fn mix_at_end() {
        assert_eq!(
            Vector2(0.0, 0.0).mix(&Vector2(10.0, 20.0), 1.0),
            Vector2(0.0, 0.0)
        );
    }

    #[test]
    fn add() {
        assert_eq!(Vector2(1.0, 2.0) + Vector2(3.0, 4.0), Vector2(4.0, 6.0));
    }

    #[test]
    fn add_ref() {
        assert_eq!(&Vector2(1.0, 2.0) + &Vector2(3.0, 4.0), Vector2(4.0, 6.0));
    }

    #[test]
    fn sub() {
        assert_eq!(Vector2(1.0, 2.0) - Vector2(3.0, 5.0), Vector2(-2.0, -3.0));
    }

    #[test]
    fn sub_ref() {
        assert_eq!(&Vector2(1.0, 2.0) - &Vector2(3.0, 5.0), Vector2(-2.0, -3.0));
    }

    #[test]
    fn dot() {
        assert_eq!(Vector2(1.0, 2.0) * Vector2(3.0, 4.0), 11.0);
    }

    #[test]
    fn mul_scalar() {
        assert_eq!(Vector2(1.0, 2.0) * 2.0, Vector2(2.0, 4.0));
    }

    #[test]
    fn div_scalar() {
        assert_eq!(Vector2(2.0, 4.0) / 2.0, Vector2(1.0, 2.0));
    }

    #[test]
    fn neg() {
        assert_eq!(-Vector2(1.0, -2.0), Vector2(-1.0, 2.0));
    }

    #[test]
    fn neg_ref() {
        assert_eq!(-&Vector2(1.0, -2.0), Vector2(-1.0, 2.0));
    }
}
