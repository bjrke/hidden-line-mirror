use crate::float::*;
use crate::range::*;
use crate::vec2::*;
use std::ops::RangeInclusive;
use std::ops::{Range, RangeBounds};

#[derive(Clone, Debug, PartialEq)]
pub struct Rect {
    pub x: RangeInclusive<Float>,
    pub y: RangeInclusive<Float>,
}

pub trait Shape {
    fn intersects(&self, r: &Rect) -> bool;

    fn contains(&self, v: &Vector2) -> bool {
        self.intersects(&Rect::from_vector(v))
    }

    fn bounds(&self) -> Rect;

    #[inline]
    fn bounds_intersect(&self, s: &dyn Shape) -> bool {
        self.bounds().intersects(&s.bounds())
    }

    #[inline]
    fn bounds_contains(&self, v: &Vector2) -> bool {
        self.bounds().contains(v)
    }
}

impl Rect {
    #[inline]
    pub fn new(x: Float, y: Float) -> Rect {
        Rect { x: x..=x, y: y..=y }
    }

    #[inline]
    pub fn from_vector(v: &Vector2) -> Rect {
        Rect::new(v.x, v.y)
    }

    #[inline]
    pub fn extend(&self, x: Float, y: Float) -> Rect {
        self.extend_rect(&Rect::new(x, y))
    }

    #[inline]
    pub fn extend_rect(&self, rect: &Rect) -> Rect {
        Rect {
            x: self.x.start().min(*rect.x.start())..=self.x.end().max(*rect.x.end()),
            y: self.y.start().min(*rect.y.start())..=self.y.end().max(*rect.y.end()),
        }
    }

    #[inline]
    pub fn extend_vector(&self, v: &Vector2) -> Rect {
        self.extend_rect(&Rect::from_vector(v))
    }

    #[inline]
    pub fn top_left(&self) -> Vector2 {
        Vector2(*self.x.start(), *self.y.end())
    }

    #[inline]
    pub fn top_right(&self) -> Vector2 {
        Vector2(*self.x.end(), *self.y.end())
    }

    #[inline]
    pub fn bottom_left(&self) -> Vector2 {
        Vector2(*self.x.start(), *self.y.start())
    }

    #[inline]
    pub fn bottom_right(&self) -> Vector2 {
        Vector2(*self.x.end(), *self.y.start())
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.x.is_empty() || self.y.is_empty()
    }

    #[inline]
    pub fn contains_rect(&self, r: &Rect) -> bool {
        self.x.contains_range(&r.x) && self.y.contains_range(&r.y)
    }
}

impl Shape for Rect {
    #[inline]
    fn intersects(&self, r: &Rect) -> bool {
        self.x.range_overlap(&r.x) && self.y.range_overlap(&r.y)
    }

    #[inline]
    fn contains(&self, v: &Vector2) -> bool {
        self.x.contains(&v.x) && self.y.contains(&v.y)
    }

    #[inline]
    fn bounds(&self) -> Rect {
        self.clone()
    }
}

impl Shape for Vector2 {
    fn intersects(&self, r: &Rect) -> bool {
        r.contains(self)
    }

    fn contains(&self, v: &Vector2) -> bool {
        self.x == v.x && self.y == v.y
    }

    fn bounds(&self) -> Rect {
        Rect::from_vector(self)
    }
}

fn lineIntersect(a1: Vector2, e1: Vector2, a2: Vector2, e2: Vector2) -> Vector2 {
    let w2 = e2.x - a2.x;
    let h2 = e2.y - a2.y;
    let w1 = e1.x - a1.x;
    let h1 = e1.y - a1.y;

    let divisor = w1 * h2 - h1 * w2;

    let q1 = a1.x * e1.y - a1.y * e1.x;
    let q2 = a2.x * e2.y - a2.y * e2.x;

    let x = w1 * q2 - w2 * q1;
    let y = h1 * q2 - h2 * q1;

    Vector2 {
        x: x / divisor,
        y: y / divisor,
    }
}

