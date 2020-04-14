use std::ops::Bound;

use crate::appcontext::*;
use crate::drawcontext::*;
use crate::float::*;
use crate::matrix::Matrix2;
use crate::quadtree::QuadTree;
use crate::range::RangeExtCopy;
use crate::rangeset::RangeSet;
use crate::shape::{Line, Rect, Shape, Triangle};
use crate::vec2::*;
use crate::vec3::*;

const DEBUG: bool = false;

#[derive(Debug)]
struct TheTriangle {
    shape: Triangle,
    plane_norm: Vector3,
    plane_dist: Float,
    o1: Vector3,
    o2: Vector3,
    o3: Vector3,
}

impl TheTriangle {
    fn new(poly: (usize, usize, usize), scene: &Scene, actx: &AppContext) -> TheTriangle {
        let (i1, i2, i3) = poly;

        let o1 = actx.scene_builder.points[i1];
        let o2 = actx.scene_builder.points[i2];
        let o3 = actx.scene_builder.points[i3];

        let plane_norm = (o2 - o1).cross(&(o3 - o1)).normalize();
        let plane_dist = plane_norm * o1;

        TheTriangle {
            shape: Triangle(scene.points[i1], scene.points[i2], scene.points[i3]),
            plane_norm,
            plane_dist,
            o1,
            o2,
            o3,
        }
    }

