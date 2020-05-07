use crate::float::*;
use crate::vec2::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FloatRange(pub Float, pub Float);

impl FloatRange {
    #[inline]
    pub fn epsilon_value(f: Float) -> Self {
        Self(f, f).epsilon_range()
    }

    #[inline]
    fn epsilon_range(&self) -> Self {
        let Self(start, end) = self;
        Self(start - EPSILON0, end + EPSILON0)
    }

    #[inline]
    pub fn line_x(&self, a: &Vector2, e: &Vector2, x: Float) -> bool {
        let Vector2(ax, ay) = *a;
        let Vector2(ex, ey) = *e;
        self.line_rect_border(ax, ay, ex, ey, x)
    }

    #[inline]
    pub fn line_y(&self, a: &Vector2, e: &Vector2, y: Float) -> bool {
        let Vector2(ax, ay) = *a;
        let Vector2(ex, ey) = *e;
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
        let Self(ss, se) = *self;
        let Self(rs, re) = *rhs;
        // assume it's non empty
        // ss <= se && (rs > re || ss <= rs && se >= re)
        ss <= rs && se >= re
    }

    #[inline]
    pub fn contains(&self, item: &Float) -> bool {
        let Self(start, end) = *self;
        start <= *item && *item <= end
    }

    #[inline]
    pub fn range_overlap(&self, rhs: &Self) -> bool {
        let Self(ss, se) = *self;
        let Self(rs, re) = *rhs;
        // assume it's non empty
        // ss <= se && ss <= re && rs <= re && rs <= se
        ss <= re && rs <= se
    }

    #[inline]
    pub fn extend(&self, rhs: Float) -> Self {
        let Self(ss, se) = *self;
        if rhs < ss {
            Self(rhs, se)
        } else if rhs <= se {
            *self
        } else {
            Self(ss, rhs)
        }
    }

    #[inline]
    pub fn extend_range(&self, rhs: &Self) -> Self {
        let Self(ss, se) = *self;
        let Self(rs, re) = *rhs;
        Self(ss.min(rs), se.max(re))
    }

    #[inline]
    pub fn intersect(&self, rhs: &Self) -> Option<FloatRange> {
        let Self(ss, se) = *self;
        let Self(rs, re) = *rhs;
        let range = FloatRange(ss.max(rs), se.min(re));

        if range.is_empty_range() {
            None
        } else {
            Some(range)
        }
    }

    fn is_empty_range(&self) -> bool {
        let Self(start, end) = *self;
        start > end
    }

    pub fn diff(&self, rhs: &Self) -> Vec<Self> {
        let Self(ss, se) = *self;
        let Self(rs, re) = *rhs;
        if ss > se {
            vec![]
        } else if rs > re || re <= ss || se <= rs {
            vec![FloatRange(ss, se)]
        } else {
            let mut result = vec![];
            if rs > ss {
                result.push(FloatRange(ss, rs))
            }
            if re < se {
                result.push(FloatRange(re, se))
            }
            result
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn intersect_with_result() {
        assert_eq!(
            FloatRange(1.0, 3.0).intersect(&FloatRange(2.0, 4.0)),
            Some(FloatRange(2.0, 3.0))
        );
    }

    #[test]
    fn intersect_without_result() {
        assert_eq!(FloatRange(1.0, 2.0).intersect(&FloatRange(3.0, 4.0)), None);
    }

    #[test]
    fn intersect_with_point_result() {
        assert_eq!(
            FloatRange(1.0, 2.0).intersect(&FloatRange(2.0, 3.0)),
            Some(FloatRange(2.0, 2.0))
        );
    }

    #[test]
    fn intersect_unbound_left() {
        assert_eq!(
            FloatRange(MIN, 2.0).intersect(&FloatRange(1.0, 3.0)),
            Some(FloatRange(1.0, 2.0))
        );
    }

    #[test]
    fn diff_left_only() {
        assert_eq!(
            FloatRange(1.0, 3.0).diff(&FloatRange(2.0, 4.0)),
            vec![FloatRange(1.0, 2.0)]
        );
    }

    #[test]
    fn diff_left_only_included() {
        assert_eq!(
            FloatRange(1.0, 3.0).diff(&FloatRange(2.0, 1000.0)),
            vec![FloatRange(1.0, 2.0)]
        );
    }

    #[test]
    fn diff_left_only_excluded() {
        assert_eq!(
            FloatRange(1.0, 3.0).diff(&FloatRange(2.0, MAX)),
            vec![FloatRange(1.0, 2.0)]
        );
    }

    #[test]
    fn diff_right_only() {
        assert_eq!(
            FloatRange(2.0, 4.0).diff(&FloatRange(1.0, 3.0)),
            vec![FloatRange(3.0, 4.0)]
        );
    }

    #[test]
    fn diff_left_and_right() {
        assert_eq!(
            FloatRange(1.0, 4.0).diff(&FloatRange(2.0, 3.0)),
            vec![FloatRange(1.0, 2.0), FloatRange(3.0, 4.0)]
        );
    }

    #[test]
    fn range_should_not_overlap() {
        assert!(!FloatRange(2.0, 3.0).range_overlap(&FloatRange(1.0, 1.5)));
    }
}
