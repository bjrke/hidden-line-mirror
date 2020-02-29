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
use std::collections::BTreeSet;
use std::collections::BinaryHeap;
use std::rc::Rc;

pub struct CalcContext {
    pub tiefePerspektive: minmax,
    pub minxQueue: BinaryHeap<MinxQueueEntry>,
    pub queue2: Vec<polygon>,
    pub maxxQueue: BTreeSet<MaxxQueueEntry>,
    pub xscan: Float,
    pub currentSweep: Vec<Rc<polygon>>,
}

impl CalcContext {
    pub fn new() -> CalcContext {
        CalcContext {
            tiefePerspektive: minmax::new(),
            minxQueue: BinaryHeap::new(),
            queue2: Vec::new(),
            maxxQueue: BTreeSet::new(),
            xscan: MIN,
            currentSweep: Vec::new(),
        }
    }

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

    pub fn pushMinX(&mut self, polygon: polygon) {
        self.minxQueue.push(MinxQueueEntry { polygon });
    }

    pub fn pushQ2(&mut self, polygon: polygon) {
        self.queue2.push(polygon);
    }

    pub fn pushSweep(&mut self, polygon: polygon, pos: usize) {
        let polygon = Rc::new(polygon);
        self.currentSweep.insert(pos, polygon.clone());
        self.maxxQueue.insert(MaxxQueueEntry { polygon });
    }

    pub fn sdelete(&mut self, pos: usize) -> polygon {
        let polygon = self.currentSweep.remove(pos);
        self.maxxQueue.remove(&MaxxQueueEntry {
            polygon: polygon.clone(),
        });
        match Rc::try_unwrap(polygon) {
            Ok(r) => r,
            _ => panic!("should not happen"),
        }
    }
}
