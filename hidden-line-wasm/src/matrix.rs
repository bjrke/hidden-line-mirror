use crate::float::Float;
use crate::vec2::Vector2;
use crate::vec3::Vector3;

pub struct Matrix2(pub Vector2, pub Vector2);

impl Matrix2 {
    pub fn determinant(&self) -> Float {
        let Matrix2(x, y) = self;
        x.x * y.y - x.y * y.x
    }
}

impl std::fmt::Display for Matrix2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Matrix2(x, y) = self;
        write!(f, "({}, {})", x, y)
    }
}

pub struct Matrix3(pub Vector3, pub Vector3, pub Vector3);

impl Matrix3 {
    pub fn determinant(&self) -> Float {
        let Matrix3(x, y, z) = self;
        x.x * y.y * z.z + y.x * z.y * x.z + z.x * x.y * y.z
            - x.x * z.y * y.z
            - y.x * x.y * z.z
            - z.x * y.y * x.z
    }
}

impl std::fmt::Display for Matrix3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Matrix3(x, y, z) = self;
        write!(f, "({}, {}, {})", x, y, z)
    }
}
