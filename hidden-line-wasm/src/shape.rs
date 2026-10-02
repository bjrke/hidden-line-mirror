use crate::float::*;
use crate::range::*;
use crate::vec2::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Rect {
    pub x: FloatRange,
    pub y: FloatRange,
}

pub trait Shape {
    fn intersects(&self, r: &Rect) -> bool;

    #[inline]
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
        Rect {
            x: FloatRange::new(x, x),
            y: FloatRange::new(y, y),
        }
    }

    #[inline]
    pub fn from_vector(v: &Vector2) -> Rect {
        let (x, y) = (*v).into();
        Rect::new(x, y)
    }

    #[inline]
    pub fn extend(&self, x: Float, y: Float) -> Rect {
        Rect {
            x: self.x.extend(x),
            y: self.y.extend(y),
        }
    }

    #[inline]
    pub fn extend_rect(&self, rect: &Rect) -> Rect {
        Rect {
            x: self.x.extend_range(&rect.x),
            y: self.y.extend_range(&rect.y),
        }
    }

    #[inline]
    pub fn extend_vector(&self, v: &Vector2) -> Rect {
        let (x, y) = (*v).into();
        self.extend(x, y)
    }

    #[inline]
    pub fn top_left(&self) -> Vector2 {
        let Rect { x, y } = *self;
        Vector2::new(x.start, y.end)
    }

    #[inline]
    pub fn top_right(&self) -> Vector2 {
        let Rect { x, y } = *self;
        Vector2::new(x.end, y.end)
    }

    #[inline]
    pub fn bottom_left(&self) -> Vector2 {
        let Rect { x, y } = *self;
        Vector2::new(x.start, y.start)
    }

    #[inline]
    pub fn bottom_right(&self) -> Vector2 {
        let Rect { x, y } = *self;
        Vector2::new(x.end, y.start)
    }

    #[inline]
    pub fn contains_rect(&self, r: &Rect) -> bool {
        self.x.contains_range(&r.x) && self.y.contains_range(&r.y)
    }

    /// warning this method should be used only after r.contains(a) and r.contains(e) check
    #[inline]
    fn line_rect(&self, a: &Vector2, e: &Vector2) -> bool {
        let (xa, xe) = self.x.into();
        let (ya, ye) = self.y.into();
        self.x.line_y(a, e, ya)
            || self.x.line_y(a, e, ye)
            || self.y.line_x(a, e, xa)
            || self.y.line_x(a, e, xe)
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
        *self
    }
}

impl Shape for Vector2 {
    #[inline]
    fn intersects(&self, r: &Rect) -> bool {
        r.contains(self)
    }

    #[inline]
    fn contains(&self, v: &Vector2) -> bool {
        self == v
    }

