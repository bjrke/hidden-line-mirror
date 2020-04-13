use crate::drawcontext::*;
use crate::float::*;
use crate::point::*;
use crate::vec2::*;
use crate::vec3::*;
use std::rc::Rc;

#[derive(Debug)]
pub struct Triangle {
    pub p1: Rc<Point>,
    pub p2: Rc<Point>,
    pub p3: Rc<Point>,
    pub plane_norm: Vector3,
    pub plane_dist: Float,
    pub gl: u8,
    pub cols: Color,
}

pub type Polygon = Rc<Triangle>;

impl Triangle {
    pub fn new(p1: Rc<Point>, p2: Rc<Point>, p3: Rc<Point>, ls: u8, cols: Color) -> Triangle {
        let plane_norm = (p2.o - p1.o).cross(&(p3.o - p1.o)).normalize();

        let plane_dist = plane_norm * p1.o;
        Triangle {
            p1,
            p2,
            p3,
            plane_norm,
            plane_dist,
            gl: ls,
            cols,
        }
    }

    pub fn has_no_area(&self) -> bool {
        !colinear(&self.p1.b, &self.p2.b, &self.p3.b)
    }
}
