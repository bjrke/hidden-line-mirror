use crate::appcontext::*;
use crate::drawcontext::*;
use crate::float::*;
use crate::mat3::*;
use crate::point::*;
use crate::vec2::*;
use crate::vec3::*;
use std::rc::Rc;

#[derive(Debug)]
pub struct dreieck {
    pub p1: Rc<punkt3d>,
    pub p2: Rc<punkt3d>,
    pub p3: Rc<punkt3d>,
    pub plane_norm: Vector3,
    pub plane_dist: Float,
    pub gl: u8,
    pub cols: Color,
}

pub type polygon = Rc<dreieck>;

impl dreieck {
    pub fn new(p1: Rc<punkt3d>, p2: Rc<punkt3d>, p3: Rc<punkt3d>, ls: u8, cols: Color) -> dreieck {
        let plane_norm = (p2.o - p1.o).cross(&(p3.o - p1.o)).normalize();

        let plane_dist = plane_norm * p1.o;
        dreieck {
            p1,
            p2,
            p3,
            plane_norm,
            plane_dist,
            gl: ls,
            cols,
        }
    }

    pub fn flaechentest(&self) -> bool {
        !colinear(&self.p1.b, &self.p2.b, &self.p3.b)
    }
}
