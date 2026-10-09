use crate::range::*;

pub struct RangeSet {
    pub ranges: Vec<FloatRange>,
    scratch: Vec<FloatRange>,
}

impl RangeSet {
    #[inline]
    pub fn new(ranges: Vec<FloatRange>) -> RangeSet {
        RangeSet {
            ranges,
            scratch: Vec::new(),
        }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.ranges.is_empty()
    }

    #[inline]
    pub fn reset(&mut self) {
        self.ranges.clear();
        self.ranges.push(FloatRange::new(0.0, 1.0));
    }

    #[inline]
    pub fn remove(&mut self, r: &FloatRange) {
        let RangeSet { ranges, scratch } = self;
        scratch.clear();
        for t in ranges.drain(..) {
            let (ss, se) = t.into();
            if ss > se {
                // empty range, drop it
                continue;
            }
            let (rs, re) = (*r).into();
            if rs > re || re <= ss || se <= rs {
                scratch.push(t);
            } else {
                if rs > ss {
                    scratch.push(FloatRange::new(ss, rs));
                }
                if re < se {
                    scratch.push(FloatRange::new(re, se));
                }
            }
        }
        std::mem::swap(ranges, scratch);
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
    use wasm_bindgen_test::wasm_bindgen_test;

    use super::*;
    use crate::float::*;

    fn set0() -> RangeSet {
        RangeSet::new(vec![])
    }

    fn set1(r1: FloatRange) -> RangeSet {
        RangeSet::new(vec![r1])
    }

    fn set2(r1: FloatRange, r2: FloatRange) -> RangeSet {
        RangeSet::new(vec![r1, r2])
    }

    #[wasm_bindgen_test(unsupported = test)]
    fn is_empty_should_return_true() {
        assert!(set0().is_empty());
    }

    #[wasm_bindgen_test(unsupported = test)]
    fn is_empty_should_return_false() {
        assert!(!set1(FloatRange::new(1.0, 2.0)).is_empty());
    }

    #[wasm_bindgen_test(unsupported = test)]
    fn remove_should_work() {
        let mut set = set2(FloatRange::new(1.0, 3.0), FloatRange::new(4.0, 8.0));
        set.remove(&FloatRange::new(2.0, 6.0));

        assert_eq!(
            set.ranges,
            vec![FloatRange::new(1.0, 2.0), FloatRange::new(6.0, 8.0)]
        );
    }

    #[wasm_bindgen_test(unsupported = test)]
    fn remove_twice() {
        let mut s = set1(FloatRange::new(0.0, 1.0));

        for _ in 0..100 {
            s.remove(&FloatRange::new(0.0, 0.5));
        }

        let RangeSet { ranges, .. } = s;
        assert_eq!(ranges, vec![FloatRange::new(0.5, 1.0)])
    }

    #[wasm_bindgen_test(unsupported = test)]
    fn remove_without_overlap_keeps_range() {
        let mut s = set1(FloatRange::new(1.0, 2.0));
        s.remove(&FloatRange::new(3.0, 4.0));

        assert_eq!(s.ranges, vec![FloatRange::new(1.0, 2.0)]);
    }

    #[wasm_bindgen_test(unsupported = test)]
    fn remove_middle_splits_range() {
        let mut s = set1(FloatRange::new(1.0, 4.0));
        s.remove(&FloatRange::new(2.0, 3.0));

        assert_eq!(
            s.ranges,
            vec![FloatRange::new(1.0, 2.0), FloatRange::new(3.0, 4.0)]
        );
    }

    #[wasm_bindgen_test(unsupported = test)]
    fn remove_left_edge() {
        let mut s = set1(FloatRange::new(1.0, 3.0));
        s.remove(&FloatRange::new(2.0, 1000.0));

        assert_eq!(s.ranges, vec![FloatRange::new(1.0, 2.0)]);
    }

    #[wasm_bindgen_test(unsupported = test)]
    fn remove_right_edge() {
        let mut s = set1(FloatRange::new(2.0, 4.0));
        s.remove(&FloatRange::new(1.0, 3.0));

        assert_eq!(s.ranges, vec![FloatRange::new(3.0, 4.0)]);
    }

    #[wasm_bindgen_test(unsupported = test)]
    fn remove_spanning_gap_removes_multiple_ranges() {
        let mut s = set2(FloatRange::new(1.0, 3.0), FloatRange::new(4.0, 8.0));
        s.remove(&FloatRange::new(2.0, 6.0));

        assert_eq!(
            s.ranges,
            vec![FloatRange::new(1.0, 2.0), FloatRange::new(6.0, 8.0)]
        );
    }

    #[wasm_bindgen_test(unsupported = test)]
    fn remove_unbounded_removes_everything() {
        let mut s = set1(FloatRange::new(1.0, 2.0));
        s.remove(&FloatRange::new(MIN, MAX));

        assert_eq!(s.ranges, vec![]);
    }

    #[wasm_bindgen_test(unsupported = test)]
    fn reset_restores_full_range() {
        let mut s = set1(FloatRange::new(0.0, 1.0));
        s.remove(&FloatRange::new(0.0, 0.5));
        s.reset();

        assert_eq!(s.ranges, vec![FloatRange::new(0.0, 1.0)]);
    }
}
