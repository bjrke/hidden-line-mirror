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
    fn new(v: Vector2) -> Self {
        let Vector2(x, y) = v;
        Self(scale_x(x), scale_y(y))
    }
}

pub struct ColorGroup {
    front: HashMap<SvgPoint, usize>,
    back: HashMap<SvgPoint, usize>,
    queues: Vec<VecDeque<SvgPoint>>,
    color: Color,
}

impl ColorGroup {
    fn new(color: Color) -> Self {
        Self {
            front: HashMap::new(),
            back: HashMap::new(),
            queues: vec![],
            color,
        }
    }

    fn path_attribute(&mut self) -> String {
        let mut path_attribute = String::new();

        for queue in self.queues.iter_mut() {
            if let Some(SvgPoint(x, y)) = queue.pop_front() {
                path_attribute.push_str(&format!(" M{},{}", x, y));
                while let Some(SvgPoint(x, y)) = queue.pop_front() {
                    path_attribute.push_str(&format!(" L{},{}", x, y));
                }
            }
        }
        path_attribute
    }
}

impl ColorContext for ColorGroup {
    fn line(&mut self, p1: Vector2, p2: Vector2) {
        let p1 = SvgPoint::new(p1);
        let p2 = SvgPoint::new(p2);

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
    group: web_sys::Element,
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
        let group = create_group(&document);
        SvgContext {
            svg,
            document,
            group,
        }
    }

    fn cls(&mut self) {
        let Self { svg, .. } = self;
        loop {
            match svg.last_child() {
                Some(e) => {
                    svg.remove_child(&e).unwrap();
                }
                _ => return,
            }
        }
    }
}

impl DrawContext<ColorGroup> for SvgContext {
    fn draw(&mut self, mut lctx: ColorGroup) {
        let path = self
            .document
            .create_element_ns(Some("http://www.w3.org/2000/svg"), "path")
            .unwrap();
        path.set_attribute("stroke", &html_color(lctx.color))
            .unwrap();

        path.set_attribute("d", &lctx.path_attribute()).unwrap();

        self.group.append_child(&path).unwrap();
    }

    #[inline]
    fn finish(&mut self) {
        self.cls();
        self.svg.append_child(&self.group).unwrap();
        self.group = create_group(&self.document);
    }

    fn color_context(&mut self, color: u8) -> ColorGroup {
        ColorGroup::new(color)
    }
}

#[inline]
fn html_color(x: u8) -> String {
    format!("#{:02x}{:02x}{:02x}", x, x, x)
}

fn create_group(document: &web_sys::Document) -> web_sys::Element {
    let group = document
        .create_element_ns(Some("http://www.w3.org/2000/svg"), "g")
        .unwrap();
    group.set_attribute("fill", &"none").unwrap();
    group
}
