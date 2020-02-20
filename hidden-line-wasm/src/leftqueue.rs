use crate::polygon::*;
use std::cmp::Ordering;

pub struct LeftQueueEntry {
    pub polygon: polygon,
}

impl Ord for LeftQueueEntry {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.polygon
            .delegate
            .p1
            .b
            .cmp(&other.polygon.delegate.p1.b)
            .then_with(|| self.polygon.delegate.p2.b.cmp(&other.polygon.delegate.p2.b))
            .then_with(|| self.polygon.delegate.p3.b.cmp(&other.polygon.delegate.p3.b))
    }
}

impl PartialOrd for LeftQueueEntry {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for LeftQueueEntry {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        match self.cmp(&other) {
            Ordering::Equal => true,
            _ => false,
        }
    }
}

impl Eq for LeftQueueEntry {}
