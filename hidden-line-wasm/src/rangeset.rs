use crate::range::*;
use std::ops::{Bound, RangeBounds};

pub struct RangeSet<T>(pub Vec<(Bound<T>, Bound<T>)>);

impl<T: PartialOrd + Copy> RangeSet<T> {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn from_range<R: RangeBounds<T>>(new_range: &R) -> Self {
        let mut result = Self::new();
        result.add(new_range);
        result
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn contains(&self, idx: T) -> bool {
        self.0.iter().any(|r| r.contains(&idx))
    }

    pub fn add<R: RangeBounds<T>>(&mut self, new_range: &R) {
        let mut new_range: (Bound<T>, Bound<T>) = new_range.to_tuple();

        self.0.retain(|r| {
            let delete = new_range.range_overlap(r);
            if delete {
                new_range = new_range.union_no_check(r);
            }
            !delete
        });

        self.0.push(new_range);
    }

    pub fn merge(&mut self, rs: &Self) {
        rs.0.iter().for_each(|r| self.add(r));
    }

    pub fn remove<R: RangeBounds<T>>(&mut self, r: &R) {
        let mut n = vec![];
        std::mem::swap(&mut n, &mut self.0);
        for t in n {
            for d in t.diff(r) {
                self.0.push(d)
            }
        }
    }

    pub fn diff(&mut self, rs: &Self) {
        rs.0.iter().for_each(|r| self.remove(r));
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    fn set0() -> RangeSet<i32> {
        RangeSet::new()
    }

    fn set1<T: PartialOrd + Copy, R: RangeBounds<T>>(r1: R) -> RangeSet<T> {
        RangeSet::from_range(&r1)
    }

    fn set2<T: PartialOrd + Copy, R1: RangeBounds<T>, R2: RangeBounds<T>>(
        r1: R1,
        r2: R2,
    ) -> RangeSet<T> {
        let mut set = set1(r1);
        set.add(&r2);
        set
    }

    #[test]
    fn is_empty_should_return_true() {
        assert!(set0().is_empty());
    }

    #[test]
    fn is_empty_should_return_false() {
        assert!(!set1(1..2).is_empty());
    }

    #[test]
    fn contains_should_return_true() {
        assert!(set2(1.0..2.0, 3.5..4.5).contains(1.5));
    }

    #[test]
    fn contains_should_return_false() {
        assert!(!set2(1.0..2.0, 3.5..4.5).contains(2.5));
    }

    #[test]
    fn contains_should_return_false_for_empty() {
        assert!(!set0().contains(1));
    }

    #[test]
    fn add_should_merge() {
        let mut set = set1(1..3);
        set.add(&(2..4));

        assert_eq!(set.0, vec![(Bound::Included(1), Bound::Excluded(4))]);
    }

    #[test]
    fn add_should_not_merge() {
        let mut set = set1(1..2);
        set.add(&(3..4));

        assert_eq!(
            set.0,
            vec![
                (Bound::Included(1), Bound::Excluded(2)),
                (Bound::Included(3), Bound::Excluded(4))
            ]
        );
    }

    #[test]
    fn merge_should_work() {
        let mut s1 = set2(1..3, 4..5);
        s1.merge(&set2(6..7, 8..9));

        assert_eq!(
            s1.0,
            vec![
                (Bound::Included(1), Bound::Excluded(3)),
                (Bound::Included(4), Bound::Excluded(5)),
                (Bound::Included(6), Bound::Excluded(7)),
                (Bound::Included(8), Bound::Excluded(9))
            ]
        );
    }

    #[test]
    fn remove_should_work() {
        let mut set = set2(1..=3, 4..=8);
        set.remove(&(2..6));

        assert_eq!(
            set.0,
            vec![
                (Bound::Included(1), Bound::Excluded(2)),
                (Bound::Included(6), Bound::Included(8))
            ]
        );
    }

    #[test]
    fn diff_should_work() {
        let mut s1 = set2(1..=3, 4..=12);
        s1.diff(&set2(2..=6, 7..8));

        assert_eq!(
            s1.0,
            vec![
                (Bound::Included(1), Bound::Excluded(2)),
                (Bound::Excluded(6), Bound::Excluded(7)),
                (Bound::Included(8), Bound::Included(12))
            ]
        );
    }
}
