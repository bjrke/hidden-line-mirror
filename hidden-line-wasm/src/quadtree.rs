use crate::shape::*;
use crate::vec2::*;
use std::fmt::Debug;

const MAX_ELEMENTS: usize = 3;

pub struct QuadTree<T> {
    content: Vec<T>,
    subtrees: Vec<QuadTree<T>>,
    bounds: Rect,
    center: Vector2,
    size: usize,
}

fn create_content_array<T>() -> Vec<T> {
    Vec::with_capacity(MAX_ELEMENTS)
}

impl<S: Shape + Debug> QuadTree<S> {
    pub fn new(bounds: Rect) -> QuadTree<S> {
        let center = Vector2(
            (bounds.x.start() + bounds.x.end()) / 2.0,
            (bounds.y.start() + bounds.y.end()) / 2.0,
        );
        QuadTree {
            content: create_content_array(),
            subtrees: Vec::with_capacity(4),
            bounds,
            center,
            size: 0,
        }
    }

    pub fn element_contains<'a>(
        &'a self,
        v: &'a Vector2,
    ) -> Box<dyn std::iter::Iterator<Item = &S> + 'a> {
        if self.bounds.contains(v) {
            Box::new(
                self.content.iter().filter(move |c| c.contains(v)).chain(
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
    ) -> Box<dyn std::iter::Iterator<Item = &S> + 'a> {
        if self.bounds.intersects(r) {
            Box::new(
                self.content.iter().filter(move |c| c.intersects(r)).chain(
                    self.subtrees
                        .iter()
                        .flat_map(move |t| t.elements_intersecting(r)),
                ),
            )
        } else {
            Box::new(std::iter::empty())
        }
    }

    pub fn elements<'a>(&'a self) -> Box<dyn std::iter::Iterator<Item = &S> + 'a> {
        Box::new(
            self.content
                .iter()
                .chain(self.subtrees.iter().flat_map(move |t| t.elements())),
        )
    }

    pub fn insert(&mut self, s: S) {
        self.size += 1;
        if self.content.len() >= MAX_ELEMENTS && self.subtrees.is_empty() {
            self.create_subtree(&self.bounds.top_left());
            self.create_subtree(&self.bounds.top_right());
            self.create_subtree(&self.bounds.bottom_left());
            self.create_subtree(&self.bounds.bottom_right());

            let mut old: Vec<S> = create_content_array();
            std::mem::swap(&mut self.content, &mut old);
            for t in old {
                self.insert_internal(t);
            }
        }
        self.insert_internal(s);
    }

    fn insert_internal(&mut self, s: S) {
        let s_bounds = s.bounds();
        for st in self.subtrees.iter_mut() {
            if st.bounds.contains_rect(&s_bounds) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tree() {
        let mut tree = QuadTree::new(Rect::new(-1000.0, -1000.0).extend(1000.0, 1000.0));

        let expected1 = Vector2(100.0, 100.0);
        tree.insert(expected1);

        tree.insert(Vector2(-100.0, 100.0));
        tree.insert(Vector2(-200.0, 200.0));
        tree.insert(Vector2(100.0, -100.0));
        tree.insert(Vector2(200.0, -200.0));
        tree.insert(Vector2(-100.0, -100.0));
        tree.insert(Vector2(-200.0, -200.0));

        let expected2 = Vector2(200.0, 200.0);
        tree.insert(expected2);

        let rect = Rect::new(90.0, 90.0).extend(210.0, 210.0);
        let elements: Vec<&Vector2> = tree.elements_intersecting(&rect).collect();

        assert_eq!(elements.len(), 2);
        assert!(elements.contains(&&expected1));
        assert!(elements.contains(&&expected2));
    }
}
