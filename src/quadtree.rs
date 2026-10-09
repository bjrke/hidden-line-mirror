use crate::shape::*;
use crate::vec2::*;

const MAX_ELEMENTS: usize = 3;

pub struct QuadTree<T> {
    content: Vec<QuadTreeLeave<T>>,
    subtrees: Vec<QuadTree<T>>,
    bounds: Rect,
    center: Vector2,
    size: usize,
}

struct QuadTreeLeave<T> {
    value: T,
    bounds: Rect,
}

fn create_content_array<T>() -> Vec<QuadTreeLeave<T>> {
    Vec::with_capacity(MAX_ELEMENTS)
}

impl<S> QuadTree<S> {
    pub fn new(bounds: Rect) -> QuadTree<S> {
        let (xs, xe) = bounds.x.into();
        let (ys, ye) = bounds.y.into();
        let center = Vector2::new((xs + xe) / 2.0, (ys + ye) / 2.0);
        QuadTree {
            content: create_content_array(),
            subtrees: Vec::with_capacity(4),
            bounds,
            center,
            size: 0,
        }
    }

    pub fn insert(&mut self, bounds: Rect, value: S) {
        self.insert2(QuadTreeLeave { bounds, value });
    }

    fn insert2(&mut self, te: QuadTreeLeave<S>) {
        self.size += 1;
        if self.content.len() >= MAX_ELEMENTS && self.subtrees.is_empty() {
            self.create_subtree(&self.bounds.top_left());
            self.create_subtree(&self.bounds.top_right());
            self.create_subtree(&self.bounds.bottom_left());
            self.create_subtree(&self.bounds.bottom_right());

            let mut old: Vec<QuadTreeLeave<S>> = create_content_array();
            std::mem::swap(&mut self.content, &mut old);
            for t in old {
                self.insert_internal(t);
            }
        }
        self.insert_internal(te);
    }

    fn insert_internal(&mut self, te: QuadTreeLeave<S>) {
        for st in self.subtrees.iter_mut() {
            if st.bounds.contains_rect(&te.bounds) {
                st.insert2(te);
                return;
            }
        }
        self.content.push(te);
    }

    fn create_subtree(&mut self, v: &Vector2) {
        self.subtrees.push(QuadTree::new(
            Rect::from_vector(v).extend_vector(&self.center),
        ));
    }

    pub fn elements_intersecting<F: FnMut(&S) -> bool>(&self, rect: &Rect, f: &mut F) -> bool {
        for QuadTreeLeave { bounds, value } in self.content.iter() {
            if bounds.intersects(rect) && f(value) {
                return true;
            }
        }
        for t in self.subtrees.iter() {
            if t.bounds.intersects(rect) && t.elements_intersecting(rect, f) {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::wasm_bindgen_test;

    impl<S: Shape> QuadTree<S> {
        #[inline]
        pub fn insert_shape(&mut self, value: S) {
            self.insert(value.bounds(), value);
        }
    }

    #[wasm_bindgen_test(unsupported = test)]
    fn tree() {
        let mut tree = QuadTree::new(Rect::new(-1000.0, -1000.0).extend(1000.0, 1000.0));

        let expected1 = Vector2::new(100.0, 100.0);
        tree.insert_shape(expected1);

        tree.insert_shape(Vector2::new(-100.0, 100.0));
        tree.insert_shape(Vector2::new(-200.0, 200.0));
        tree.insert_shape(Vector2::new(100.0, -100.0));
        tree.insert_shape(Vector2::new(200.0, -200.0));
        tree.insert_shape(Vector2::new(-100.0, -100.0));
        tree.insert_shape(Vector2::new(-200.0, -200.0));

        let expected2 = Vector2::new(200.0, 200.0);
        tree.insert_shape(expected2);

        let rect = Rect::new(90.0, 90.0).extend(210.0, 210.0);

        let mut elements = vec![];
        tree.elements_intersecting(&rect, &mut |r| {
            elements.push(*r);
            false
        });

        assert_eq!(elements.len(), 2);
        assert!(elements.contains(&expected1));
        assert!(elements.contains(&expected2));
    }
}
