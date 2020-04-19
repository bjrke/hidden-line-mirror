use crate::drawcontext::*;
use crate::float::*;
use std::collections::HashMap;

const SCALE: Float = 1000.0;
type SvgInt = i32;

struct SvgLine(SvgInt, SvgInt, SvgInt, SvgInt);

pub struct SvgContext {
    svg: web_sys::SvgElement,
    document: web_sys::Document,
    color_path: HashMap<u8, Vec<SvgLine>>,
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
    fn line(&mut self, xa: Float, ya: Float, xe: Float, ye: Float, c: Color) {
        self.color_path
            .entry(color_number(c))
            .or_insert(vec![])
            .push(SvgLine(scale_x(xa), scale_y(ya), scale_x(xe), scale_y(ye)))
    }

    #[inline]
    fn finish(&mut self) {
        let Self {
            color_path, svg, ..
        } = self;
        for (&c, lines) in color_path {
            let path = self
                .document
                .create_element_ns(Some("http://www.w3.org/2000/svg"), "path")
                .unwrap();

            path.set_attribute("stroke", &html_color(c)).unwrap();
            path.set_attribute(
                "d",
                &lines
                    .iter()
                    .map(|&SvgLine(xa, ya, xe, ye)| format!("M{} {} L{} {}", xa, ya, xe, ye))
                    .collect::<Vec<String>>()
                    .join(" "),
            )
            .unwrap();

            svg.append_child(&path).unwrap();
        }
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
