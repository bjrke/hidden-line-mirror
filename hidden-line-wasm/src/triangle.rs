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
    pub p1: point,
    pub p2: point,
    pub p3: point,
    pub gl: u8,
    pub cols: Color,
}

impl dreiecktyp {
    pub fn flaechentest(&self) -> bool {
        !colinear(&self.p1.b, &self.p2.b, &self.p3.b)
    }
}

#[derive(Debug)]
pub struct dreieck {
    pub delegate: dreiecktyp,
    pub origPoint1: Rc<punkt3d>,
    pub origPoint2: Rc<punkt3d>,
    pub origPoint3: Rc<punkt3d>,
    pub planeNorm: Vector3,
    pub planeDist: Float,
}

impl dreieck {
    pub fn new(p1: Rc<punkt3d>, p2: Rc<punkt3d>, p3: Rc<punkt3d>, ls: u8, cols: Color) -> dreieck {
        let planeNorm = (p2.o - p1.o).cross(&(p3.o - p1.o)).normalize();

        let dp1 = p1.b;
        let dp2 = p2.b;
        let dp3 = p3.b;

        let planeDist = planeNorm.skalar(&p1.o);
        dreieck {
            origPoint1: p1,
            origPoint2: p2,
            origPoint3: p3,
            delegate: dreiecktyp {
                p1: dp1,
                p2: dp2,
                p3: dp3,
                gl: ls,
                cols,
            },
            planeNorm,
            planeDist,
        }
    }
}
