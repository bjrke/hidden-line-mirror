use crate::float::Float;
use crate::vec2::Vector2;
use crate::vec3::Vector3;

#[derive(Clone, Copy)]
pub struct Matrix2 {
    pub a: Vector2,
    pub b: Vector2,
}

impl Matrix2 {
    pub const fn new(a: Vector2, b: Vector2) -> Matrix2 {
        Matrix2 { a, b }
    }

    pub fn determinant(&self) -> Float {
        let Matrix2 { a, b } = *self;
        a.x * b.y - a.y * b.x
    }
}

impl From<(Vector2, Vector2)> for Matrix2 {
    #[inline]
    fn from((a, b): (Vector2, Vector2)) -> Matrix2 {
        Matrix2::new(a, b)
    }
}

impl From<Matrix2> for (Vector2, Vector2) {
    #[inline]
    fn from(m: Matrix2) -> (Vector2, Vector2) {
        (m.a, m.b)
    }
}

#[derive(Clone, Copy)]
pub struct Matrix3 {
    pub a: Vector3,
    pub b: Vector3,
    pub c: Vector3,
}

impl Matrix3 {
    pub const fn new(a: Vector3, b: Vector3, c: Vector3) -> Matrix3 {
        Matrix3 { a, b, c }
    }

    pub fn determinant(&self) -> Float {
        let Matrix3 { a, b, c } = *self;
        a.x * b.y * c.z + b.x * c.y * a.z + c.x * a.y * b.z
            - a.x * c.y * b.z
            - b.x * a.y * c.z
            - c.x * b.y * a.z
    }
}

impl From<(Vector3, Vector3, Vector3)> for Matrix3 {
    #[inline]
    fn from((a, b, c): (Vector3, Vector3, Vector3)) -> Matrix3 {
        Matrix3::new(a, b, c)
    }
}

impl From<Matrix3> for (Vector3, Vector3, Vector3) {
    #[inline]
    fn from(m: Matrix3) -> (Vector3, Vector3, Vector3) {
        (m.a, m.b, m.c)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn matrix2_determinant() {
        assert_eq!(
            Matrix2::new(Vector2::new(1.0, 2.0), Vector2::new(3.0, 4.0)).determinant(),
            -2.0
        );
    }

    #[test]
    fn matrix2_determinant_zero() {
        assert_eq!(
            Matrix2::new(Vector2::new(1.0, 2.0), Vector2::new(2.0, 4.0)).determinant(),
            0.0
        );
    }

    #[test]
    fn matrix3_identity_determinant() {
        assert_eq!(
            Matrix3::new(
                Vector3::new(1.0, 0.0, 0.0),
                Vector3::new(0.0, 1.0, 0.0),
                Vector3::new(0.0, 0.0, 1.0),
            )
            .determinant(),
            1.0
        );
    }

    #[test]
    fn matrix3_diagonal_determinant() {
        assert_eq!(
            Matrix3::new(
                Vector3::new(2.0, 0.0, 0.0),
                Vector3::new(0.0, 3.0, 0.0),
                Vector3::new(0.0, 0.0, 4.0),
            )
            .determinant(),
            24.0
        );
    }

    #[test]
    fn matrix3_singular_determinant() {
        assert_eq!(
            Matrix3::new(
                Vector3::new(1.0, 2.0, 3.0),
                Vector3::new(4.0, 5.0, 6.0),
                Vector3::new(7.0, 8.0, 9.0),
            )
            .determinant(),
            0.0
        );
    }
}
