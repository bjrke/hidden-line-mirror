use crate::float::*;
use std::hash::{Hash, Hasher};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Clone, Copy, Debug)]
pub struct Vector3 {
    pub x: Float,
    pub y: Float,
    pub z: Float,
}

impl Vector3 {
    #[inline]
    pub const fn new(x: Float, y: Float, z: Float) -> Vector3 {
        Vector3 { x, y, z }
    }

    #[inline]
    pub fn cross(&self, rhs: &Self) -> Self {
        let (sx, sy, sz) = (*self).into();
        let (rx, ry, rz) = (*rhs).into();
        Vector3::new(sy * rz - sz * ry, sz * rx - sx * rz, sx * ry - sy * rx)
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

impl From<(Float, Float, Float)> for Vector3 {
    #[inline]
    fn from((x, y, z): (Float, Float, Float)) -> Vector3 {
        Vector3::new(x, y, z)
    }
}

impl From<Vector3> for (Float, Float, Float) {
    #[inline]
    fn from(v: Vector3) -> (Float, Float, Float) {
        (v.x, v.y, v.z)
    }
}

impl Eq for Vector3 {}

impl PartialEq for Vector3 {
    fn eq(&self, rhs: &Self) -> bool {
        let (sx, sy, sz) = (*self).into();
        let (rx, ry, rz) = (*rhs).into();
        sx.round_for_eq() == rx.round_for_eq()
            && sy.round_for_eq() == ry.round_for_eq()
            && sz.round_for_eq() == rz.round_for_eq()
    }
}

impl Hash for Vector3 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let (x, y, z) = (*self).into();
        x.round_for_eq().hash(state);
        y.round_for_eq().hash(state);
        z.round_for_eq().hash(state);
    }
}

impl Add for Vector3 {
    type Output = Vector3;

