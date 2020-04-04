use crate::appcontext::*;
use crate::drawcontext::*;
use crate::float::*;
use crate::maxxqueue::*;
use crate::minxqueue::*;
use crate::point::punkt3d;
use crate::polygon::*;
use crate::quadtree::QuadTree;
use crate::rangeset::RangeSet;
use crate::shape::{Line, Rect, Shape, Triangle};
use crate::time::*;
use crate::vec2::Vector2;
use std::collections::BTreeSet;
use std::collections::BinaryHeap;
use std::ops::Bound;
use std::rc::Rc;

pub struct CalcContext {
    pub tiefePerspektive: minmax,
    pub minxQueue: BinaryHeap<MinxQueueEntry>,
    pub queue2: Vec<polygon>,
    pub maxxQueue: BTreeSet<MaxxQueueEntry>,
    pub xscan: Float,
    pub currentSweep: Vec<Rc<polygon>>,
}

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
            &poly.delegate.p3.b,
        );

        TheTriangle { poly, shape }
    }

    fn lines(&self) -> Vec<TheLine> {
        vec![
            TheLine::new(
                self.poly.originalTriangle.origPoint1.clone(),
                self.poly.originalTriangle.origPoint2.clone(),
                self.poly.delegate.cols,
            ),
            TheLine::new(
                self.poly.originalTriangle.origPoint2.clone(),
                self.poly.originalTriangle.origPoint3.clone(),
                self.poly.delegate.cols,
            ),
            TheLine::new(
                self.poly.originalTriangle.origPoint3.clone(),
                self.poly.originalTriangle.origPoint1.clone(),
                self.poly.delegate.cols,
            ),
        ]
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

struct TheLine {
    p1: Rc<punkt3d>,
    p2: Rc<punkt3d>,
    color: Color,
    shape: Line,
}

impl TheLine {
    fn new(p1: Rc<punkt3d>, p2: Rc<punkt3d>, color: Color) -> TheLine {
        let shape = Line::new(p1.b.b, p2.b.b);
        TheLine {
            p1,
            p2,
            color,
            shape,
        }
    }

    pub fn draw(&self, ctx: &mut dyn DrawContext, ranges: &RangeSet<Float>) {
        let RangeSet(ranges) = ranges;
        for range in ranges {
            match range {
                (Bound::Included(l1), Bound::Included(l2))
                | (Bound::Excluded(l1), Bound::Excluded(l2))
                | (Bound::Included(l1), Bound::Excluded(l2))
                | (Bound::Excluded(l1), Bound::Included(l2)) => {
                    let p1 = self.shape.a.mix(&self.shape.e, *l1);
                    let p2 = self.shape.a.mix(&self.shape.e, *l2);

                    ctx.line(p1.x, p1.y, p2.x, p2.y, self.color);
                }
                _ => {}
            }
        }
    }
}

impl Shape for TheLine {
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
            triangles.push(TheTriangle::new(poly));
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
            for t in triangles {
                tree.insert(t);
            }

            for t in tree.elements() {
                for l in t.lines() {
                    let mut r: RangeSet<Float> = RangeSet::from_range(&(0.0..=1.0));
                    let bounds = t.shape.bounds();
                    let mut candidates = tree.elements_intersecting(&bounds);
                    while let Some(next) = if r.is_empty() {
                        None
                    } else {
                        candidates.next()
                    } {
                        let n1 = next.shape.p1;
                        let n2 = next.shape.p2;
                        let n3 = next.shape.p3;
                        let p1 = l.shape.a;
                        let p2 = l.shape.e;

                        let i1 =
                            n1.nearly_equals(&p1) || n2.nearly_equals(&p1) || n3.nearly_equals(&p1);
                        let i2 =
                            n1.nearly_equals(&p2) || n2.nearly_equals(&p2) || n3.nearly_equals(&p2);

                        if !(i1 && i2) {
                            if next.contains(&p1) && next.contains(&p2) && actx.drawmode == 0 {
                                r.remove(&(..));
                            }
                        }
                    }

                    l.draw(dctx, &r);
                }
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
