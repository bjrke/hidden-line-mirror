use crate::drawcontext::*;
use crate::float::*;
use std::collections::HashMap;

const SCALE: Float = 1000.0;

pub struct SvgContext {
    svg: web_sys::SvgElement,
    document: web_sys::Document,

    color_groups: HashMap<u8, web_sys::Element>,
}

impl SvgContext {
    pub fn new(svg: web_sys::SvgElement) -> SvgContext {
        let document = svg.owner_document().unwrap();
        SvgContext {
            svg,
            document,
            color_groups: HashMap::new(),
        }
    }

    #[inline]
    fn append(&mut self, element: web_sys::Element, c: Color) {
        let c = color_number(c);
        let Self {
            color_groups,
            document,
            ..
        } = self;

        color_groups
            .entry(c)
            .or_insert_with(move || {
                let result = document
                    .create_element_ns(Some("http://www.w3.org/2000/svg"), "g")
                    .unwrap();
                result.set_attribute("stroke", &html_color(c)).unwrap();
                result
            })
            .append_child(&element)
            .unwrap();
    }

    #[inline]
    pub fn scale(&self, f: Float) -> i32 {
        (SCALE * f).round() as i32
    }

    #[inline]
    pub fn scale_x(&self, f: Float) -> i32 {
        self.scale(f)
    }

    #[inline]
    pub fn scale_y(&self, f: Float) -> i32 {
        self.scale(-f)
    }
}

impl DrawContext for SvgContext {
    #[inline]
    fn line(&mut self, xa: Float, ya: Float, xe: Float, ye: Float, c: Color) {
        let line = self
            .document
            .create_element_ns(Some("http://www.w3.org/2000/svg"), "polyline")
            .unwrap();
        line.set_attribute(
            "points",
            &format!(
                "{},{} {},{}",
                self.scale_x(xa),
                self.scale_y(ya),
                self.scale_x(xe),
                self.scale_y(ye)
            ),
        )
        .unwrap();
        self.append(line, c);
    }

    #[inline]
    fn finish(&mut self) {
        for group in self.color_groups.values() {
            self.svg.append_child(&group).unwrap();
        }
    }

    #[inline]
    fn cls(&mut self) {
        self.color_groups.clear();
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
