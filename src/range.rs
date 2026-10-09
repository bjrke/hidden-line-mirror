use crate::float::*;
use crate::vec2::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FloatRange {
    pub start: Float,
    pub end: Float,
}

impl FloatRange {
    #[inline]
    pub const fn new(start: Float, end: Float) -> FloatRange {
        FloatRange { start, end }
    }

    #[inline]
    pub fn epsilon_value(f: Float) -> Self {
        Self::new(f, f).epsilon_range()
    }

    #[inline]
    fn epsilon_range(&self) -> Self {
        let (start, end) = (*self).into();
        Self::new(start - EPSILON0, end + EPSILON0)
    }

    #[inline]
    pub fn line_x(&self, a: &Vector2, e: &Vector2, x: Float) -> bool {
        let (ax, ay) = (*a).into();
        let (ex, ey) = (*e).into();
        self.line_rect_border(ax, ay, ex, ey, x)
    }

    #[inline]
    pub fn line_y(&self, a: &Vector2, e: &Vector2, y: Float) -> bool {
        let (ax, ay) = (*a).into();
        let (ex, ey) = (*e).into();
        self.line_rect_border(ay, ax, ey, ex, y)
    }

    #[inline]
    fn line_rect_border(&self, x1: Float, y1: Float, x2: Float, y2: Float, x: Float) -> bool {
        let divisor = x2 - x1;
        let s = x2 - x;
        let t = divisor - s;
        if t.abs() < EPSILON0 {
            self.contains(&y2)
        } else if s == 0.0 {
            self.contains(&y1)
        } else {
            s.sign() == divisor.sign() && s.abs() < divisor.abs() && {
                let y = (s * y1 + t * y2) / divisor;
                self.contains(&y)
            }
        }
    }

    #[inline]
    pub fn contains_range(&self, rhs: &Self) -> bool {
        let (ss, se) = (*self).into();
        let (rs, re) = (*rhs).into();
        // assume it's non empty
        // ss <= se && (rs > re || ss <= rs && se >= re)
        ss <= rs && se >= re
    }

    #[inline]
    pub fn contains(&self, item: &Float) -> bool {
        let (start, end) = (*self).into();
        start <= *item && *item <= end
    }

    #[inline]
    pub fn range_overlap(&self, rhs: &Self) -> bool {
        let (ss, se) = (*self).into();
        let (rs, re) = (*rhs).into();
        // assume it's non empty
        // ss <= se && ss <= re && rs <= re && rs <= se
        ss <= re && rs <= se
    }

    #[inline]
    pub fn extend(&self, rhs: Float) -> Self {
        let (ss, se) = (*self).into();
        if rhs < ss {
            Self::new(rhs, se)
        } else if rhs <= se {
            *self
        } else {
            Self::new(ss, rhs)
        }
    }

    #[inline]
    pub fn extend_range(&self, rhs: &Self) -> Self {
        let (ss, se) = (*self).into();
        let (rs, re) = (*rhs).into();
        Self::new(ss.min(rs), se.max(re))
    }

    #[inline]
    pub fn intersect(&self, rhs: &Self) -> Option<FloatRange> {
        let (ss, se) = (*self).into();
        let (rs, re) = (*rhs).into();
        let range = FloatRange::new(ss.max(rs), se.min(re));

        if range.is_empty_range() {
            None
        } else {
            Some(range)
        }
    }

    fn is_empty_range(&self) -> bool {
        let (start, end) = (*self).into();
        start > end
    }
}

impl From<(Float, Float)> for FloatRange {
    #[inline]
    fn from((start, end): (Float, Float)) -> FloatRange {
        FloatRange::new(start, end)
    }
}

impl From<FloatRange> for (Float, Float) {
    #[inline]
    fn from(r: FloatRange) -> (Float, Float) {
        (r.start, r.end)
    }
}

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::wasm_bindgen_test;

    use super::*;

    #[wasm_bindgen_test(unsupported = test)]
    fn intersect_with_result() {
        assert_eq!(
            FloatRange::new(1.0, 3.0).intersect(&FloatRange::new(2.0, 4.0)),
            Some(FloatRange::new(2.0, 3.0))
        );
    }

    #[wasm_bindgen_test(unsupported = test)]
    fn intersect_without_result() {
        assert_eq!(
            FloatRange::new(1.0, 2.0).intersect(&FloatRange::new(3.0, 4.0)),
            None
        );
    }

    #[wasm_bindgen_test(unsupported = test)]
    fn intersect_with_point_result() {
        assert_eq!(
            FloatRange::new(1.0, 2.0).intersect(&FloatRange::new(2.0, 3.0)),
            Some(FloatRange::new(2.0, 2.0))
        );
    }

    #[wasm_bindgen_test(unsupported = test)]
    fn intersect_unbound_left() {
        assert_eq!(
            FloatRange::new(MIN, 2.0).intersect(&FloatRange::new(1.0, 3.0)),
            Some(FloatRange::new(1.0, 2.0))
        );
    }

    #[wasm_bindgen_test(unsupported = test)]
    fn range_should_not_overlap() {
        assert!(!FloatRange::new(2.0, 3.0).range_overlap(&FloatRange::new(1.0, 1.5)));
    }
}
