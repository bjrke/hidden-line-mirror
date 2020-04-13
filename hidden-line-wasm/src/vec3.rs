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
    pub fn new(x: Float, y: Float, z: Float) -> Vector3 {
        Vector3 { x, y, z }
    }

    #[inline]
    pub fn sub3d(&self, v: &Vector3) -> Vector3 {
        *self - *v
    }

    #[inline]
    pub fn add3d(&self, v: &Vector3) -> Vector3 {
        *self + *v
    }

    #[inline]
    pub fn mul3d(&self, f: Float) -> Vector3 {
        *self * f
    }

    #[inline]
    pub fn div3d(&self, d: Float) -> Vector3 {
        *self / d
    }

    #[inline]
    pub fn neg3d(&self) -> Vector3 {
        Vector3::new(-self.x, -self.y, -self.z)
    }

    #[inline]
    pub fn kreuz(&self, v: &Vector3) -> Vector3 {
        Vector3::new(
            self.y * v.z - self.z * v.y,
            self.z * v.x - self.x * v.z,
            self.x * v.y - self.y * v.x,
        )
    }

    #[inline]
    pub fn skalar(&self, v: &Vector3) -> Float {
        *self * *v
    }

    #[inline]
    pub fn invBetrag3d(&self) -> Float {
        (*self * *self).inv_sqrt()
    }

    #[inline]
    pub fn move3d(&self, direction: &Vector3, polarisation: Float) -> Vector3 {
        *self + (*direction * (polarisation * MOVE_SPEED * direction.invBetrag3d()))
    }

    #[inline]
    pub fn normalize(&self) -> Vector3 {
        *self * self.invBetrag3d()
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
    type Output = Vector3;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Vector3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub for Vector3 {
    type Output = Vector3;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Vector3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Mul<Vector3> for Vector3 {
    type Output = Float;

    #[inline]
    fn mul(self, rhs: Vector3) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl Mul<Float> for Vector3 {
    type Output = Vector3;

    #[inline]
    fn mul(self, rhs: Float) -> Self::Output {
        Vector3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Div<Float> for Vector3 {
    type Output = Vector3;

    #[inline]
    fn div(self, rhs: Float) -> Self::Output {
        Vector3::new(self.x / rhs, self.y / rhs, self.y / rhs)
    }
}

impl std::fmt::Display for Vector3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.y)
    }
}

pub fn RotVec(ToRot1: &mut Vector3, ToRot2: &mut Vector3, t: Float) {
    let RotInc = (t * PI / 180.0).cos() / (t * PI / 180.0).sin();
    let InvRotVecLength = 1.0 / (RotInc * RotInc + 1.0).sqrt();

    let Copy1 = *ToRot1;
    let Copy2 = *ToRot2;

    let InvLength1 = Copy1.invBetrag3d();
    let InvLength2 = Copy2.invBetrag3d();

    if InvRotVecLength == 0.0 {
        println!("InvRotVecLength=0");
    }
    if InvLength1 == 0.0 {
        println!("InvLength1=0");
    }
    if InvLength2 == 0.0 {
        println!("InvLength2=0");
    }
    ToRot1.x =
        (InvRotVecLength / InvLength1) * (RotInc * Copy1.x * InvLength1 + Copy2.x * InvLength2);
    ToRot1.y =
        (InvRotVecLength / InvLength1) * (RotInc * Copy1.y * InvLength1 + Copy2.y * InvLength2);
    ToRot1.z =
        (InvRotVecLength / InvLength1) * (RotInc * Copy1.z * InvLength1 + Copy2.z * InvLength2);

    ToRot2.x =
        (InvRotVecLength / InvLength2) * (-Copy1.x * InvLength1 + RotInc * Copy2.x * InvLength2);
    ToRot2.y =
        (InvRotVecLength / InvLength2) * (-Copy1.y * InvLength1 + RotInc * Copy2.y * InvLength2);
    ToRot2.z =
        (InvRotVecLength / InvLength2) * (-Copy1.z * InvLength1 + RotInc * Copy2.z * InvLength2);
}
