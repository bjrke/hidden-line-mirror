use crate::float::*;

#[derive(Clone, Copy)]
pub struct Vector3 {
    pub x: Float,
    pub y: Float,
    pub z: Float,
}

const MOVE_SPEED: Float = 1.0;

pub type vector3d = Vector3;

impl Vector3 {
    pub fn new(x: Float, y: Float, z: Float) -> Vector3 {
        Vector3 { x, y, z }
    }

    pub fn sub3d(&self, v: &Vector3) -> Vector3 {
        Vector3::new(self.x - v.x, self.y - v.y, self.z - v.z)
    }

    pub fn add3d(&self, v: &Vector3) -> Vector3 {
        Vector3::new(self.x + v.x, self.y + v.y, self.z + v.z)
    }

    pub fn mul3d(&self, f: Float) -> Vector3 {
        Vector3::new(self.x * f, self.y * f, self.z * f)
    }

    pub fn div3d(&self, d: Float) -> Vector3 {
        if d == 0.0 {
            println!("d=0");
        }
        Vector3::new(self.x / d, self.y / d, self.z / d)
    }

    pub fn neg3d(&self) -> Vector3 {
        Vector3::new(-self.x, -self.y, -self.z)
    }

    pub fn kreuz(&self, v: &Vector3) -> Vector3 {
        Vector3::new(
            self.y * v.z - self.z * v.y,
            self.z * v.x - self.x * v.z,
            self.x * v.y - self.y * v.x,
        )
    }

    pub fn skalar(&self, v: &Vector3) -> Float {
        self.x * v.x + self.y * v.y + self.z * v.z
    }

    pub fn invBetrag3d(&self) -> Float {
        (self.x.sqr() + self.y.sqr() + self.z.sqr()).inv_sqrt()
    }

    pub fn move3d(&self, direction: &Vector3, polarisation: Float) -> Vector3 {
        self.add3d(&direction.mul3d(polarisation * MOVE_SPEED * direction.invBetrag3d()))
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
