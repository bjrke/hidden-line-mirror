use crate::float::*;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone, Copy)]
pub struct Vector3 {
    pub x: Float,
    pub y: Float,
    pub z: Float,
}

const MOVE_SPEED: Float = 1.0;

impl Vector3 {
    pub fn new(x: Float, y: Float, z: Float) -> Self {
        Self { x, y, z }
    }

    #[inline]
    pub fn sub3d(&self, v: &Self) -> Self {
        self - v
    }

    #[inline]
    pub fn add3d(&self, v: &Self) -> Self {
        self + v
    }

    #[inline]
    pub fn mul3d(&self, f: Float) -> Self {
        self * f
    }

    #[inline]
    pub fn div3d(&self, d: Float) -> Self {
        self / d
    }

    #[inline]
    pub fn neg3d(&self) -> Self {
        Vector3::new(-self.x, -self.y, -self.z)
    }

    #[inline]
    pub fn cross(&self, v: &Self) -> Self {
        Vector3::new(
            self.y * v.z - self.z * v.y,
            self.z * v.x - self.x * v.z,
            self.x * v.y - self.y * v.x,
        )
    }

    #[inline]
    pub fn skalar(&self, v: &Self) -> Float {
        self * v
    }

    #[inline]
    pub fn invBetrag3d(&self) -> Float {
        1.0 / self.len()
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
    pub fn move3d(&self, direction: &Self, polarisation: Float) -> Self {
        self + &(direction * (polarisation * MOVE_SPEED * direction.invBetrag3d()))
    }

    #[inline]
    pub fn normalize(&self) -> Self {
        self * self.invBetrag3d()
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
        Self::Output::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Add for &Vector3 {
    type Output = Vector3;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self::Output::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub for Vector3 {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Sub for &Vector3 {
    type Output = Vector3;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
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
        Self::Output::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Mul<Float> for &Vector3 {
    type Output = Vector3;

    #[inline]
    fn mul(self, rhs: Float) -> Self::Output {
        Self::Output::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Div<Float> for Vector3 {
    type Output = Self;

    #[inline]
    fn div(self, rhs: Float) -> Self::Output {
        Self::Output::new(self.x / rhs, self.y / rhs, self.y / rhs)
    }
}

impl Div<Float> for &Vector3 {
    type Output = Vector3;

    #[inline]
    fn div(self, rhs: Float) -> Self::Output {
        Self::Output::new(self.x / rhs, self.y / rhs, self.y / rhs)
    }
}

impl std::fmt::Display for Vector3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.y)
    }
}

pub fn rot_vec(to_rot1: &mut Vector3, to_rot2: &mut Vector3, t: Float) {
    let rad = t * PI / 180.0;
    let rot_inc = rad.cos() / rad.sin();
    let rot_len = (rot_inc.sqr() + 1.0).sqrt();
    let rot_inc = (rot_inc / rot_len);

    let copy1 = *to_rot1;
    let copy2 = *to_rot2;

    let len1 = to_rot1.len();
    let len2 = to_rot2.len();

    if len1 == 0.0 {
        println!("len1 = 0");
    }
    if len2 == 0.0 {
        println!("len2 = 0");
    }

    let f1 = len1 / (len2 * rot_len);
    let f2 = -len2 / (len1 * rot_len);

    to_rot1.x = rot_inc * copy1.x + copy2.x * f1;
    to_rot1.y = rot_inc * copy1.y + copy2.y * f1;
    to_rot1.z = rot_inc * copy1.z + copy2.z * f1;

    to_rot2.x = f2 * copy1.x + rot_inc * copy2.x;
    to_rot2.y = f2 * copy1.y + rot_inc * copy2.y;
    to_rot2.z = f2 * copy1.z + rot_inc * copy2.z;
}
