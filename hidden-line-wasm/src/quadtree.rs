use crate::shape::*;
use crate::vec2::*;
use std::collections::VecDeque;
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
        let center = Vector2 {
            x: (bounds.x.start() + bounds.x.end()) / 2.0,
            y: (bounds.y.start() + bounds.y.end()) / 2.0,
        };
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

    fn entries<P>(&self, p: P) -> QuadTreeIterator<S, P>
    where
        P: FnMut(&dyn Shape) -> bool,
    {
        QuadTreeIterator {
            element_stack: VecDeque::new(),
            tree_stack: VecDeque::new(),
            p,
        }
    }
}

struct QuadTreeIterator<'a, T, P> {
    element_stack: VecDeque<&'a T>,
    tree_stack: VecDeque<&'a QuadTree<T>>,
    p: P,
}

impl<'a, T: Shape, P: FnMut(&dyn Shape) -> bool> Iterator for QuadTreeIterator<'a, T, P> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(element) = self.element_stack.pop_back() {
                return Some(element);
            } else if let Some(tree) = self.tree_stack.pop_back() {
                for subtree in tree.subtrees.iter() {
                    if (self.p)(&subtree.bounds) {
                        self.tree_stack.push_back(subtree);
                    }
                }
                for element in tree.content.iter() {
                    if (self.p)(element) {
                        self.element_stack.push_back(element);
                    }
                }
            } else {
                return None;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tree() {
        let mut tree = QuadTree::new(Rect::new(-1000.0, -1000.0).extend(1000.0, 1000.0));

        let expected1 = Vector2::new(100.0, 100.0);
        tree.insert(expected1);

        tree.insert(Vector2::new(-100.0, 100.0));
        tree.insert(Vector2::new(-200.0, 200.0));
        tree.insert(Vector2::new(100.0, -100.0));
        tree.insert(Vector2::new(200.0, -200.0));
        tree.insert(Vector2::new(-100.0, -100.0));
        tree.insert(Vector2::new(-200.0, -200.0));

        let expected2 = Vector2::new(200.0, 200.0);
        tree.insert(expected2);

        let rect = Rect::new(90.0, 90.0).extend(210.0, 210.0);
        let elements: Vec<&Vector2> = tree.elements_intersecting(&rect).collect();

        assert_eq!(elements.len(), 2);
        assert!(elements.contains(&&expected1));
        assert!(elements.contains(&&expected2));
        // let mut expected = HashSet::new();
        // expected.insert(Vector2::new(100.0, 100.0));
        // expected.insert(Vector2::new(200.0, 200.0));
        // assert_eq!(expected, tree);
    }
}
