use crate::appcontext::*;
use crate::calcctontext::*;
use crate::drawcontext::*;
use crate::float::*;
use crate::point::*;
use crate::triangle::*;
use crate::vec2::*;
use rand::{thread_rng, Rng};
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct polygon {
    pub delegate: dreiecktyp,
    pub originalTriangle: Rc<dreieck>,
    pub farbe: Color,
    pub ymin: Float,
    pub ymax: Float,
}

impl polygon {
    pub fn initpoly(
        p1: &point,
        p2: &point,
        p3: &point,
        ls: u8,
        originalTriangle: Rc<dreieck>,
    ) -> polygon {
        //   zaehl.polygons.ins;

        //TODO copy?
        let mut p1 = *p1;
        let mut p2 = *p2;
        let mut p3 = *p3;

        let mut gl = ls;

        if p1.b.x > p2.b.x {
            let h = p2;
            p2 = p1;
            p1 = h;
            let mut glneu = gl & 4;
            if gl & 1 != 0 {
                glneu += 2;
            }
            if gl & 2 != 0 {
                glneu += 1;
            }
            gl = glneu;
        }

        if p1.b.x > p3.b.x {
            let h = p3;
            p3 = p1;
            p1 = h;
            let mut glneu = gl & 2;
            if gl & 1 != 0 {
                glneu += 4;
            }
            if gl & 4 != 0 {
                glneu += 1;
            }
            gl = glneu;
        }

        if p2.b.x > p3.b.x {
            let h = p3;
            p3 = p2;
            p2 = h;
            let mut glneu = gl & 1;
            if gl & 2 != 0 {
                glneu += 4;
            }
            if gl & 4 != 0 {
                glneu += 2;
            }
            gl = glneu;
        }

        let cols = originalTriangle.delegate.cols;

        polygon {
            delegate: dreiecktyp {
                p1,
                p2,
                p3,
                gl,
                cols,
            },
            farbe: cols,
            originalTriangle,
            ymin: p1.b.y.min(p2.b.y).min(p3.b.y),
            ymax: p1.b.y.max(p2.b.y).max(p3.b.y),
        }
    }
    pub fn newpoly(aOriginalTriangle: Rc<dreieck>) -> polygon {
        polygon::initpoly(
            &aOriginalTriangle.delegate.p1,
            &aOriginalTriangle.delegate.p2,
            &aOriginalTriangle.delegate.p3,
            aOriginalTriangle.delegate.gl,
            aOriginalTriangle.clone(), //TODO find a way to prevent cloning
        )
    }
}