#[inline]
fn lineRectBorder<R: RangeBounds<Float>>(
    x1: Float,
    y1: Float,
    x2: Float,
    y2: Float,
    x: &Float,
    y_range: &R,
) -> bool {
    let divisor = x2 - x1;
    let s = x2 - x;
    if s == divisor {
        y_range.contains(&y2)
    } else if s == 0.0 {
        y_range.contains(&y1)
    } else {
        s.signum() == divisor.signum() && s.abs() < divisor.abs() && {
            let t = divisor - s;
            let y = (s * y1 + t * y2) / divisor;
            y_range.contains(&y)
        }
    }
}

#[inline]
fn lineX<R: RangeBounds<Float>>(a: &Vector2, e: &Vector2, x: &Float, y_range: &R) -> bool {
    lineRectBorder(a.x, a.y, e.x, e.y, x, y_range)
}

#[inline]
fn lineY<R: RangeBounds<Float>>(a: &Vector2, e: &Vector2, y: &Float, x_range: &R) -> bool {
    lineRectBorder(a.y, a.x, e.y, e.x, y, x_range)
}

/// warning this method should be used only after r.contains(a) and r.contains(e) check
#[inline]
fn lineRect(a: &Vector2, e: &Vector2, r: &&Rect) -> bool {
    lineY(a, e, r.y.start(), &r.x)
        || lineY(a, e, r.y.end(), &r.x)
        || lineX(a, e, r.x.start(), &r.y)
        || lineX(a, e, r.x.end(), &r.y)
}

#[derive(Clone, Debug)]
pub struct Line {
    pub a: Vector2,
    pub e: Vector2,
}

impl Line {
    #[inline]
    pub fn new(a: Vector2, e: Vector2) -> Line {
        Line { a, e }
    }
}

#[inline]
fn epsilon_value(f: Float) -> Range<Float> {
    Range {
        start: f - epsilon0,
        end: f + epsilon0,
    }
}

#[inline]
fn epsilon_range(r: &Range<Float>) -> Range<Float> {
    Range {
        start: r.start - epsilon0,
        end: r.end + epsilon0,
    }
}

impl Shape for Line {
    fn intersects(&self, r: &Rect) -> bool {
        self.bounds_intersect(r) && {
            r.contains(&self.a) || r.contains(&self.e) || lineRect(&self.a, &self.e, &r)
        }
    }

    fn contains(&self, v: &Vector2) -> bool {
        self.bounds_contains(v) && {
            lineX(&self.a, &self.e, &v.x, &epsilon_value(v.y))
                || lineY(&self.a, &self.e, &v.y, &epsilon_value(v.x))
        }
    }

