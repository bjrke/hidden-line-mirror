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

#[inline]
fn line_rect_border<R: RangeBounds<Float>>(
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
fn line_x<R: RangeBounds<Float>>(a: &Vector2, e: &Vector2, x: &Float, y_range: &R) -> bool {
    line_rect_border(a.x, a.y, e.x, e.y, x, y_range)
}

#[inline]
fn line_y<R: RangeBounds<Float>>(a: &Vector2, e: &Vector2, y: &Float, x_range: &R) -> bool {
    line_rect_border(a.y, a.x, e.y, e.x, y, x_range)
}

/// warning this method should be used only after r.contains(a) and r.contains(e) check
#[inline]
fn line_rect(a: &Vector2, e: &Vector2, r: &&Rect) -> bool {
    line_y(a, e, r.y.start(), &r.x)
        || line_y(a, e, r.y.end(), &r.x)
        || line_x(a, e, r.x.start(), &r.y)
        || line_x(a, e, r.x.end(), &r.y)
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
        start: f - EPSILON0,
        end: f + EPSILON0,
    }
}

#[inline]
fn epsilon_range(r: &Range<Float>) -> Range<Float> {
    Range {
        start: r.start - EPSILON0,
        end: r.end + EPSILON0,
    }
}

impl Shape for Line {
    fn intersects(&self, r: &Rect) -> bool {
        self.bounds_intersect(r) && {
            r.contains(&self.a) || r.contains(&self.e) || line_rect(&self.a, &self.e, &r)
        }
    }

    fn contains(&self, v: &Vector2) -> bool {
        self.bounds_contains(v) && {
            line_x(&self.a, &self.e, &v.x, &epsilon_value(v.y))
                || line_y(&self.a, &self.e, &v.y, &epsilon_value(v.x))
        }
    }

    #[inline]
    fn bounds(&self) -> Rect {
        Rect::from_vector(&self.a).extend_vector(&self.e)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Triangle(pub Vector2, pub Vector2, pub Vector2);

#[inline]
fn sign(p: &Vector2, a: &Vector2, e: &Vector2) -> Float {
    ((p.x - e.x) * (a.y - e.y) - (a.x - e.x) * (p.y - e.y)).signum()
}

impl Triangle {
    #[inline]
    pub fn new(p1: &Vector2, p2: &Vector2, p3: &Vector2) -> Triangle {
        Triangle(*p1, *p2, *p3)
    }
}

impl Shape for Triangle {
    fn intersects(&self, r: &Rect) -> bool {
        self.bounds_intersect(r) && {
            r.contains(&self.0)
                || r.contains(&self.1)
                || r.contains(&self.2)
                || line_rect(&self.0, &self.1, &r)
                || line_rect(&self.1, &self.2, &r)
                || line_rect(&self.2, &self.0, &r)
        }
    }

    #[inline]
    fn contains(&self, v: &Vector2) -> bool {
        self.bounds_contains(v) && {
            let Triangle(p1, p2, p3) = self;
            let d1 = sign(v, &p1, &p2);
            d1 == sign(v, &p2, &p3) && d1 == sign(v, &p3, &p1)
        }
    }

    #[inline]
    fn bounds(&self) -> Rect {
        let Triangle(p1, p2, p3) = self;
        Rect::from_vector(&p1).extend_vector(&p2).extend_vector(&p3)
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
