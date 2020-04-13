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
}

impl polygon {
    pub fn initpoly(
        p1: Vector2,
        p2: Vector2,
        p3: Vector2,
        gl: u8,
        originalTriangle: Rc<dreieck>,
    ) -> polygon {
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
        }
    }
    pub fn newpoly(aOriginalTriangle: Rc<dreieck>) -> polygon {
        polygon::initpoly(
            aOriginalTriangle.delegate.p1,
            aOriginalTriangle.delegate.p2,
            aOriginalTriangle.delegate.p3,
            aOriginalTriangle.delegate.gl,
            aOriginalTriangle.clone(),
        )
    }
}
