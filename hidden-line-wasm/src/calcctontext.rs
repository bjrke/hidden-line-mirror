use crate::appcontext::*;
use crate::drawcontext::*;
use crate::dreidext::Scene3;
use crate::float::*;
use crate::matrix::Matrix2;
use crate::quadtree::QuadTree;
use crate::range::*;
use crate::rangeset::RangeSet;
use crate::shape::*;
use crate::vec2::*;
use crate::vec3::*;

#[derive(Debug)]
pub struct TreeTriangle {
    shape: Triangle,
    plane_norm: Vector3,
    plane_dist: Float,
    o1: Vector3,
    o2: Vector3,
    o3: Vector3,
}

impl TreeTriangle {
    #[inline]
    fn new(scene: &Scene3, rendered: &Scene2, i1: usize, i2: usize, i3: usize) -> TreeTriangle {
        let o1 = scene.points[i1];
        let o2 = scene.points[i2];
        let o3 = scene.points[i3];

        let plane_norm = (o2 - o1).cross(&(o3 - o1)).normalize();
        let plane_dist = plane_norm * o1;

        TreeTriangle {
            shape: Triangle::new(
                rendered.points[i1],
                rendered.points[i2],
                rendered.points[i3],
            ),
            plane_norm,
            plane_dist,
            o1,
            o2,
            o3,
        }
    }
}

impl Shape for TreeTriangle {
    #[inline]
    fn intersects(&self, r: &Rect) -> bool {
        self.shape.intersects(r)
    }

    #[inline]
    fn contains(&self, v: &Vector2) -> bool {
        self.shape.contains(v)
    }

    #[inline]
    fn bounds(&self) -> Rect {
        self.shape.bounds()
    }
}

pub fn create_tree(scene3: &Scene3, scene2: &Scene2) -> QuadTree<TreeTriangle> {
    let mut triangles = vec![];
    for poly in scene2.triangles.iter() {
        let (i1, i2, i3) = *poly;
        triangles.push(TreeTriangle::new(scene3, scene2, i1, i2, i3));
    }

    let screen = triangles
        .iter()
        .fold(None, |acc: Option<Rect>, t| {
            let bound = t.bounds();
            if let Some(existing) = acc {
                Some(existing.extend_rect(&bound))
            } else {
                Some(bound)
            }
        })
        .unwrap_or_else(|| Rect::new(0.0, 0.0));

    let mut tree = QuadTree::new(screen);
    for t in triangles {
        tree.insert(t.bounds(), t);
    }
    tree
}

pub fn hidden_line(
    tree: &QuadTree<TreeTriangle>,
    scene3: &Scene3,
    scene2: &Scene2,
    lines: &Vec<(usize, usize)>,
    color_context: &mut dyn ColorContext,
) -> (usize, usize) {
    let visited = std::cell::Cell::new(0usize);
    let mut emitted = 0usize;
    let mut ranges: RangeSet = RangeSet::new(vec![FloatRange::new(0.0, 1.0)]);
    for &(a, e) in lines.iter() {
        let pa = scene2.points[a];
        let pe = scene2.points[e];
        let line_shape = Line::new(pa, pe);

        let bounds = line_shape.bounds();
        ranges.reset();

        tree.elements_intersecting(&bounds, &mut |triangle| {
            intersect(
                &triangle,
                &line_shape,
                scene2.eye,
                scene3.points[a],
                scene3.points[e],
                &mut ranges,
            );
            ranges.is_empty()
        });

        emitted += ranges.ranges.len();
        for &FloatRange { start: l1, end: l2 } in ranges.ranges.iter() {
            let start = pa.mix(&pe, l1);
            let end = pa.mix(&pe, l2);
            color_context.line(start, end);
        }
    }
    (visited.get(), emitted)
}

fn clip(
    p1: Vector3,
    p2: Vector3,
    v1: Vector3,
    v2: Vector3,
    v3: Vector3,
    ref_point: Vector3,
    ref_point_visible: bool,
) -> FloatRange {
    let normal = (v1 - v2).cross(&v3).normalize();

    let ref_dist = ref_point * normal;
    if ref_dist.abs() < EPSILON0 {
        return FloatRange::new(MIN, MAX);
    }

    let ref_dist = if !ref_point_visible {
        ref_dist.sign()
    } else {
        !ref_dist.sign()
    };

    //https://quickmath.com/webMathematica3/quickmath/equations/solve/advanced.jsp#c=solve_advancedsolveequations&v1=lx_1%2Bmx_2%253Dp%250Aly_1%2Bmy_2%253Dq%250Alz_1%2Bmz_2%253Dr%250Al%2Bm%253D1%250Apx%2Bqy%2Brz%253D0%250A&v2=l%250Am%250A%250A&v5=1&v6=p%250Aq%250Ar

    let p1n = p1 * normal;
    let p2n = p2 * normal;

    let d = p2n - p1n;

    if d.abs() < EPSILON0 {
        FloatRange::new(MIN, MAX)
    } else {
        let l = p2n / d;

        if l > 0.5 {
            if p2n.sign() == ref_dist {
                // l == 0 visible
                FloatRange::new(l, MAX)
            } else {
                FloatRange::new(MIN, l)
            }
        } else if p1n.sign() == ref_dist {
            // l == 1 visible
            FloatRange::new(MIN, l)
        } else {
            FloatRange::new(l, MAX)
        }
    }
}

