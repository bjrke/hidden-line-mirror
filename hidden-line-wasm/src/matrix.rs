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
