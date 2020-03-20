use std::cmp::Ordering;
use std::fmt::Debug;
use std::ops::Bound;
use std::ops::RangeBounds;

pub trait BoundExt<Idx> {
    fn unwrap<'a>(&'a self, unbound: &'a Idx) -> &'a Idx;
}

impl<Idx> BoundExt<Idx> for Bound<Idx> {
    fn unwrap<'a>(&'a self, unbound: &'a Idx) -> &'a Idx {
        match self {
            Bound::Unbounded => unbound,
            Bound::Included(s) | Bound::Excluded(s) => s,
        }
    }
}

pub trait RangeExt<Idx: PartialOrd> {
    fn is_empty(&self) -> bool;
}

pub trait RangeExtCopy<Idx: PartialOrd + Copy>: RangeExt<Idx> {
    fn range_overlap<R: RangeBounds<Idx>>(&self, other: &R) -> bool;

    fn intersect<R: RangeBounds<Idx>>(&self, other: &R) -> Option<(Bound<Idx>, Bound<Idx>)>;

    fn union_no_check<R: RangeBounds<Idx>>(&self, other: &R) -> (Bound<Idx>, Bound<Idx>);

    fn diff<R: RangeBounds<Idx>>(&self, other: &R) -> Vec<(Bound<Idx>, Bound<Idx>)>;

    fn not(&self) -> Vec<(Bound<Idx>, Bound<Idx>)>;

    fn start(&self) -> Bound<Idx>;

    fn end(&self) -> Bound<Idx>;

    fn to_tuple(&self) -> (Bound<Idx>, Bound<Idx>);
}

impl<Idx: PartialOrd<Idx>, T: RangeBounds<Idx>> RangeExt<Idx> for T {
    fn is_empty(&self) -> bool {
        match (self.start_bound(), self.end_bound()) {
            (Bound::Unbounded, _) | (_, Bound::Unbounded) => false,
            (Bound::Included(a), Bound::Included(e)) => a > e,
            (Bound::Excluded(a), Bound::Excluded(e))
            | (Bound::Included(a), Bound::Excluded(e))
            | (Bound::Excluded(a), Bound::Included(e)) => a >= e,
        }
    }
}

impl<Idx: PartialOrd<Idx> + Copy, T: RangeBounds<Idx>> RangeExtCopy<Idx> for T {
    #[inline]
    fn range_overlap<R: RangeBounds<Idx>>(&self, other: &R) -> bool {
        self.intersect(other).is_some()
    }

    #[inline]
    fn intersect<R: RangeBounds<Idx>>(&self, other: &R) -> Option<(Bound<Idx>, Bound<Idx>)> {
        let range = (
            max(self.start(), other.start(), true),
            min(self.end(), other.end(), false),
        );

        if range.is_empty() {
            None
        } else {
            Some(range)
        }
    }

    #[inline]
    fn union_no_check<R: RangeBounds<Idx>>(&self, other: &R) -> (Bound<Idx>, Bound<Idx>) {
        (
            min(self.start(), other.start(), true),
            max(self.end(), other.end(), false),
        )
    }

    fn diff<R: RangeBounds<Idx>>(&self, other: &R) -> Vec<(Bound<Idx>, Bound<Idx>)> {
        other.not().iter().flat_map(|o| self.intersect(o)).collect()
    }

    #[inline]
    fn not(&self) -> Vec<(Bound<Idx>, Bound<Idx>)> {
        match (self.start(), self.end()) {
            (Bound::Unbounded, Bound::Unbounded) => vec![],
            (Bound::Unbounded, Bound::Included(e)) => vec![(Bound::Excluded(e), Bound::Unbounded)],
            (Bound::Unbounded, Bound::Excluded(e)) => vec![(Bound::Included(e), Bound::Unbounded)],
            (Bound::Included(a), Bound::Unbounded) => vec![(Bound::Unbounded, Bound::Excluded(a))],
            (Bound::Excluded(a), Bound::Unbounded) => vec![(Bound::Unbounded, Bound::Included(a))],
            (Bound::Included(a), Bound::Included(e)) => {
                if a > e {
                    vec![(Bound::Unbounded, Bound::Unbounded)]
                } else {
                    vec![
                        (Bound::Unbounded, Bound::Excluded(a)),
                        (Bound::Excluded(e), Bound::Unbounded),
                    ]
                }
            }
            (Bound::Excluded(a), Bound::Excluded(e)) => {
                if a >= e {
                    vec![(Bound::Unbounded, Bound::Unbounded)]
                } else {
                    vec![
                        (Bound::Unbounded, Bound::Included(a)),
                        (Bound::Included(e), Bound::Unbounded),
                    ]
                }
            }
            (Bound::Included(a), Bound::Excluded(e)) => {
                if a >= e {
                    vec![(Bound::Unbounded, Bound::Unbounded)]
                } else {
                    vec![
                        (Bound::Unbounded, Bound::Excluded(a)),
                        (Bound::Included(e), Bound::Unbounded),
                    ]
                }
            }

            (Bound::Excluded(a), Bound::Included(e)) => {
                if a >= e {
                    vec![(Bound::Unbounded, Bound::Unbounded)]
                } else {
                    vec![
                        (Bound::Unbounded, Bound::Included(a)),
                        (Bound::Excluded(e), Bound::Unbounded),
                    ]
                }
            }
        }
    }

