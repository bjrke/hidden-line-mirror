use crate::float::*;
use crate::range::*;
use crate::vec2::*;
use std::ops::Range;

pub struct Rect {
    pub x: Range<Float>,
    pub y: Range<Float>,
}

pub trait Shape {
    fn contains(&self, v: &Vector2) -> bool;

    fn intersects(&self, r: &Rect) -> bool;

    fn bounds(&self) -> &Rect;

    #[inline]
    fn bounds_intersect(&self, s: &Shape) -> bool {
        self.bounds().intersects(s.bounds())
    }

    #[inline]
    fn bounds_contains(&self, v: &Vector2) -> bool {
        self.bounds().bounds_contains(v)
    }
}

impl Rect {
    #[inline]
    pub fn new(x: Float, y: Float) -> Rect {
        Rect {
            x: Range { start: x, end: x },
            y: Range { start: y, end: y },
        }
    }

    #[inline]
    pub fn from_vector(v: &Vector2) -> Rect {
        Rect::new(v.x, v.y)
    }

    #[inline]
    pub fn extend(&self, x: Float, y: Float) -> Rect {
        Rect {
            x: Range {
                start: self.x.start.min(x),
                end: self.x.end.max(x),
            },
            y: Range {
                start: self.y.start.min(y),
                end: self.y.end.max(y),
            },
        }
    }

    #[inline]
    pub fn extend_vector(&self, v: &Vector2) -> Rect {
        self.extend(v.x, v.y)
    }

    #[inline]
    pub fn top_left(&self) -> Vector2 {
        Vector2::new(self.x.start, self.y.start)
    }

    #[inline]
    pub fn top_right(&self) -> Vector2 {
        Vector2::new(self.x.start, self.y.end)
    }

    #[inline]
    pub fn bottom_left(&self) -> Vector2 {
        Vector2::new(self.x.end, self.y.start)
    }

    #[inline]
    pub fn bottom_right(&self) -> Vector2 {
        Vector2::new(self.x.end, self.y.end)
    }
}

impl Shape for Rect {
    #[inline]
    fn contains(&self, v: &Vector2) -> bool {
        self.x.contains(&v.x) && self.y.contains(&v.y)
    }

    #[inline]
    fn intersects(&self, r: &Rect) -> bool {
        self.x.range_overlap(&r.x) || self.y.range_overlap(&r.y)
    }

    #[inline]
    fn bounds(&self) -> &Rect {
        &self
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
fn lineRectBorder(x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y_range: &Range<f32>) -> bool {
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
fn lineX(a: &Vector2, e: &Vector2, x: Float, y_range: &Range<Float>) -> bool {
    lineRectBorder(a.x, a.y, e.x, e.y, x, y_range)
}

#[inline]
fn lineY(a: &Vector2, e: &Vector2, y: Float, x_range: &Range<Float>) -> bool {
    lineRectBorder(a.y, a.x, e.y, e.x, y, x_range)
}

/// warning this method should be used only after r.contains(a) and r.contains(e) check
#[inline]
fn lineRect(a: &Vector2, e: &Vector2, r: &&Rect) -> bool {
    lineY(a, e, r.y.start, &r.x)
        || lineY(a, e, r.y.end, &r.x)
        || lineX(a, e, r.x.start, &r.y)
        || lineX(a, e, r.x.end, &r.y)
}

struct Line {
    a: Vector2,
    e: Vector2,
    bounds: Rect,
}

impl Line {
    #[inline]
    fn new(a: Vector2, e: Vector2) -> Line {
        Line {
            a,
            e,
            bounds: Rect::from_vector(&a).extend_vector(&e),
        }
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
    fn contains(&self, v: &Vector2) -> bool {
        self.bounds_contains(v) && {
            lineX(&self.a, &self.e, v.x, &epsilon_value(v.y))
                || lineY(&self.a, &self.e, v.y, &epsilon_value(v.x))
        }
    }

    fn intersects(&self, r: &Rect) -> bool {
        self.bounds_intersect(r) && {
            r.contains(&self.a) || r.contains(&self.e) || lineRect(&self.a, &self.e, &r)
        }
    }

    #[inline]
    fn bounds(&self) -> &Rect {
        &self.bounds
    }
}

struct Triangle {
    p1: Vector2,
    p2: Vector2,
    p3: Vector2,
    bounds: Rect,
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
            bounds: Rect::from_vector(p1).extend_vector(p2).extend_vector(p3),
        }
    }
}

impl Shape for Triangle {
    #[inline]
    fn contains(&self, v: &Vector2) -> bool {
        self.bounds_contains(v) && {
            let d1 = sign(v, &self.p1, &self.p2);
            d1 == sign(v, &self.p2, &self.p3) && d1 == sign(v, &self.p3, &self.p1)
        }
    }

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
    fn bounds(&self) -> &Rect {
        &self.bounds
    }
}
