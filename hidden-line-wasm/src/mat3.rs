use crate::float::Float;
use crate::vec3::Vector3;

#[derive(Clone, Copy)]
struct Matrix3 {
    pub x: Vector3,
    pub y: Vector3,
    pub z: Vector3,
}

impl Matrix3 {
    pub fn new(x: Vector3, y: Vector3, z: Vector3) -> Matrix3 {
        Matrix3 { x, y, z }
    }

    pub fn det3d(&self) -> Float {
        self.x.x * self.y.y * self.z.z
            + self.y.x * self.z.y * self.x.z
            + self.z.x * self.x.y * self.y.z
            - self.x.x * self.z.y * self.y.z
            - self.y.x * self.x.y * self.z.z
            - self.z.x * self.y.y * self.x.z
    }
}

impl std::fmt::Display for Matrix3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

type matrix3d = Matrix3;
