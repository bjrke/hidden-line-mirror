use crate::drawcontext::*;
use crate::float::Float;
use crate::point::*;
use crate::range::*;
use crate::rangeset::*;
use std::ops::RangeBounds;
use std::rc::Rc;

pub struct Line {
    p_1: Rc<punkt3d>,
    p_2: Rc<punkt3d>,
    color: Color,
    ranges: RangeSet<Float>,
}

impl Line {
    pub fn new(p_1: &Rc<punkt3d>, p_2: &Rc<punkt3d>, color: Color) -> Line {
        Line {
            p_1: p_1.clone(),
            p_2: p_2.clone(),
            color,
            ranges: RangeSet::from_range(&(0.0..1.0)),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.ranges.is_empty()
    }

    pub fn draw(&self, dctx: &mut dyn DrawContext) {
        for (start, end) in self.ranges.0.iter() {
            let a = self.p_1.b.b.mix(&self.p_2.b.b, *start.unwrap(&0.0));
            let e = self.p_1.b.b.mix(&self.p_2.b.b, *end.unwrap(&1.0));
            dctx.line(a.x, a.y, e.x, e.y, self.color);
        }
    }

    fn remove_range<R: RangeBounds<Float>>(&mut self, r: &R) {
        self.ranges.remove(r);
    }
}
