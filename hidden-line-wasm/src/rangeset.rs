use crate::range::*;

pub struct RangeSet {
    pub ranges: Vec<FloatRange>,
}

impl RangeSet {
    #[inline]
    pub fn new(ranges: Vec<FloatRange>) -> RangeSet {
        RangeSet { ranges }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.ranges.is_empty()
    }

    #[inline]
    pub fn remove(&mut self, r: &FloatRange) {
        let mut n = vec![];
        std::mem::swap(&mut n, &mut self.ranges);
        for t in n {
            for d in t.diff(r) {
                self.ranges.push(d)
            }
        }
    }
}

impl From<Vec<FloatRange>> for RangeSet {
    #[inline]
    fn from(ranges: Vec<FloatRange>) -> RangeSet {
        RangeSet::new(ranges)
    }
}

impl From<RangeSet> for Vec<FloatRange> {
    #[inline]
    fn from(r: RangeSet) -> Vec<FloatRange> {
        r.ranges
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    fn set0() -> RangeSet {
        RangeSet::new(vec![])
    }

    fn set1(r1: FloatRange) -> RangeSet {
        RangeSet::new(vec![r1])
    }

    fn set2(r1: FloatRange, r2: FloatRange) -> RangeSet {
        RangeSet::new(vec![r1, r2])
    }

    #[test]
    fn is_empty_should_return_true() {
        assert!(set0().is_empty());
    }

    #[test]
    fn is_empty_should_return_false() {
        assert!(!set1(FloatRange::new(1.0, 2.0)).is_empty());
    }

    #[test]
    fn remove_should_work() {
        let mut set = set2(FloatRange::new(1.0, 3.0), FloatRange::new(4.0, 8.0));
        set.remove(&FloatRange::new(2.0, 6.0));

        assert_eq!(
            set.ranges,
            vec![FloatRange::new(1.0, 2.0), FloatRange::new(6.0, 8.0)]
        );
    }

    #[test]
    fn remove_twice() {
        let mut s = set1(FloatRange::new(0.0, 1.0));

        for _ in 0..100 {
            s.remove(&FloatRange::new(0.0, 0.5));
        }

        let RangeSet { ranges } = s;
        assert_eq!(ranges, vec![FloatRange::new(0.5, 1.0)])
    }
}
