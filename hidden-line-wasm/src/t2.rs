use crate::linesegment::*;
use crate::point::*;
use std::rc::Rc;

struct triangle {
    p_1: Rc<punkt3d>,
    p_2: Rc<punkt3d>,
    p_3: Rc<punkt3d>,
}

impl triangle {
    pub fn intersect(&self, l: &mut Line) {
        if l.is_empty() {
            return;
        }
    }
}
