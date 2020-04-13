use crate::polygon::*;
use std::cmp::Ordering;

pub struct MinxQueueEntry {
    pub polygon: polygon,
}

impl Ord for MinxQueueEntry {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .polygon
            .delegate
            .p1
            .cmp(&self.polygon.delegate.p1)
            .then_with(|| self.polygon.delegate.p2.cmp(&other.polygon.delegate.p2))
            .then_with(|| self.polygon.delegate.p3.cmp(&other.polygon.delegate.p3))
    }
}

impl PartialOrd for MinxQueueEntry {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for MinxQueueEntry {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        match self.cmp(&other) {
            Ordering::Equal => true,
            _ => false,
        }
    }
}

impl Eq for MinxQueueEntry {}