    fn lines(&self) -> Vec<Line> {
        let Triangle(p1, p2, p3) = self.shape;
        vec![Line(p1, p2), Line(p2, p3), Line(p3, p1)]
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

pub fn draw(ctx: &mut dyn DrawContext, line: &Line, ranges: &RangeSet<Float>, color: Color) {
    let mut last = 0.0;
    let RangeSet(ranges) = ranges;
    for range in ranges {
        match range {
            (Bound::Included(l1), Bound::Included(l2))
            | (Bound::Excluded(l1), Bound::Excluded(l2))
            | (Bound::Included(l1), Bound::Excluded(l2))
            | (Bound::Excluded(l1), Bound::Included(l2)) => {
                if DEBUG && last < *l1 {
                    draw_line_range(ctx, line, last, *l1, -MAX);
                }
                draw_line_range(ctx, line, *l1, *l2, color);
                last = *l2;
            }
            _ => {}
        }
    }
    if DEBUG && last < 1.0 {
        draw_line_range(ctx, line, last, 1.0, -MAX);
    }
}

fn draw_line_range(ctx: &mut dyn DrawContext, line: &Line, l1: Float, l2: Float, color: Color) {
    let Line(a, e) = line;
    let p1 = a.mix(&e, l1);
    let p2 = a.mix(&e, l2);

    ctx.line(p1.x, p1.y, p2.x, p2.y, color);
}

pub fn hidden_line(mut scene: Scene, dctx: &mut dyn DrawContext, actx: &AppContext) {
    let mut triangles = vec![];
    while let Some(poly) = scene.triangles.pop() {
        triangles.push(TheTriangle::new(poly, &scene, &actx));
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

        for (&(a, e), &color) in scene.lines.iter() {
            let line_shape = Line(scene.points[a], scene.points[e]);

            let bounds = line_shape.bounds();
            let mut r: RangeSet<Float> = RangeSet::from_range(&(0.0..=1.0));
            let mut candidates = tree.elements_intersecting(&bounds);
            while let Some(next) = if r.is_empty() {
                None
            } else {
                candidates.next()
            } {
                intersect(
                    actx,
                    &next,
                    &line_shape,
                    actx.scene_builder.points[a],
                    actx.scene_builder.points[e],
                    &mut r,
                );
            }

            draw(dctx, &line_shape, &r, color);
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
    let normal = (a - b).cross(&c).normalize();

    let ref_dist = ref_point * normal;
    if ref_dist.abs() < EPSILON0 {
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

    if d.abs() < EPSILON0 {
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
    actx: &AppContext,
    triangle: &TheTriangle,
    p1: Vector3,
    p2: Vector3,
    range: &mut RangeSet<Float>,
) {
    let t1 = triangle.o1;
    let t2 = triangle.o2;
    let t3 = triangle.o3;
    let eye = actx.eye;

    let reye = clip(p1, p2, t1, t2, t3, eye, true);
    let rt1 = clip(p1, p2, eye, t2, t3, t1, false);

    let mut remove = reye.intersect(&rt1);
    if let Some(r) = remove {
        let rt2 = clip(p1, p2, t1, eye, t3, t2, false);
        remove = r.intersect(&rt2);
    }

    if let Some(r) = remove {
        let rt3 = clip(p1, p2, t1, t2, eye, t3, false);
        remove = r.intersect(&rt3);
    }

    if let Some(r) = remove {
        range.remove(&r);
    }
}

fn intersect(
    actx: &AppContext,
    triangle: &TheTriangle,
    line_shape: &Line,
    o1: Vector3,
    o2: Vector3,
    range: &mut RangeSet<Float>,
) {
    let t1 = triangle.o1;
    let t2 = triangle.o2;
    let t3 = triangle.o3;

    let nv = triangle.plane_norm;
    let pd = triangle.plane_dist;

    let eye_dist = nv * actx.eye - pd;

    if eye_dist.abs() < EPSILON0 {
        // assume we are on the triangle and can see everything else
        return;
    }

    let p1_dist = nv * o1 - pd;
    let p2_dist = nv * o2 - pd;

    let p1_on_tri = p1_dist.abs() < EPSILON0;
    let p2_on_tri = p2_dist.abs() < EPSILON0;
    if p1_on_tri && p2_on_tri {
        // the lies on the triangle and is there fore visible
        if DEBUG {
            println!("p1_on_tri && p2_on_tri");
        }
        return;
    }

    let p1_visible = eye_dist < 0.0 && p1_dist < EPSILON0 || eye_dist > 0.0 && p1_dist > -EPSILON0;
    let p2_visible = eye_dist < 0.0 && p2_dist < EPSILON0 || eye_dist > 0.0 && p2_dist > -EPSILON0;

    if p1_visible && p2_visible {
        // both points of the line are on the same side of the triangle like the eye, so it is always visible even if it intersects
        if DEBUG {
            println!("p1_visible && p2_visible");
        }
        return;
    }

    if DEBUG {
        println!("t {:?} {:?} {:?} l {:?} {:?}", t1, t2, t3, o1, o2);
        println!(
            "eye_dist {:?} p1_dist {:?} p2_dist {:?}",
            eye_dist, p1_dist, p2_dist
        );
    }

    if p1_visible != p2_visible {
        // range.remove(&(..));
    }

    let Line(b1, b2) = *line_shape;

    if DEBUG {
        println!("<path style=\"fill:#fff;stroke:#000000;stroke-width: 0.01px;\" d=\"M {:?},{:?} {:?},{:?} {:?},{:?} Z\" />",
                     triangle.shape.0.x,
                     -triangle.shape.0.y,
                     triangle.shape.1.x,
                     -triangle.shape.1.y,
                     triangle.shape.2.x,
                     -triangle.shape.2.y);

        println!(
            "<path style=\"stroke:#000000;stroke-width: 0.01px;\" d=\"M {:?},{:?} {:?},{:?} \"/>",
            b1.x, -b1.y, b2.x, -b2.y
        );
    }

    let d21 = b2 - b1;
    let mut min = None;
    let mut max = None;
    for Line(p3, p4) in triangle.lines() {
        //https://quickmath.com/webMathematica3/quickmath/equations/solve/advanced.jsp#c=solve_advancedsolveequations&v1=lx_1%2Bmx_2%253Dp%250Aly_1%2Bmy_2%253Dq%250Al%2Bm%253D1%250Anx_3%2Box_4%253Dp%250Any_3%2Boy_4%253Dq%250An%2Bo%253D1%250A&v2=l%250Am%250An%250Ao%250Ap%250Aq%250A&v5=1

        let d43 = p4 - p3;

        let fx1 = b1.x * d43.y;
        let fy1 = b1.y * d43.x;

        let fx2 = b2.x * d43.y;
        let fy2 = b2.y * d43.x;

        let fx3 = p3.x * d21.y;
        let fy3 = p3.y * d21.x;

        let fx4 = p4.x * d21.y;
        let fy4 = p4.y * d21.x;

        let d = fx2 - fy2 + fy1 - fx1;

        let k21 = Matrix2(b1, b2).determinant();

        let n = -(fx4 - fy4 - k21) / d;
        let o = (fx3 - fy3 - k21) / d;

        if n > -EPSILON0 && o > -EPSILON0 {
            let k43 = Matrix2(p3, p4).determinant();
            let l = (fx2 - fy2 - k43) / d;

            match min {
                None => min = Some(l),
                Some(r) => min = Some(l.min(r)),
            }

            match max {
                None => max = Some(l),
                Some(r) => max = Some(l.max(r)),
            }
        }
    }

    if let (Some(l), Some(r)) = (min, max) {
        range.remove(&(Bound::Excluded(l), Bound::Excluded(r)));
    }
}
