use crate::range::*;
pub struct RangeSet(pub Vec<FloatRange>);

impl RangeSet {
    #[inline]
    pub fn is_empty(&self) -> bool {
        let RangeSet(ranges) = self;
        ranges.is_empty()
    }

    #[inline]
    pub fn add(&mut self, new_range: &FloatRange) {
        let RangeSet(ranges) = self;

        let mut new_range = *new_range;
        ranges.retain(|r| {
            let delete = new_range.range_overlap(r);
            if delete {
                new_range = new_range.union_no_check(r);
            }
            !delete
        });

        ranges.push(new_range);
    }

    #[inline]
    pub fn remove(&mut self, r: &FloatRange) {
        let RangeSet(ranges) = self;
        let mut n = vec![];
        std::mem::swap(&mut n, ranges);
        for t in n {
            for d in t.diff(r) {
                ranges.push(d)
            }
        }
    }

    #[inline]
    pub fn diff(&mut self, rs: &Self) {
        let RangeSet(ranges) = rs;
        ranges.iter().for_each(|r| self.remove(r));
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    fn set0() -> RangeSet {
        RangeSet(vec![])
    }

    fn set1(r1: FloatRange) -> RangeSet {
        RangeSet(vec![r1])
    }

    fn set2(r1: FloatRange, r2: FloatRange) -> RangeSet {
        RangeSet(vec![r1, r2])
    }

    #[test]
    fn is_empty_should_return_true() {
        assert!(set0().is_empty());
    }

    #[test]
    fn is_empty_should_return_false() {
        assert!(!set1(FloatRange(1.0, 2.0)).is_empty());
    }

    #[test]
    fn add_should_merge() {
        let mut set = set1(FloatRange(1.0, 3.0));
        set.add(&FloatRange(2.0, 4.0));

        assert_eq!(set.0, vec![FloatRange(1.0, 4.0)]);
    }

    #[test]
    fn add_should_not_merge() {
        let mut set = set1(FloatRange(1.0, 2.0));
        set.add(&FloatRange(3.0, 4.0));

        assert_eq!(set.0, vec![FloatRange(1.0, 2.0), FloatRange(3.0, 4.0)]);
    }

    #[test]
    fn remove_should_work() {
        let mut set = set2(FloatRange(1.0, 3.0), FloatRange(4.0, 8.0));
        set.remove(&FloatRange(2.0, 6.0));

        assert_eq!(set.0, vec![FloatRange(1.0, 2.0), FloatRange(6.0, 8.0)]);
    }

    #[test]
    fn diff_should_work() {
        let mut s1 = set2(FloatRange(1.0, 3.0), FloatRange(4.0, 12.0));
        s1.diff(&set2(FloatRange(2.0, 6.0), FloatRange(7.0, 8.0)));

        assert_eq!(
            s1.0,
            vec![
                FloatRange(1.0, 2.0),
                FloatRange(6.0, 7.0),
                FloatRange(8.0, 12.0)
            ]
        );
    }

    #[test]
    fn remove_twice() {
        let mut s = set1(FloatRange(0.0, 1.0));

        for _ in 0..100 {
            s.remove(&FloatRange(0.0, 0.5));
        }

        let RangeSet(ranges) = s;
        assert_eq!(ranges, vec![FloatRange(0.5, 1.0)])
    }
}