    #[inline]
    fn start(&self) -> Bound<Idx> {
        deRefIdx(self.start_bound())
    }

    #[inline]
    fn end(&self) -> Bound<Idx> {
        deRefIdx(self.end_bound())
    }

    #[inline]
    fn to_tuple(&self) -> (Bound<Idx>, Bound<Idx>) {
        (self.start(), self.end())
    }
}

#[inline]
fn deRefIdx<Idx: Copy>(b: Bound<&Idx>) -> Bound<Idx> {
    match b {
        Bound::Unbounded => Bound::Unbounded,
        Bound::Included(&i) => Bound::Included(i),
        Bound::Excluded(&e) => Bound::Excluded(e),
    }
}

pub fn min<Idx: PartialOrd<Idx> + Copy>(
    b1: Bound<Idx>,
    b2: Bound<Idx>,
    is_lower_bound: bool,
) -> Bound<Idx> {
    match (b1, b2) {
        (Bound::Unbounded, _) | (_, Bound::Unbounded) if is_lower_bound => Bound::Unbounded,
        (Bound::Unbounded, _) => b2,
        (_, Bound::Unbounded) => b1,
        (Bound::Excluded(v1), Bound::Excluded(v2)) | (Bound::Included(v1), Bound::Included(v2)) => {
            if v1 < v2 {
                b1
            } else {
                b2
            }
        }
        (Bound::Excluded(exc), Bound::Included(inc))
        | (Bound::Included(inc), Bound::Excluded(exc)) => match inc.partial_cmp(&exc) {
            Some(Ordering::Less) => Bound::Included(inc),
            Some(Ordering::Equal) => {
                if is_lower_bound {
                    Bound::Included(inc)
                } else {
                    Bound::Excluded(exc)
                }
            }
            Some(Ordering::Greater) => Bound::Excluded(exc),
            None => panic!(),
        },
    }
}

pub fn max<Idx: PartialOrd<Idx> + Copy>(
    b1: Bound<Idx>,
    b2: Bound<Idx>,
    is_lower_bound: bool,
) -> Bound<Idx> {
    match (b1, b2) {
        (Bound::Unbounded, _) | (_, Bound::Unbounded) if !is_lower_bound => Bound::Unbounded,
        (Bound::Unbounded, _) => b2,
        (_, Bound::Unbounded) => b1,
        (Bound::Excluded(v1), Bound::Excluded(v2)) | (Bound::Included(v1), Bound::Included(v2)) => {
            if v1 < v2 {
                b2
            } else {
                b1
            }
        }
        (Bound::Excluded(exc), Bound::Included(inc))
        | (Bound::Included(inc), Bound::Excluded(exc)) => match inc.partial_cmp(&exc) {
            Some(Ordering::Less) => Bound::Excluded(exc),
            Some(Ordering::Equal) => {
                if is_lower_bound {
                    Bound::Excluded(exc)
                } else {
                    Bound::Included(inc)
                }
            }
            Some(Ordering::Greater) => Bound::Included(inc),
            None => panic!(),
        },
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use std::ops::Range;

    #[test]
    fn intersect_with_result() {
        assert_eq!(Some((2..3).to_tuple()), (1..3).intersect(&(2..4)));
    }

    #[test]
    fn intersect_without_result() {
        assert_eq!(None, (1..2).intersect(&(3..4)));
    }

    #[test]
    fn intersect_with_point_result() {
        assert_eq!(Some((2..=2).to_tuple()), (1..=2).intersect(&(2..=3)));
    }

    #[test]
    fn intersect_unbound_left() {
        assert_eq!(Some((1..2).to_tuple()), (..2).intersect(&(1..3)));
    }

    #[test]
    fn diff_left_only() {
        assert_eq!(vec![(1..2).to_tuple()], (1..3).diff(&(2..4)));
    }

    #[test]
    fn not_right_unbound() {
        assert_eq!(
            vec![(..2).to_tuple()],
            (Bound::Included(2), Bound::Unbounded).not()
        );
    }

    #[test]
    fn not_left_right() {
        assert_eq!(vec![(..1).to_tuple(), (2..).to_tuple()], (1..2).not());
    }

    #[test]
    fn diff_left_only_included() {
        assert_eq!(vec![(1..2).to_tuple()], (1..3).diff(&(2..1000)));
    }

    #[test]
    fn diff_left_only_excluded() {
        assert_eq!(
            vec![(1..=2).to_tuple()],
            (1..3).diff(&(Bound::Excluded(2), Bound::Unbounded))
        );
    }

    #[test]
    fn diff_right_only() {
        assert_eq!(vec![(3..4).to_tuple()], (2..4).diff(&(1..3)));
    }

    #[test]
    fn diff_left_and_right() {
        assert_eq!(
            vec![(1..2).to_tuple(), (3..4).to_tuple()],
            (1..4).diff(&(2..3))
        );
    }
}
