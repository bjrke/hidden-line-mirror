use crate::float::*;
use crate::shape::*;
use crate::vec2::*;

const MAX_ELEMENTS: usize = 3;

pub struct QuadTree<T> {
    content: Vec<T>,
    subtrees: Vec<QuadTree<T>>,
    bounds: Rect,
    center: Vector2,
    size: usize,
}

impl<T> QuadTree<T> {
    pub fn new(bounds: Rect) -> QuadTree<T> {
        let center = Vector2 {
            x: (bounds.x.start() + bounds.x.end()) / 2.0,
            y: (bounds.y.start() + bounds.y.end()) / 2.0,
        };
        QuadTree {
            content: Vec::with_capacity(MAX_ELEMENTS),
            subtrees: Vec::with_capacity(4),
            bounds,
            center,
            size: 0,
        }
    }
}

impl<T> QuadTree<T> {
    pub fn element_contains<'a>(
        &'a self,
        v: &'a Vector2,
    ) -> Box<dyn std::iter::Iterator<Item = &T> + 'a> {
        if self.bounds.contains(v) {
            Box::new(
                self.content.iter().chain(
                    self.subtrees
                        .iter()
                        .flat_map(move |t| t.element_contains(v)),
                ),
            )
        } else {
            Box::new(std::iter::empty())
        }
    }

    pub fn elements_intersecting<'a>(
        &'a self,
        r: &'a Rect,
    ) -> Box<dyn std::iter::Iterator<Item = &T> + 'a> {
        if self.bounds.intersects(r) {
            Box::new(
                self.content.iter().chain(
                    self.subtrees
                        .iter()
                        .flat_map(move |t| t.elements_intersecting(r)),
                ),
            )
        } else {
            Box::new(std::iter::empty())
        }
    }
}

impl<S: Shape> QuadTree<S> {
    pub fn insert(&mut self, s: S) {
        self.size += 1;
        if self.size >= MAX_ELEMENTS && self.subtrees.is_empty() {
            self.create_subtree(&self.bounds.top_left());
            self.create_subtree(&self.bounds.top_right());
            self.create_subtree(&self.bounds.bottom_left());
            self.create_subtree(&self.bounds.bottom_right());

            let mut old: Vec<S> = Vec::with_capacity(MAX_ELEMENTS);
            std::mem::swap(&mut self.content, &mut old);
            for t in old {
                self.insert_internal(t);
            }
        }
        self.insert_internal(s);
    }

    fn insert_internal(&mut self, s: S) {
        for st in self.subtrees.iter_mut() {
            if st.bounds.intersects(s.bounds()) {
                st.insert(s);
                return;
            }
        }
        self.content.push(s);
    }

    fn create_subtree(&mut self, v: &Vector2) {
        self.subtrees.push(QuadTree::new(
            Rect::from_vector(v).extend_vector(&self.center),
        ));
    }
}