    #[inline]
    fn bounds(&self) -> Rect {
        Rect::from_vector(&self.a).extend_vector(&self.e)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Triangle {
    pub p1: Vector2,
    pub p2: Vector2,
    pub p3: Vector2,
}

#[inline]
fn sign(p: &Vector2, a: &Vector2, e: &Vector2) -> Float {
    ((p.x - e.x) * (a.y - e.y) - (a.x - e.x) * (p.y - e.y)).signum()
}

impl Triangle {
    #[inline]
    pub fn new(p1: &Vector2, p2: &Vector2, p3: &Vector2) -> Triangle {
        Triangle {
            p1: *p1,
            p2: *p2,
            p3: *p3,
        }
    }
}

impl Shape for Triangle {
    fn intersects(&self, r: &Rect) -> bool {
        self.bounds_intersect(r) && {
            r.contains(&self.p1)
                || r.contains(&self.p2)
                || r.contains(&self.p3)
                || lineRect(&self.p1, &self.p2, &r)
                || lineRect(&self.p2, &self.p3, &r)
                || lineRect(&self.p3, &self.p1, &r)
        }
    }

    #[inline]
    fn contains(&self, v: &Vector2) -> bool {
        self.bounds_contains(v) && {
            let d1 = sign(v, &self.p1, &self.p2);
            d1 == sign(v, &self.p2, &self.p3) && d1 == sign(v, &self.p3, &self.p1)
        }
    }

    #[inline]
    fn bounds(&self) -> Rect {
        Rect::from_vector(&self.p1)
            .extend_vector(&self.p2)
            .extend_vector(&self.p3)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn rect_should_contain_vector() {
        assert!(Rect::new(2.0, 3.0)
            .extend(4.0, 6.0)
            .contains(&Vector2(2.5, 5.0)));
    }

    #[test]
    fn rect_should_contain_top_left() {
        assert!(Rect::new(2.0, 3.0)
            .extend(4.0, 6.0)
            .contains(&Vector2(2.0, 3.0)));
    }

    #[test]
    fn rect_should_contain_bottom_right() {
        assert!(Rect::new(2.0, 3.0)
            .extend(4.0, 6.0)
            .contains(&Vector2(4.0, 6.0)));
    }

    #[test]
    fn rect_top_left() {
        assert_eq!(
            Rect::new(2.0, 3.0).extend(4.0, 6.0).top_left(),
            Vector2(2.0, 6.0)
        );
    }

    #[test]
    fn rect_top_right() {
        assert_eq!(
            Rect::new(2.0, 3.0).extend(4.0, 6.0).top_right(),
            Vector2(4.0, 6.0)
        );
    }

    #[test]
    fn rect_bottom_left() {
        assert_eq!(
            Rect::new(2.0, 3.0).extend(4.0, 6.0).bottom_left(),
            Vector2(2.0, 3.0)
        );
    }

    #[test]
    fn rect_bottom_right() {
        assert_eq!(
            Rect::new(2.0, 3.0).extend(4.0, 6.0).bottom_right(),
            Vector2(4.0, 3.0)
        );
    }

    #[test]
    fn rect_should_intersect() {
        assert!(Rect::new(2.0, 3.5)
            .extend(4.0, 6.0)
            .intersects(&Rect::new(1.0, 2.5).extend(3.0, 4.0)));
    }

    #[test]
    fn rect_should_not_intersect_y() {
        assert!(!Rect::new(2.0, 3.5)
            .extend(4.0, 6.0)
            .intersects(&Rect::new(1.0, 2.5).extend(1.5, 4.0)));
    }

    #[test]
    fn rect_should_not_intersect_x() {
        assert!(!Rect::new(2.0, 3.5)
            .extend(4.0, 6.0)
            .intersects(&Rect::new(1.0, 2.5).extend(3.0, 3.0)));
    }

    #[test]
    fn triangle_should_contain() {
        let triangle = Triangle::new(&Vector2(1.0, 1.0), &Vector2(6.0, 2.0), &Vector2(4.0, 4.0));

        assert!(triangle.contains(&Vector2(3.0, 3.0)))
    }

    #[test]
    fn triangle_should_contain_point_in_rect() {
        let triangle = Triangle::new(&Vector2(1.0, 1.0), &Vector2(6.0, 2.0), &Vector2(4.0, 4.0));

        let v = Vector2(1.1, 3.9);
        assert!(triangle.bounds_contains(&v));
        assert!(!triangle.contains(&v))
    }

    #[test]
    fn triangle_should_contain_out_of_rect() {
        let triangle = Triangle::new(&Vector2(1.0, 1.0), &Vector2(6.0, 2.0), &Vector2(4.0, 4.0));

        let v = Vector2(-3.0, -3.0);
        assert!(!triangle.bounds_contains(&v));
        assert!(!triangle.contains(&v))
    }

    #[test]
    fn triangle_should_have_correct_bound() {
        let triangle = Triangle::new(&Vector2(1.0, 1.0), &Vector2(6.0, 2.0), &Vector2(4.0, 4.0));

        assert_eq!(
            triangle.bounds(),
            Rect {
                x: 1.0..=6.0,
                y: 1.0..=4.0
            }
        )
    }

    #[test]
    fn line_should_have_correct_bounds() {
        let line = Line::new(Vector2(1.0, 1.0), Vector2(6.0, 2.0));
        assert_eq!(
            line.bounds(),
            Rect {
                x: 1.0..=6.0,
                y: 1.0..=2.0
            }
        )
    }
}
