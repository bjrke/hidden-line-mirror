use crate::vec2::*;
use crate::vec3::*;

#[derive(Debug)]
pub struct Point {
    pub b: Vector2,
    pub o: Vector3,
}

impl Point {
    pub fn new(o: &Vector3) -> Point {
        Point {
            o: *o,
            b: Vector2(0.0, 0.0),
        }
    }
}