#[allow(dead_code)]
fn intersect2(
    eye: Vector3,
    triangle: &TreeTriangle,
    p1: Vector3,
    p2: Vector3,
    range: &mut RangeSet,
) {
    let t1 = triangle.o1;
    let t2 = triangle.o2;
    let t3 = triangle.o3;

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

#[inline]
fn intersect(
    triangle: &TreeTriangle,
    line_shape: &Line,
    eye: Vector3,
    o1: Vector3,
    o2: Vector3,
    range: &mut RangeSet,
) {
    let nv = triangle.plane_norm;
    let pd = triangle.plane_dist;

    let eye_dist = nv * eye - pd;

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
        return;
    }

    let p1_visible = eye_dist < 0.0 && p1_dist < EPSILON0 || eye_dist > 0.0 && p1_dist > -EPSILON0;
    let p2_visible = eye_dist < 0.0 && p2_dist < EPSILON0 || eye_dist > 0.0 && p2_dist > -EPSILON0;

    if p1_visible && p2_visible {
        // both points of the line are on the same side of the triangle like the eye, so it is always visible even if it intersects
        return;
    }

    if p1_visible != p2_visible {
        // range.remove(&(..));
    }

    let (b1, b2) = (*line_shape).into();

    let d21 = b2 - b1;
    let mut min = None;
    let mut max = None;
    let shape = triangle.shape;
    for (p3, p4) in [
        (shape.p1, shape.p2),
        (shape.p2, shape.p3),
        (shape.p3, shape.p1),
    ] {
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

        let divisor = fx2 - fy2 + fy1 - fx1;

        let k21 = Matrix2::new(b1, b2).determinant();

        let nn = -(fx4 - fy4 - k21) / divisor;
        let oo = (fx3 - fy3 - k21) / divisor;

        if nn > -EPSILON0 && oo > -EPSILON0 {
            let k43 = Matrix2::new(p3, p4).determinant();
            let l1 = (fx2 - fy2 - k43) / divisor;

            match min {
                None => min = Some(l1),
                Some(l2) => min = Some(l1.min(l2)),
            }

            match max {
                None => max = Some(l1),
                Some(l2) => max = Some(l1.max(l2)),
            }
        }
    }

    if let (Some(l), Some(r)) = (min, max) {
        range.remove(&FloatRange::new(l, r));
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::dreidext::SceneTriangle;
    use std::collections::HashMap;

    const EYE: Vector3 = Vector3::new(0.5, 0.5, 2.0);

    fn test_triangle() -> TreeTriangle {
        let scene3 = Scene3 {
            points: vec![
                Vector3::new(0.0, 0.0, 0.0),
                Vector3::new(1.0, 0.0, 0.0),
                Vector3::new(0.0, 1.0, 0.0),
            ],
            triangles: vec![],
        };
        let scene2 = Scene2 {
            points: vec![
                Vector2::new(0.0, 0.0),
                Vector2::new(1.0, 0.0),
                Vector2::new(0.0, 1.0),
            ],
            triangles: vec![(0, 1, 2)],
            lines: HashMap::new(),
            eye: EYE,
        };
        TreeTriangle::new(&scene3, &scene2, 0, 1, 2)
    }

    fn full_range() -> RangeSet {
        RangeSet::new(vec![FloatRange::new(0.0, 1.0)])
    }

    #[test]
    fn clip_returns_full_range_when_ref_point_on_plane() {
        let v1 = Vector3::new(1.0, 0.0, 0.0);
        let v2 = Vector3::new(0.0, 1.0, 0.0);
        let v3 = Vector3::new(1.0, 1.0, 0.0);
        let p1 = Vector3::new(0.0, 0.0, 5.0);
        let p2 = Vector3::new(0.0, 0.0, -1.0);

        assert_eq!(
            clip(p1, p2, v1, v2, v3, Vector3::new(0.0, 0.0, 0.0), true),
            FloatRange::new(MIN, MAX)
        );
    }

    #[test]
    fn clip_returns_full_range_when_points_parallel() {
        let v1 = Vector3::new(1.0, 0.0, 0.0);
        let v2 = Vector3::new(0.0, 1.0, 0.0);
        let v3 = Vector3::new(1.0, 1.0, 0.0);
        let p1 = Vector3::new(0.0, 0.0, 1.0);
        let p2 = Vector3::new(0.0, 0.0, 1.0);

        assert_eq!(
            clip(p1, p2, v1, v2, v3, Vector3::new(0.0, 0.0, 1.0), true),
            FloatRange::new(MIN, MAX)
        );
    }

    #[test]
    fn clip_visible_ref_point() {
        let v1 = Vector3::new(1.0, 0.0, 0.0);
        let v2 = Vector3::new(0.0, 1.0, 0.0);
        let v3 = Vector3::new(1.0, 1.0, 0.0);
        let p1 = Vector3::new(0.0, 0.0, 5.0);
        let p2 = Vector3::new(0.0, 0.0, -1.0);

        assert_eq!(
            clip(p1, p2, v1, v2, v3, Vector3::new(0.0, 0.0, 1.0), true),
            FloatRange::new(0.16666667, MAX)
        );
    }

    #[test]
    fn clip_hidden_ref_point() {
        let v1 = Vector3::new(1.0, 0.0, 0.0);
        let v2 = Vector3::new(0.0, 1.0, 0.0);
        let v3 = Vector3::new(1.0, 1.0, 0.0);
        let p1 = Vector3::new(0.0, 0.0, 5.0);
        let p2 = Vector3::new(0.0, 0.0, -1.0);

        assert_eq!(
            clip(p1, p2, v1, v2, v3, Vector3::new(0.0, 0.0, 1.0), false),
            FloatRange::new(MIN, 0.16666667)
        );
    }

    #[test]
    fn intersect_keeps_range_when_line_is_on_eye_side() {
        let triangle = test_triangle();
        let line_shape = Line::new(Vector2::new(0.25, 0.5), Vector2::new(0.25, 1.0));

        let mut range = full_range();
        intersect(
            &triangle,
            &line_shape,
            EYE,
            Vector3::new(0.25, 0.25, 0.5),
            Vector3::new(0.25, 0.25, 1.0),
            &mut range,
        );

        assert_eq!(range.ranges, vec![FloatRange::new(0.0, 1.0)]);
    }

    #[test]
    fn intersect_keeps_range_when_eye_is_on_plane() {
        let triangle = test_triangle();
        let line_shape = Line::new(Vector2::new(0.25, -1.0), Vector2::new(0.25, 1.0));

        let mut range = full_range();
        intersect(
            &triangle,
            &line_shape,
            Vector3::new(0.5, 0.5, 0.0),
            Vector3::new(0.25, 0.25, -1.0),
            Vector3::new(0.25, 0.25, 1.0),
            &mut range,
        );

        assert_eq!(range.ranges, vec![FloatRange::new(0.0, 1.0)]);
    }

    #[test]
    fn intersect_keeps_range_when_line_lies_on_triangle() {
        let triangle = test_triangle();
        let line_shape = Line::new(Vector2::new(0.25, 0.25), Vector2::new(0.75, 0.25));

        let mut range = full_range();
        intersect(
            &triangle,
            &line_shape,
            EYE,
            Vector3::new(0.25, 0.25, 0.0),
            Vector3::new(0.75, 0.25, 0.0),
            &mut range,
        );

        assert_eq!(range.ranges, vec![FloatRange::new(0.0, 1.0)]);
    }

    #[test]
    fn intersect_removes_occluded_middle() {
        let triangle = test_triangle();
        let line_shape = Line::new(Vector2::new(0.25, -1.0), Vector2::new(0.25, 1.0));

        let mut range = full_range();
        intersect(
            &triangle,
            &line_shape,
            EYE,
            Vector3::new(0.25, 0.25, -1.0),
            Vector3::new(0.25, 0.25, 1.0),
            &mut range,
        );

        assert_eq!(
            range.ranges,
            vec![FloatRange::new(0.0, 0.125), FloatRange::new(0.5, 1.0)]
        );
    }

    struct CollectContext {
        lines: Vec<(Vector2, Vector2)>,
    }

    impl ColorContext for CollectContext {
        fn line(&mut self, p1: Vector2, p2: Vector2) {
            self.lines.push((p1, p2));
        }
    }

    #[test]
    fn hidden_line_occludes_middle_of_crossing_line() {
        let scene3 = Scene3 {
            points: vec![
                Vector3::new(0.0, 0.0, 0.0),
                Vector3::new(1.0, 0.0, 0.0),
                Vector3::new(0.0, 1.0, 0.0),
                Vector3::new(0.25, 0.25, -1.0),
                Vector3::new(0.25, 0.25, 1.0),
            ],
            triangles: vec![SceneTriangle {
                p1: 0,
                p2: 1,
                p3: 2,
                lset: 0,
            }],
        };
        let scene2 = Scene2 {
            points: vec![
                Vector2::new(0.0, 0.0),
                Vector2::new(1.0, 0.0),
                Vector2::new(0.0, 1.0),
                Vector2::new(0.25, -1.0),
                Vector2::new(0.25, 1.0),
            ],
            triangles: vec![(0, 1, 2)],
            lines: HashMap::new(),
            eye: EYE,
        };

        let tree = create_tree(&scene3, &scene2);
        let mut ctx = CollectContext { lines: vec![] };
        hidden_line(&tree, &scene3, &scene2, &vec![(3, 4)], &mut ctx);

        assert_eq!(
            ctx.lines,
            vec![
                (Vector2::new(0.25, 1.0), Vector2::new(0.25, 0.75)),
                (Vector2::new(0.25, 0.0), Vector2::new(0.25, -1.0)),
            ]
        );
    }
}
