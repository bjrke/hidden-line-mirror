use crate::drawcontext::*;
use crate::float::*;
use crate::vec2::Vector2;
use std::collections::{HashMap, VecDeque};

const SCALE: Float = 1000.0;
type SvgInt = i32;

#[derive(Clone, Copy, PartialEq, Hash, Eq)]
struct SvgPoint(SvgInt, SvgInt);

impl SvgPoint {
    #[inline]
    fn new(v: &Vector2) -> Self {
        let &Vector2(x, y) = v;
        Self(scale_x(x), scale_y(y))
    }
}
struct ColorGroup {
    front: HashMap<SvgPoint, usize>,
    back: HashMap<SvgPoint, usize>,
    queues: Vec<VecDeque<SvgPoint>>,
}

impl ColorGroup {
    fn new() -> Self {
        Self {
            front: HashMap::new(),
            back: HashMap::new(),
            queues: vec![],
        }
    }

    fn push(&mut self, p1: SvgPoint, p2: SvgPoint) {
        if let Some(q) = self.front.remove(&p1) {
            self.queues[q].push_front(p2);
            self.front.insert(p2, q);
        } else if let Some(q) = self.back.remove(&p1) {
            self.queues[q].push_back(p2);
            self.back.insert(p2, q);
        } else if let Some(q) = self.front.remove(&p2) {
            self.queues[q].push_front(p1);
            self.front.insert(p1, q);
        } else if let Some(q) = self.back.remove(&p2) {
            self.queues[q].push_back(p1);
            self.back.insert(p1, q);
        } else {
            let q = self.queues.len();
            let mut queue = VecDeque::new();
            queue.push_front(p1);
            queue.push_back(p2);
            self.queues.push(queue);
            self.front.insert(p1, q);
            self.back.insert(p2, q);
        }
    }
}

pub struct SvgContext {
    svg: web_sys::SvgElement,
    document: web_sys::Document,
    color_path: HashMap<u8, ColorGroup>,
}

#[inline]
fn scale(f: Float) -> SvgInt {
    (SCALE * f).round() as SvgInt
}

#[inline]
fn scale_x(f: Float) -> SvgInt {
    scale(f)
}

#[inline]
fn scale_y(f: Float) -> SvgInt {
    scale(-f)
}

impl SvgContext {
    pub fn new(svg: web_sys::SvgElement) -> SvgContext {
        let document = svg.owner_document().unwrap();
        SvgContext {
            svg,
            document,
            color_path: HashMap::new(),
        }
    }
}

impl DrawContext for SvgContext {
    #[inline]
    fn line(&mut self, start: &Vector2, end: &Vector2, c: Color) {
        self.color_path
            .entry(color_number(c))
            .or_insert_with(|| ColorGroup::new())
            .push(SvgPoint::new(start), SvgPoint::new(end));
    }

    #[inline]
    fn finish(&mut self) {
        let Self {
            color_path, svg, ..
        } = self;

        let group = self
            .document
            .create_element_ns(Some("http://www.w3.org/2000/svg"), "g")
            .unwrap();
        group.set_attribute("fill", &"none").unwrap();

        for (&c, ColorGroup { queues, .. }) in color_path {
            let path = self
                .document
                .create_element_ns(Some("http://www.w3.org/2000/svg"), "path")
                .unwrap();
            path.set_attribute("stroke", &html_color(c)).unwrap();

            let mut path_attribute = String::new();

            for queue in queues {
                if let Some(SvgPoint(x, y)) = queue.pop_front() {
                    path_attribute.push_str(&format!(" M{},{}", x, y));
                    while let Some(SvgPoint(x, y)) = queue.pop_front() {
                        path_attribute.push_str(&format!(" L{},{}", x, y));
                    }
                }
            }

            path.set_attribute("d", &path_attribute).unwrap();

            group.append_child(&path).unwrap();
        }
        svg.append_child(&group).unwrap();
    }

    #[inline]
    fn cls(&mut self) {
        self.color_path.clear();
        loop {
            match self.svg.last_child() {
                Some(e) => {
                    self.svg.remove_child(&e).unwrap();
                }
                _ => return,
            }
        }
    }
}

#[inline]
fn html_color(x: u8) -> String {
    format!("#{:02x}{:02x}{:02x}", x, x, x)
}

#[inline]
fn color_number(c: f32) -> u8 {
    if c >= 1.0 {
        255
    } else if c <= 0.0 {
        0
    } else {
        (c * 255.0).round() as u8
    }
}
