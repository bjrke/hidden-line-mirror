use crate::appcontext::*;
use crate::drawcontext::*;
use crate::dreidext::*;
use crate::float::*;
use crate::mat3::*;
use crate::maxxqueue::*;
use crate::minxqueue::*;
use crate::minxqueue::*;
use crate::point::*;
use crate::polygon::*;
use crate::time::*;
use crate::triangle::*;
use crate::vec3::*;
use std::collections::BinaryHeap;
use std::rc::Rc;

pub struct CalcContext {
    pub tiefePerspektive: minmax,
    pub minxQueue: BinaryHeap<MinxQueueEntry>,
    pub maxxQueue: BinaryHeap<MaxxQueueEntry>,
    pub xscan: Float,
    pub currentSweep: Vec<polygon>,
}

impl CalcContext {
    pub fn test(&mut self, dctx: &mut dyn DrawContext, actx: &AppContext) {
        loop {
            match self.minxQueue.pop() {
                Some(poly) => {
                    poly.polygon.drawpoly(self, dctx, actx);
                }

                _ => return,
            }
        }
    }
}