    #[inline]
    fn bounds(&self) -> Rect {
        Rect::from_vector(self)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Line {
    pub a: Vector2,
    pub e: Vector2,
}

impl Line {
    #[inline]
    pub const fn new(a: Vector2, e: Vector2) -> Line {
        Line { a, e }
    }
}

impl From<(Vector2, Vector2)> for Line {
    #[inline]
    fn from((a, e): (Vector2, Vector2)) -> Line {
        Line::new(a, e)
    }
}

impl From<Line> for (Vector2, Vector2) {
    #[inline]
    fn from(l: Line) -> (Vector2, Vector2) {
        (l.a, l.e)
    }
}

impl Shape for Line {
    #[inline]
    fn intersects(&self, r: &Rect) -> bool {
        self.bounds_intersect(r) && {
            let (a, e) = (*self).into();
            r.contains(&a) || r.contains(&e) || r.line_rect(&a, &e)
        }
    }

    #[inline]
    fn contains(&self, v: &Vector2) -> bool {
        self.bounds_contains(v) && {
            let (a, e) = (*self).into();
            let (vx, vy) = (*v).into();
            FloatRange::epsilon_value(vy).line_x(&a, &e, vx)
                || FloatRange::epsilon_value(vx).line_y(&a, &e, vy)
        }
    }

    #[inline]
    fn bounds(&self) -> Rect {
        let (a, e) = (*self).into();
        Rect::from_vector(&a).extend_vector(&e)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Triangle {
    pub p1: Vector2,
    pub p2: Vector2,
    pub p3: Vector2,
}

impl Triangle {
    #[inline]
    pub const fn new(p1: Vector2, p2: Vector2, p3: Vector2) -> Triangle {
        Triangle { p1, p2, p3 }
    }
}

impl From<(Vector2, Vector2, Vector2)> for Triangle {
    #[inline]
    fn from((p1, p2, p3): (Vector2, Vector2, Vector2)) -> Triangle {
        Triangle::new(p1, p2, p3)
    }
}

impl From<Triangle> for (Vector2, Vector2, Vector2) {
    #[inline]
    fn from(t: Triangle) -> (Vector2, Vector2, Vector2) {
        (t.p1, t.p2, t.p3)
    }
}

#[inline]
fn sign(p: &Vector2, a: &Vector2, e: &Vector2) -> bool {
    let (px, py) = (*p).into();
    let (ax, ay) = (*a).into();
    let (ex, ey) = (*e).into();
    ((px - ex) * (ay - ey) - (ax - ex) * (py - ey)).sign()
}

impl Shape for Triangle {
    #[inline]
    fn intersects(&self, r: &Rect) -> bool {
        self.bounds_intersect(r) && {
            let (p1, p2, p3) = (*self).into();
            r.contains(&p1)
                || r.contains(&p2)
                || r.contains(&p3)
                || r.line_rect(&p1, &p2)
                || r.line_rect(&p2, &p3)
                || r.line_rect(&p3, &p1)
        }
    }

    #[inline]
    fn contains(&self, v: &Vector2) -> bool {
        self.bounds_contains(v) && {
            let (p1, p2, p3) = (*self).into();
            let d1 = sign(v, &p1, &p2);
            d1 == sign(v, &p2, &p3) && d1 == sign(v, &p3, &p1)
        }
    }

    #[inline]
    fn bounds(&self) -> Rect {
        let (p1, p2, p3) = (*self).into();
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
            .contains(&Vector2::new(2.5, 5.0)));
    }

    #[test]
    fn rect_should_contain_top_left() {
        assert!(Rect::new(2.0, 3.0)
            .extend(4.0, 6.0)
            .contains(&Vector2::new(2.0, 3.0)));
    }

    #[test]
    fn rect_should_contain_bottom_right() {
        assert!(Rect::new(2.0, 3.0)
            .extend(4.0, 6.0)
            .contains(&Vector2::new(4.0, 6.0)));
    }

    #[test]
    fn rect_top_left() {
        assert_eq!(
            Rect::new(2.0, 3.0).extend(4.0, 6.0).top_left(),
            Vector2::new(2.0, 6.0)
        );
    }

    #[test]
    fn rect_top_right() {
        assert_eq!(
            Rect::new(2.0, 3.0).extend(4.0, 6.0).top_right(),
            Vector2::new(4.0, 6.0)
        );
    }

    #[test]
    fn rect_bottom_left() {
        assert_eq!(
            Rect::new(2.0, 3.0).extend(4.0, 6.0).bottom_left(),
            Vector2::new(2.0, 3.0)
        );
    }

    #[test]
    fn rect_bottom_right() {
        assert_eq!(
            Rect::new(2.0, 3.0).extend(4.0, 6.0).bottom_right(),
            Vector2::new(4.0, 3.0)
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
    fn triangle_should_contain_point_on_line() {
        let triangle = Triangle::new(
            Vector2::new(1.0, 1.0),
            Vector2::new(6.0, 2.0),
            Vector2::new(4.0, 4.0),
        );

        assert!(triangle.contains(&Vector2::new(3.0, 3.0)))
    }

    #[test]
    fn triangle_should_contain_point_in_rect() {
        let triangle = Triangle::new(
            Vector2::new(1.0, 1.0),
            Vector2::new(6.0, 2.0),
            Vector2::new(4.0, 4.0),
        );

        let v = Vector2::new(1.1, 3.9);
        assert!(triangle.bounds_contains(&v));
        assert!(!triangle.contains(&v))
    }

    #[test]
    fn triangle_should_contain_out_of_rect() {
        let triangle = Triangle::new(
            Vector2::new(1.0, 1.0),
            Vector2::new(6.0, 2.0),
            Vector2::new(4.0, 4.0),
        );

        let v = Vector2::new(-3.0, -3.0);
        assert!(!triangle.bounds_contains(&v));
        assert!(!triangle.contains(&v))
    }

    #[test]
    fn triangle_should_have_correct_bound() {
        let triangle = Triangle::new(
            Vector2::new(1.0, 1.0),
            Vector2::new(6.0, 2.0),
            Vector2::new(4.0, 4.0),
        );

        assert_eq!(
            triangle.bounds(),
            Rect {
                x: FloatRange::new(1.0, 6.0),
                y: FloatRange::new(1.0, 4.0)
            }
        )
    }

    #[test]
    fn line_should_have_correct_bounds() {
        let line = Line::new(Vector2::new(1.0, 1.0), Vector2::new(6.0, 2.0));
        assert_eq!(
            line.bounds(),
            Rect {
                x: FloatRange::new(1.0, 6.0),
                y: FloatRange::new(1.0, 2.0)
            }
        )
    }

    #[test]
    fn range_should_contain_range() {
        assert!(FloatRange::new(-9.0, 3.0).contains_range(&FloatRange::new(-9.0, -3.0)));
    }

    #[test]
    fn range_should_not_contain_range() {
        assert!(!FloatRange::new(-10.0, 1.0).contains_range(&FloatRange::new(-2.0, 5.0)));
    }
}
