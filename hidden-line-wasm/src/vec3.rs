use crate::float::*;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct Vector3 {
    pub x: Float,
    pub y: Float,
    pub z: Float,
}

#[inline]
pub fn Vector3(x: Float, y: Float, z: Float) -> Vector3 {
    Vector3 { x, y, z }
}

impl Vector3 {
    #[inline]
    pub fn new(x: Float, y: Float, z: Float) -> Self {
        Vector3 { x, y, z }
    }

    #[inline]
    pub fn cross(&self, v: &Self) -> Self {
        Vector3(
            self.y * v.z - self.z * v.y,
            self.z * v.x - self.x * v.z,
            self.x * v.y - self.y * v.x,
        )
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

impl std::fmt::Debug for Vector3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("")
            .field(&self.x)
            .field(&self.y)
            .field(&self.z)
            .finish()
    }
}

impl Add for Vector3 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Vector3(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Add for &Vector3 {
    type Output = Vector3;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Vector3(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl AddAssign for Vector3 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub for Vector3 {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Vector3(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Sub for &Vector3 {
    type Output = Vector3;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Vector3(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl SubAssign for Vector3 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Mul for Vector3 {
    type Output = Float;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl Mul for &Vector3 {
    type Output = Float;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl Mul<Float> for Vector3 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Float) -> Self::Output {
        Vector3(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Mul<Float> for &Vector3 {
    type Output = Vector3;

    #[inline]
    fn mul(self, rhs: Float) -> Self::Output {
        Vector3(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl MulAssign<Float> for Vector3 {
    #[inline]
    fn mul_assign(&mut self, rhs: Float) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl Div<Float> for Vector3 {
    type Output = Self;

    #[inline]
    fn div(self, rhs: Float) -> Self::Output {
        Vector3(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl Div<Float> for &Vector3 {
    type Output = Vector3;

    #[inline]
    fn div(self, rhs: Float) -> Self::Output {
        Vector3(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl DivAssign<Float> for Vector3 {
    #[inline]
    fn div_assign(&mut self, rhs: Float) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl Neg for Vector3 {
    type Output = Vector3;

    #[inline]
    fn neg(self) -> Self::Output {
        Vector3(-self.x, -self.y, -self.z)
    }
}

impl Neg for &Vector3 {
    type Output = Vector3;

    #[inline]
    fn neg(self) -> Self::Output {
        Vector3(-self.x, -self.y, -self.z)
    }
}

impl std::fmt::Display for Vector3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.y)
    }
}
