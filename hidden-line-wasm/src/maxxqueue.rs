use crate::polygon::*;
use std::cmp::Ordering;
use std::rc::Rc;

pub struct MaxxQueueEntry {
    pub polygon: Rc<polygon>,
}

impl MaxxQueueEntry {
    fn raw_ptr_addr(&self) -> usize {
        let ptr: *const polygon = &*self.polygon;
        ptr as usize
    }
}

impl Ord for MaxxQueueEntry {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.polygon
            .delegate
            .p3
            .cmp(&other.polygon.delegate.p3)
            .then_with(|| self.raw_ptr_addr().cmp(&other.raw_ptr_addr()))
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
