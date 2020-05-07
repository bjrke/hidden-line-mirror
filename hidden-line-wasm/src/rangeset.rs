use crate::range::*;
pub struct RangeSet(pub Vec<FloatRange>);

impl RangeSet {
    #[inline]
    pub fn is_empty(&self) -> bool {
        let RangeSet(ranges) = self;
        ranges.is_empty()
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
    fn remove_should_work() {
        let mut set = set2(FloatRange(1.0, 3.0), FloatRange(4.0, 8.0));
        set.remove(&FloatRange(2.0, 6.0));

        assert_eq!(set.0, vec![FloatRange(1.0, 2.0), FloatRange(6.0, 8.0)]);
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
