use crate::drawcontext::*;
use crate::float::*;
use crate::vec2::*;
use crate::vec3::*;

#[derive(Debug)]
pub struct punkt3d {
    pub b: Vector2,
    pub o: Vector3,
}

impl punkt3d {
    pub fn new(o: &Vector3) -> punkt3d {
        punkt3d {
            o: *o,
            b: Vector2(0.0, 0.0),
        }
    }
}
