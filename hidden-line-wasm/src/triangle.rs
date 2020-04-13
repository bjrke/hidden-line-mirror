use crate::drawcontext::*;
use crate::float::*;
use crate::point::*;
use crate::vec2::*;
use crate::vec3::*;
use std::rc::Rc;

#[derive(Debug)]
pub struct Polygon {
    pub p1: Rc<Point>,
    pub p2: Rc<Point>,
    pub p3: Rc<Point>,
    pub plane_norm: Vector3,
    pub plane_dist: Float,
    pub gl: u8,
    pub cols: Color,
}

impl Polygon {
    pub fn new(p1: Rc<Point>, p2: Rc<Point>, p3: Rc<Point>, ls: u8, cols: Color) -> Polygon {
        let plane_norm = (p2.o - p1.o).cross(&(p3.o - p1.o)).normalize();

        let plane_dist = plane_norm * p1.o;
        Polygon {
            p1,
            p2,
            p3,
            plane_norm,
            plane_dist,
            gl: ls,
            cols,
        }
    }
}
