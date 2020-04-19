use crate::float::*;
use std::hash::{Hash, Hasher};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Clone, Copy, Debug)]
pub struct Vector3(pub Float, pub Float, pub Float);

impl Vector3 {
    #[inline]
    pub fn cross(&self, rhs: &Self) -> Self {
        let Vector3(sx, sy, sz) = *self;
        let Vector3(rx, ry, rz) = *rhs;
        Vector3(sy * rz - sz * ry, sz * rx - sx * rz, sx * ry - sy * rx)
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
    pub fn normalize(&self) -> Self {
        self / self.len()
    }
}

impl Eq for Vector3 {}

impl PartialEq for Vector3 {
    fn eq(&self, rhs: &Self) -> bool {
        let Vector3(sx, sy, sz) = *self;
        let Vector3(rx, ry, rz) = *rhs;
        sx.round_for_eq() == rx.round_for_eq()
            && sy.round_for_eq() == ry.round_for_eq()
            && sz.round_for_eq() == rz.round_for_eq()
    }
}

impl Hash for Vector3 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let Vector3(x, y, z) = *self;
        x.round_for_eq().hash(state);
        y.round_for_eq().hash(state);
        z.round_for_eq().hash(state);
    }
}

impl Add for Vector3 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        let Vector3(sx, sy, sz) = self;
        let Vector3(rx, ry, rz) = rhs;
        Vector3(sx + rx, sy + ry, sz + rz)
    }
}

impl Add for &Vector3 {
    type Output = Vector3;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        let Vector3(sx, sy, sz) = *self;
        let Vector3(rx, ry, rz) = *rhs;
        Vector3(sx + rx, sy + ry, sz + rz)
    }
}

impl AddAssign for Vector3 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        let Vector3(ref mut sx, ref mut sy, ref mut sz) = self;
        let Vector3(rx, ry, rz) = rhs;
        *sx += rx;
        *sy += ry;
        *sz += rz;
    }
}

impl Sub for Vector3 {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        let Vector3(sx, sy, sz) = self;
        let Vector3(rx, ry, rz) = rhs;
        Vector3(sx - rx, sy - ry, sz - rz)
    }
}

impl Sub for &Vector3 {
    type Output = Vector3;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        let Vector3(sx, sy, sz) = *self;
        let Vector3(rx, ry, rz) = *rhs;
        Vector3(sx - rx, sy - ry, sz - rz)
    }
}

impl SubAssign for Vector3 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        let Vector3(ref mut sx, ref mut sy, ref mut sz) = self;
        let Vector3(rx, ry, rz) = rhs;
        *sx -= rx;
        *sy -= ry;
        *sz -= rz;
    }
}

impl Mul for Vector3 {
    type Output = Float;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        let Vector3(sx, sy, sz) = self;
        let Vector3(rx, ry, rz) = rhs;
        sx * rx + sy * ry + sz * rz
    }
}

impl Mul for &Vector3 {
    type Output = Float;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        let Vector3(sx, sy, sz) = *self;
        let Vector3(rx, ry, rz) = *rhs;
        sx * rx + sy * ry + sz * rz
    }
}

impl Mul<Float> for Vector3 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Float) -> Self::Output {
        let Vector3(sx, sy, sz) = self;
        Vector3(sx * rhs, sy * rhs, sz * rhs)
    }
}

impl Mul<Float> for &Vector3 {
    type Output = Vector3;

    #[inline]
    fn mul(self, rhs: Float) -> Self::Output {
        let Vector3(sx, sy, sz) = *self;
        Vector3(sx * rhs, sy * rhs, sz * rhs)
    }
}

impl MulAssign<Float> for Vector3 {
    #[inline]
    fn mul_assign(&mut self, rhs: Float) {
        let Vector3(ref mut sx, ref mut sy, ref mut sz) = self;
        *sx *= rhs;
        *sy *= rhs;
        *sz *= rhs;
    }
}

impl Div<Float> for Vector3 {
    type Output = Self;

    #[inline]
    fn div(self, rhs: Float) -> Self::Output {
        let Vector3(sx, sy, sz) = self;
        Vector3(sx / rhs, sy / rhs, sz / rhs)
    }
}

impl Div<Float> for &Vector3 {
    type Output = Vector3;

    #[inline]
    fn div(self, rhs: Float) -> Self::Output {
        let Vector3(sx, sy, sz) = *self;
        Vector3(sx / rhs, sy / rhs, sz / rhs)
    }
}

impl DivAssign<Float> for Vector3 {
    #[inline]
    fn div_assign(&mut self, rhs: Float) {
        let Vector3(ref mut sx, ref mut sy, ref mut sz) = self;
        *sx /= rhs;
        *sy /= rhs;
        *sz /= rhs;
    }
}

impl Neg for Vector3 {
    type Output = Vector3;

    #[inline]
    fn neg(self) -> Self::Output {
        let Vector3(sx, sy, sz) = self;
        Vector3(-sx, -sy, -sz)
    }
}

impl Neg for &Vector3 {
    type Output = Vector3;

    #[inline]
    fn neg(self) -> Self::Output {
        let Vector3(sx, sy, sz) = *self;
        Vector3(-sx, -sy, -sz)
    }
}