    #[inline]
    fn add(self, rhs: Vector3) -> Vector3 {
        Vector3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Add for &Vector3 {
    type Output = Vector3;

    #[inline]
    fn add(self, rhs: &Vector3) -> Vector3 {
        Vector3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl AddAssign for Vector3 {
    #[inline]
    fn add_assign(&mut self, rhs: Vector3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub for Vector3 {
    type Output = Vector3;

    #[inline]
    fn sub(self, rhs: Vector3) -> Vector3 {
        Vector3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Sub for &Vector3 {
    type Output = Vector3;

    #[inline]
    fn sub(self, rhs: &Vector3) -> Vector3 {
        Vector3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl SubAssign for Vector3 {
    #[inline]
    fn sub_assign(&mut self, rhs: Vector3) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Mul for Vector3 {
    type Output = Float;

    #[inline]
    fn mul(self, rhs: Vector3) -> Float {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl Mul for &Vector3 {
    type Output = Float;

    #[inline]
    fn mul(self, rhs: &Vector3) -> Float {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl Mul<Float> for Vector3 {
    type Output = Vector3;

    #[inline]
    fn mul(self, rhs: Float) -> Vector3 {
        Vector3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Mul<Float> for &Vector3 {
    type Output = Vector3;

    #[inline]
    fn mul(self, rhs: Float) -> Vector3 {
        Vector3::new(self.x * rhs, self.y * rhs, self.z * rhs)
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
    type Output = Vector3;

    #[inline]
    fn div(self, rhs: Float) -> Vector3 {
        Vector3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl Div<Float> for &Vector3 {
    type Output = Vector3;

    #[inline]
    fn div(self, rhs: Float) -> Vector3 {
        Vector3::new(self.x / rhs, self.y / rhs, self.z / rhs)
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
    fn neg(self) -> Vector3 {
        Vector3::new(-self.x, -self.y, -self.z)
    }
}

impl Neg for &Vector3 {
    type Output = Vector3;

    #[inline]
    fn neg(self) -> Vector3 {
        Vector3::new(-self.x, -self.y, -self.z)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::collections::hash_map::DefaultHasher;

    fn hash_of(v: &Vector3) -> u64 {
        let mut hasher = DefaultHasher::new();
        v.hash(&mut hasher);
        hasher.finish()
    }

    #[test]
    fn cross_of_basis_vectors() {
        assert_eq!(
            Vector3::new(1.0, 0.0, 0.0).cross(&Vector3::new(0.0, 1.0, 0.0)),
            Vector3::new(0.0, 0.0, 1.0)
        );
    }

    #[test]
    fn cross_is_anticommutative() {
        assert_eq!(
            Vector3::new(1.0, 2.0, 3.0).cross(&Vector3::new(4.0, 5.0, 6.0)),
            -Vector3::new(4.0, 5.0, 6.0).cross(&Vector3::new(1.0, 2.0, 3.0))
        );
    }

    #[test]
    fn len_sq() {
        assert_eq!(Vector3::new(1.0, 2.0, 2.0).len_sq(), 9.0);
    }

    #[test]
    fn len() {
        assert_eq!(Vector3::new(1.0, 2.0, 2.0).len(), 3.0);
    }

    #[test]
    fn normalize() {
        assert_eq!(
            Vector3::new(2.0, 0.0, 0.0).normalize(),
            Vector3::new(1.0, 0.0, 0.0)
        );
    }

    #[test]
    fn normalize_preserves_direction() {
        let v = Vector3::new(2.0, 4.0, 8.0);
        let n = v.normalize();
        assert!((n.len() - 1.0).abs() < 1.0e-6);
        assert!((n.cross(&v).len()).abs() < 1.0e-6);
    }

    #[test]
    fn eq_rounds_close_values() {
        assert_eq!(
            Vector3::new(1.0, 0.0, 0.0),
            Vector3::new(1.000001, 0.0, 0.0)
        );
    }

    #[test]
    fn eq_distinguishes_distinct_values() {
        assert!(Vector3::new(1.0, 0.0, 0.0) != Vector3::new(2.0, 0.0, 0.0));
    }

    #[test]
    fn hash_consistent_with_eq() {
        let a = Vector3::new(1.0, 2.0, 3.0);
        let b = Vector3::new(1.000001, 2.0, 3.0);
        assert_eq!(hash_of(&a), hash_of(&b));
    }

    #[test]
    fn add() {
        assert_eq!(
            Vector3::new(1.0, 2.0, 3.0) + Vector3::new(4.0, 5.0, 6.0),
            Vector3::new(5.0, 7.0, 9.0)
        );
    }

    #[test]
    fn add_ref() {
        assert_eq!(
            &Vector3::new(1.0, 2.0, 3.0) + &Vector3::new(4.0, 5.0, 6.0),
            Vector3::new(5.0, 7.0, 9.0)
        );
    }

    #[test]
    fn sub() {
        assert_eq!(
            Vector3::new(1.0, 2.0, 3.0) - Vector3::new(4.0, 6.0, 8.0),
            Vector3::new(-3.0, -4.0, -5.0)
        );
    }

    #[test]
    fn dot() {
        assert_eq!(
            Vector3::new(1.0, 2.0, 3.0) * Vector3::new(4.0, 5.0, 6.0),
            32.0
        );
    }

    #[test]
    fn mul_scalar() {
        assert_eq!(
            Vector3::new(1.0, 2.0, 3.0) * 2.0,
            Vector3::new(2.0, 4.0, 6.0)
        );
    }

    #[test]
    fn div_scalar() {
        assert_eq!(
            Vector3::new(2.0, 4.0, 6.0) / 2.0,
            Vector3::new(1.0, 2.0, 3.0)
        );
    }

    #[test]
    fn neg() {
        assert_eq!(-Vector3::new(1.0, -2.0, 3.0), Vector3::new(-1.0, 2.0, -3.0));
    }

    #[test]
    fn neg_ref() {
        assert_eq!(
            -&Vector3::new(1.0, -2.0, 3.0),
            Vector3::new(-1.0, 2.0, -3.0)
        );
    }
}
