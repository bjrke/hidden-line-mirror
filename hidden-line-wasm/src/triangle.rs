use crate::appcontext::*;
use crate::drawcontext::*;
use crate::float::*;
use crate::mat3::*;
use crate::point::*;
use crate::vec2::*;
use crate::vec3::*;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct dreiecktyp {
    pub p1: Vector2,
    pub p2: Vector2,
    pub p3: Vector2,
    pub gl: u8,
    pub cols: Color,
}

impl dreiecktyp {
    pub fn flaechentest(&self) -> bool {
        !colinear(&self.p1, &self.p2, &self.p3)
    }
}

#[derive(Debug)]
pub struct dreieck {
    pub delegate: dreiecktyp,
    pub p1: Rc<punkt3d>,
    pub p2: Rc<punkt3d>,
    pub p3: Rc<punkt3d>,
    pub plane_norm: Vector3,
    pub plane_dist: Float,
}

pub type polygon = Rc<dreieck>;

impl dreieck {
    pub fn new(p1: Rc<punkt3d>, p2: Rc<punkt3d>, p3: Rc<punkt3d>, ls: u8, cols: Color) -> dreieck {
        let plane_norm = (p2.o - p1.o).cross(&(p3.o - p1.o)).normalize();

        let dp1 = p1.b;
        let dp2 = p2.b;
        let dp3 = p3.b;

        let plane_dist = plane_norm * p1.o;
        dreieck {
            p1,
            p2,
            p3,
            delegate: dreiecktyp {
                p1: dp1,
                p2: dp2,
                p3: dp3,
                gl: ls,
                cols,
            },
            plane_norm,
            plane_dist,
        }
    }
}
