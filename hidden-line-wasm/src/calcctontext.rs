use std::collections::BTreeSet;
use std::collections::BinaryHeap;
use std::ops::Bound;
use std::rc::Rc;

use crate::appcontext::*;
use crate::drawcontext::*;
use crate::float::*;
use crate::mat2::Matrix2;
use crate::maxxqueue::*;
use crate::minxqueue::*;
use crate::point::punkt3d;
use crate::polygon::*;
use crate::quadtree::QuadTree;
use crate::range::RangeExtCopy;
use crate::rangeset::RangeSet;
use crate::shape::{Line, Rect, Shape, Triangle};
use crate::time::*;
use crate::vec2::Vector2;
use crate::vec3::Vector3;

const DEBUG: bool = false;

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
            &poly.originalTriangle.origPoint1.b.b,
            &poly.originalTriangle.origPoint2.b.b,
            &poly.originalTriangle.origPoint3.b.b,
        );

        TheTriangle { poly, shape }
    }

    fn lines(&self, all: bool) -> Vec<TheLine> {
        let mut result = vec![];

        if all || DEBUG || self.poly.originalTriangle.delegate.gl & 4 == 4 {
            result.push(TheLine::new(
                self.poly.originalTriangle.origPoint1.clone(),
                self.poly.originalTriangle.origPoint2.clone(),
                self.poly.delegate.cols,
            ));
        }
        if all || DEBUG || self.poly.originalTriangle.delegate.gl & 1 == 1 {
            result.push(TheLine::new(
                self.poly.originalTriangle.origPoint2.clone(),
                self.poly.originalTriangle.origPoint3.clone(),
                self.poly.delegate.cols,
            ));
        }
        if all || DEBUG || self.poly.originalTriangle.delegate.gl & 2 == 2 {
            result.push(TheLine::new(
                self.poly.originalTriangle.origPoint3.clone(),
                self.poly.originalTriangle.origPoint1.clone(),
                self.poly.delegate.cols,
            ));
        }

        result
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

#[derive(Debug)]
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
        let mut last = 0.0;
        let RangeSet(ranges) = ranges;
        for range in ranges {
            match range {
                (Bound::Included(l1), Bound::Included(l2))
                | (Bound::Excluded(l1), Bound::Excluded(l2))
                | (Bound::Included(l1), Bound::Excluded(l2))
                | (Bound::Excluded(l1), Bound::Included(l2)) => {
                    if DEBUG && last < *l1 {
                        self.line(ctx, last, *l1, -MAX);
                    }
                    self.line(ctx, *l1, *l2, self.color);
                    last = *l2;
                }
                _ => {}
            }
        }
        if DEBUG && last < 1.0 {
            self.line(ctx, last, 1.0, -MAX);
        }
    }

    fn line(&self, ctx: &mut dyn DrawContext, l1: Float, l2: Float, color: Color) {
        let p1 = self.shape.a.mix(&self.shape.e, l1);
        let p2 = self.shape.a.mix(&self.shape.e, l2);

        ctx.line(p1.x, p1.y, p2.x, p2.y, color);
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
                for l in t.lines(false).iter() {
                    let bounds = l.shape.bounds();
                    let mut r: RangeSet<Float> = RangeSet::from_range(&(0.0..=1.0));
                    let mut candidates = tree.elements_intersecting(&bounds);
                    while let Some(next) = if r.is_empty() {
                        None
                    } else {
                        candidates.next()
                    } {
                        self.intersect(actx, &next, &l, &mut r);
                    }

                    l.draw(dctx, &r);
                }
            }
        }
    }

    fn clip(
        p1: Vector3,
        p2: Vector3,
        a: Vector3,
        b: Vector3,
        c: Vector3,
        ref_point: Vector3,
        ref_point_visible: bool,
    ) -> (Bound<Float>, Bound<Float>) {
        let normal = (a - b).kreuz(&c).normalize();

        let ref_dist = ref_point * normal;
        if ref_dist.abs() < epsilon0 {
            return (Bound::Unbounded, Bound::Unbounded);
        }

        let ref_dist = if !ref_point_visible {
            ref_dist.signum()
        } else {
            -ref_dist.signum()
        };

        //https://quickmath.com/webMathematica3/quickmath/equations/solve/advanced.jsp#c=solve_advancedsolveequations&v1=lx_1%2Bmx_2%253Dp%250Aly_1%2Bmy_2%253Dq%250Alz_1%2Bmz_2%253Dr%250Al%2Bm%253D1%250Apx%2Bqy%2Brz%253D0%250A&v2=l%250Am%250A%250A&v5=1&v6=p%250Aq%250Ar

        let p1n = p1 * normal;
        let p2n = p2 * normal;

        let d = p2n - p1n;

        if d.abs() < epsilon0 {
            (Bound::Unbounded, Bound::Unbounded)
        } else {
            let l = p2n / d;

            if l > 0.5 {
                if p2n.signum() == ref_dist {
                    // l == 0 visible
                    (Bound::Included(l), Bound::Unbounded)
                } else {
                    (Bound::Unbounded, Bound::Included(l))
                }
            } else {
                if p1n.signum() == ref_dist {
                    // l == 1 visible
                    (Bound::Unbounded, Bound::Included(l))
                } else {
                    (Bound::Included(l), Bound::Unbounded)
                }
            }
        }
    }

    fn intersect2(
        &self,
        actx: &AppContext,
        triangle: &TheTriangle,
        line: &TheLine,
        range: &mut RangeSet<Float>,
    ) {
        let p1 = line.p1.o;
        let p2 = line.p2.o;
        let t1 = triangle.poly.originalTriangle.origPoint1.o;
        let t2 = triangle.poly.originalTriangle.origPoint2.o;
        let t3 = triangle.poly.originalTriangle.origPoint3.o;
        let eye = actx.Auge;

        let reye = Self::clip(p1, p2, t1, t2, t3, eye, true);
        let rt1 = Self::clip(p1, p2, eye, t2, t3, t1, false);

        let mut remove = reye.intersect(&rt1);
        if let Some(r) = remove {
            let rt2 = Self::clip(p1, p2, t1, eye, t3, t2, false);
            remove = r.intersect(&rt2);
        }

        if let Some(r) = remove {
            let rt3 = Self::clip(p1, p2, t1, t2, eye, t3, false);
            remove = r.intersect(&rt3);
        }

        if let Some(r) = remove {
            range.remove(&r);
        }
    }

    fn intersect(
        &self,
        actx: &AppContext,
        triangle: &TheTriangle,
        line: &TheLine,
        range: &mut RangeSet<Float>,
    ) {
        let t1 = triangle.poly.originalTriangle.origPoint1.o;
        let t2 = triangle.poly.originalTriangle.origPoint2.o;
        let t3 = triangle.poly.originalTriangle.origPoint3.o;

        let nv = triangle.poly.originalTriangle.planeNorm;
        let pd = triangle.poly.originalTriangle.planeDist;

        let eye_dist = nv * actx.Auge - pd;

        if eye_dist.abs() < epsilon0 {
            // assume we are on the triangle and can see everything else
            return;
        }

        let p1_dist = nv * line.p1.o - pd;
        let p2_dist = nv * line.p2.o - pd;

        let p1_on_tri = p1_dist.abs() < epsilon0;
        let p2_on_tri = p2_dist.abs() < epsilon0;
        if p1_on_tri && p2_on_tri {
            // the lies on the triangle and is there fore visible
            if DEBUG {
                println!("p1_on_tri && p2_on_tri");
            }
            return;
        }

        let p1_visible =
            eye_dist < 0.0 && p1_dist < epsilon0 || eye_dist > 0.0 && p1_dist > -epsilon0;
        let p2_visible =
            eye_dist < 0.0 && p2_dist < epsilon0 || eye_dist > 0.0 && p2_dist > -epsilon0;

        if p1_visible && p2_visible {
            // both points of the line are on the same side of the triangle like the eye, so it is always visible even if it intersects
            if DEBUG {
                println!("p1_visible && p2_visible");
            }
            return;
        }

        if DEBUG {
            println!(
                "t {:?} {:?} {:?} l {:?} {:?}",
                t1, t2, t3, line.p1.o, line.p2.o
            );
            println!(
                "eye_dist {:?} p1_dist {:?} p2_dist {:?}",
                eye_dist, p1_dist, p2_dist
            );
        }

        if p1_visible != p2_visible {
            // range.remove(&(..));
        }

        range.remove(&Self::zeug(triangle, line));
    }

    fn zeug(triangle: &TheTriangle, line: &TheLine) -> (Bound<Float>, Bound<Float>) {
        if DEBUG {
            println!("<path style=\"fill:#fff;stroke:#000000;stroke-width: 0.01px;\" d=\"M {:?},{:?} {:?},{:?} {:?},{:?} Z\" />",
                     triangle.shape.p1.x,
                     -triangle.shape.p1.y,
                     triangle.shape.p2.x,
                     -triangle.shape.p2.y,
                     triangle.shape.p3.x,
                     -triangle.shape.p3.y);

            println!(
                "<path style=\"stroke:#000000;stroke-width: 0.01px;\" d=\"M {:?},{:?} {:?},{:?} \"/>",
                line.shape.a.x, -line.shape.a.y, line.shape.e.x, -line.shape.e.y
            );
        }

        let p1 = line.shape.a;
        let p2 = line.shape.e;

        let p1_contained = triangle.shape.contains(&p1);
        let p2_contained = triangle.shape.contains(&p2);

        if p1_contained && p2_contained {
            return if p1 != triangle.shape.p1 && p1 != triangle.shape.p2 && p1 != triangle.shape.p3
                || p2 != triangle.shape.p1 && p2 != triangle.shape.p2 && p2 != triangle.shape.p3
            {
                if DEBUG {
                    println!("contained 1");
                }
                (Bound::Included(0.0), Bound::Excluded(0.0))
            } else {
                if DEBUG {
                    println!("contained 2");
                }
                (Bound::Unbounded, Bound::Unbounded)
            };
        }

        let d21 = p2 - p1;
        let mut other_range = None;
        for triLine in triangle.lines(true).iter() {
            //https://quickmath.com/webMathematica3/quickmath/equations/solve/advanced.jsp#c=solve_advancedsolveequations&v1=lx_1%2Bmx_2%253Dp%250Aly_1%2Bmy_2%253Dq%250Al%2Bm%253D1%250Anx_3%2Box_4%253Dp%250Any_3%2Boy_4%253Dq%250An%2Bo%253D1%250A&v2=l%250Am%250An%250Ao%250Ap%250Aq%250A&v5=1
            let p3 = triLine.shape.a;
            let p4 = triLine.shape.e;

            let d43 = p4 - p3;

            let fx1 = p1.x * d43.y;
            let fy1 = p1.y * d43.x;

            let fx2 = p2.x * d43.y;
            let fy2 = p2.y * d43.x;

            let fx3 = p3.x * d21.y;
            let fy3 = p3.y * d21.x;

            let fx4 = p4.x * d21.y;
            let fy4 = p4.y * d21.x;

            let d = fx2 - fy2 + fy1 - fx1;

            let k21 = Matrix2::new(p1, p2).det2d();

            let n = -(fx4 - fy4 - k21) / d;
            let o = (fx3 - fy3 - k21) / d;

            if n > -epsilon0 && o > -epsilon0 {
                let k43 = Matrix2::new(p3, p4).det2d();
                let l = (fx2 - fy2 - k43) / d;
                match other_range {
                    Some(r) => {
                        if DEBUG {
                            println!("Some {:?} {:?}", l, r);
                        }
                        return (Bound::Excluded(l.min(r)), Bound::Excluded(l.max(r)));
                    }
                    None => {
                        other_range = Some(l);
                    }
                }
            }
        }

        if DEBUG {
            println!("noting found {:?}", other_range);
        }

        (Bound::Included(0.0), Bound::Excluded(0.0))
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
