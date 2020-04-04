use crate::appcontext::*;
use crate::drawcontext::*;
use crate::float::*;
use crate::maxxqueue::*;
use crate::minxqueue::*;
use crate::polygon::*;
use crate::quadtree::QuadTree;
use crate::shape::{Rect, Shape, Triangle};
use crate::time::*;
use crate::vec2::Vector2;
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

struct TheLine {}

#[derive(Debug)]
struct TheTriangle {
    shape: Triangle,
    poly: polygon,
}

impl TheTriangle {
    fn new(poly: polygon) -> TheTriangle {
        let shape = Triangle::new(
            &poly.delegate.p1.b,
            &poly.delegate.p2.b,
            &poly.delegate.p2.b,
        );

        TheTriangle { poly, shape }
    }
}

impl Shape for TheTriangle {
    fn intersects(&self, r: &Rect) -> bool {
        self.shape.intersects(r)
    }

    fn contains(&self, v: &Vector2) -> bool {
        self.shape.contains(v)
    }

    fn bounds(&self) -> Rect {
        self.shape.bounds()
    }
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
        let mut triangles = vec![];
        while let Some(MinxQueueEntry { polygon: poly }) = self.minxQueue.pop() {
            let triangle = TheTriangle::new(poly);
            triangles.push(triangle);
        }

        let screen = triangles.iter().fold(None, |acc: Option<Rect>, t| {
            let bound = t.bounds();
            if let Some(existing) = acc {
                Some(existing.extend_rect(&bound))
            } else {
                Some(bound)
            }
        });

        if let Some(screen) = screen {
            let mut tree = QuadTree::new(screen);
            while let Some(mut t) = triangles.pop() {
                // t.poly.drawpoly(self, dctx, actx);

                for o in tree.elements_intersecting_mut(&t.bounds()) {
                    CalcContext::intesect(&mut t, o)
                }
                tree.insert(t);
            }

            for t in tree.elements() {
                t.poly.drawpoly(self, dctx, actx);
            }
        }
    }

    fn intesect(t1: &mut TheTriangle, t2: &mut TheTriangle) {}

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
