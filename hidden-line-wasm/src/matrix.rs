use crate::float::Float;
use crate::vec2::Vector2;
use crate::vec3::Vector3;

pub struct Matrix2(pub Vector2, pub Vector2);

impl Matrix2 {
    pub fn determinant(&self) -> Float {
        let Matrix2(Vector2(xx, xy), Vector2(yx, yy)) = self;
        xx * yy - xy * yx
    }
}

pub struct Matrix3(pub Vector3, pub Vector3, pub Vector3);

impl Matrix3 {
    pub fn determinant(&self) -> Float {
        let Matrix3(Vector3(xx, xy, xz), Vector3(yx, yy, yz), Vector3(zx, zy, zz)) = *self;
        xx * yy * zz + yx * zy * xz + zx * xy * yz - xx * zy * yz - yx * xy * zz - zx * yy * xz
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn matrix2_determinant() {
        assert_eq!(
            Matrix2(Vector2(1.0, 2.0), Vector2(3.0, 4.0)).determinant(),
            -2.0
        );
    }

    #[test]
    fn matrix2_determinant_zero() {
        assert_eq!(
            Matrix2(Vector2(1.0, 2.0), Vector2(2.0, 4.0)).determinant(),
            0.0
        );
    }

    #[test]
    fn matrix3_identity_determinant() {
        assert_eq!(
            Matrix3(
                Vector3(1.0, 0.0, 0.0),
                Vector3(0.0, 1.0, 0.0),
                Vector3(0.0, 0.0, 1.0)
            )
            .determinant(),
            1.0
        );
    }

    #[test]
    fn matrix3_diagonal_determinant() {
        assert_eq!(
            Matrix3(
                Vector3(2.0, 0.0, 0.0),
                Vector3(0.0, 3.0, 0.0),
                Vector3(0.0, 0.0, 4.0)
            )
            .determinant(),
            24.0
        );
    }

    #[test]
    fn matrix3_singular_determinant() {
        assert_eq!(
            Matrix3(
                Vector3(1.0, 2.0, 3.0),
                Vector3(4.0, 5.0, 6.0),
                Vector3(7.0, 8.0, 9.0)
            )
            .determinant(),
            0.0
        );
    }
}
