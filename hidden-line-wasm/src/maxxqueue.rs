use crate::polygon::*;
use std::cmp::Ordering;

pub struct MaxxQueueEntry {
    pub polygon: polygon,
}

impl Ord for MaxxQueueEntry {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.polygon
            .delegate
            .p3
            .b
            .cmp(&other.polygon.delegate.p3.b)
            .then_with(|| self.polygon.delegate.p2.b.cmp(&other.polygon.delegate.p2.b))
            .then_with(|| self.polygon.delegate.p1.b.cmp(&other.polygon.delegate.p1.b))
    }
}

impl PartialOrd for MaxxQueueEntry {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for MaxxQueueEntry {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        match self.cmp(&other) {
            Ordering::Equal => true,
            _ => false,
        }
    }
}

impl Eq for MaxxQueueEntry {}
