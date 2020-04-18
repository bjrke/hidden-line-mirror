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
            x: FloatRange(x, x),
            y: FloatRange(y, y),
        }
    }

    #[inline]
    pub fn from_vector(v: &Vector2) -> Rect {
        let Vector2(x, y) = *v;
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
        let Vector2(x, y) = *v;
        self.extend(x, y)
    }

    #[inline]
    pub fn top_left(&self) -> Vector2 {
        let Rect {
            x: FloatRange(x, _),
            y: FloatRange(_, y),
        } = *self;
        Vector2(x, y)
    }

    #[inline]
    pub fn top_right(&self) -> Vector2 {
        let Rect {
            x: FloatRange(_, x),
            y: FloatRange(_, y),
        } = *self;
        Vector2(x, y)
    }

    #[inline]
    pub fn bottom_left(&self) -> Vector2 {
        let Rect {
            x: FloatRange(x, _),
            y: FloatRange(y, _),
        } = *self;
        Vector2(x, y)
    }

    #[inline]
    pub fn bottom_right(&self) -> Vector2 {
        let Rect {
            x: FloatRange(_, x),
            y: FloatRange(y, _),
        } = *self;
        Vector2(x, y)
    }

    #[inline]
    pub fn contains_rect(&self, r: &Rect) -> bool {
        self.x.contains_range(&r.x) && self.y.contains_range(&r.y)
    }

    /// warning this method should be used only after r.contains(a) and r.contains(e) check
    #[inline]
    fn line_rect(&self, a: &Vector2, e: &Vector2) -> bool {
        let Rect {
            x: FloatRange(xa, xe),
            y: FloatRange(ya, ye),
        } = *self;
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
        self.x.contains(&v.0) && self.y.contains(&v.1)
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
        return self == v;
    }

    #[inline]
    fn bounds(&self) -> Rect {
        Rect::from_vector(self)
    }
}

#[derive(Debug)]
pub struct Line(pub Vector2, pub Vector2);

impl Shape for Line {
    #[inline]
    fn intersects(&self, r: &Rect) -> bool {
        self.bounds_intersect(r) && {
            let Line(p1, p2) = self;
            r.contains(&p1) || r.contains(&p2) || r.line_rect(&p1, &p2)
        }
    }

    #[inline]
    fn contains(&self, v: &Vector2) -> bool {
        self.bounds_contains(v) && {
            let Line(a, e) = self;
            let Vector2(vx, vy) = *v;
            FloatRange::epsilon_value(vy).line_x(a, e, vx)
                || FloatRange::epsilon_value(vx).line_y(a, e, vy)
        }
    }

    #[inline]
    fn bounds(&self) -> Rect {
        let Line(a, e) = self;
        Rect::from_vector(a).extend_vector(e)
    }
}

#[derive(Debug)]
pub struct Triangle(pub Vector2, pub Vector2, pub Vector2);

#[inline]
fn sign(p: &Vector2, a: &Vector2, e: &Vector2) -> Float {
    let Vector2(px, py) = *p;
    let Vector2(ax, ay) = *a;
    let Vector2(ex, ey) = *e;
    ((px - ex) * (ay - ey) - (ax - ex) * (py - ey)).signum()
}

impl Shape for Triangle {
    #[inline]
    fn intersects(&self, r: &Rect) -> bool {
        self.bounds_intersect(r) && {
            let Triangle(p1, p2, p3) = self;
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
        let triangle = Triangle(Vector2(1.0, 1.0), Vector2(6.0, 2.0), Vector2(4.0, 4.0));

        assert!(triangle.contains(&Vector2(3.0, 3.0)))
    }

    #[test]
    fn triangle_should_contain_point_in_rect() {
        let triangle = Triangle(Vector2(1.0, 1.0), Vector2(6.0, 2.0), Vector2(4.0, 4.0));

        let v = Vector2(1.1, 3.9);
        assert!(triangle.bounds_contains(&v));
        assert!(!triangle.contains(&v))
    }

    #[test]
    fn triangle_should_contain_out_of_rect() {
        let triangle = Triangle(Vector2(1.0, 1.0), Vector2(6.0, 2.0), Vector2(4.0, 4.0));

        let v = Vector2(-3.0, -3.0);
        assert!(!triangle.bounds_contains(&v));
        assert!(!triangle.contains(&v))
    }

    #[test]
    fn triangle_should_have_correct_bound() {
        let triangle = Triangle(Vector2(1.0, 1.0), Vector2(6.0, 2.0), Vector2(4.0, 4.0));

        assert_eq!(
            triangle.bounds(),
            Rect {
                x: FloatRange(1.0, 6.0),
                y: FloatRange(1.0, 4.0)
            }
        )
    }

    #[test]
    fn line_should_have_correct_bounds() {
        let line = Line(Vector2(1.0, 1.0), Vector2(6.0, 2.0));
        assert_eq!(
            line.bounds(),
            Rect {
                x: FloatRange(1.0, 6.0),
                y: FloatRange(1.0, 2.0)
            }
        )
    }

    #[test]
    fn range_should_contain_range() {
        assert!(FloatRange(-9.0, 3.0).contains_range(&FloatRange(-9.0, -3.0)));
    }

    #[test]
    fn range_should_not_contain_range() {
        assert!(!FloatRange(-10.0, 1.0).contains_range(&FloatRange(-2.0, 5.0)));
    }
}
