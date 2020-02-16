use crate::drawcontext::*;
use crate::float::*;
use crate::vec2::*;

pub struct SvgContext {
    svg: web_sys::SvgElement,
    document: web_sys::Document,
    scale: Float,
}

impl SvgContext {
    pub fn new(svg: web_sys::SvgElement) -> SvgContext {
        let document = svg.owner_document().unwrap();
        SvgContext {
            svg,
            document,
            scale: 2000.0,
        }
    }

    pub fn append(&mut self, element: web_sys::Element) {
        self.svg.append_child(&element).unwrap();
    }

    pub fn scale(&self, f: Float) -> String {
        format!("{:.0}", self.scale * f)
    }

    pub fn scale_x(&self, f: Float) -> String {
        self.scale(f)
    }

    pub fn scale_y(&self, f: Float) -> String {
        self.scale(-f)
    }
}

impl DrawContext for SvgContext {
    fn circle(&mut self, x: Float, y: Float, r: Float, c: Color) {
        let circle = self
            .document
            .create_element_ns(Some("http://www.w3.org/2000/svg"), "circle")
            .unwrap();
        circle.set_attribute("cx", &self.scale_x(x)).unwrap();
        circle.set_attribute("cy", &self.scale_y(y)).unwrap();
        circle.set_attribute("r", &self.scale(r)).unwrap();
        circle.set_attribute("fill", &html_color(c)).unwrap();
        self.append(circle);
    }

    fn line(&mut self, xa: Float, ya: Float, xe: Float, ye: Float, c: Color) {
        let line = self
            .document
            .create_element_ns(Some("http://www.w3.org/2000/svg"), "line")
            .unwrap();
        line.set_attribute("x1", &self.scale_x(xa)).unwrap();
        line.set_attribute("y1", &self.scale_y(ya)).unwrap();
        line.set_attribute("x2", &self.scale_x(xe)).unwrap();
        line.set_attribute("y2", &self.scale_y(ye)).unwrap();
        line.set_attribute("stroke", &html_color(c)).unwrap();
        self.append(line);
    }

    fn poly(&mut self, coordinates: &[Vector2], c: Color) {
        let polygon = self
            .document
            .create_element_ns(Some("http://www.w3.org/2000/svg"), "polygon")
            .unwrap();

        let points: Vec<String> = coordinates
            .iter()
            .map(|p| format!("{},{}", self.scale_x(p.x), self.scale_y(p.y)))
            .collect();

        polygon.set_attribute("points", &points.join(" ")).unwrap();
        let color = &html_color(c);
        polygon.set_attribute("fill", color).unwrap();
        polygon.set_attribute("stroke", color).unwrap();
        self.append(polygon);
    }

    fn putpixel(&mut self, x: i32, y: i32, c: Color) {
        let rect = self
            .document
            .create_element_ns(Some("http://www.w3.org/2000/svg"), "rect")
            .unwrap();
        rect.set_attribute("x", &self.scale_x(x as Float)).unwrap();
        rect.set_attribute("y", &self.scale_y(y as Float)).unwrap();
        rect.set_attribute("width", "1").unwrap();
        rect.set_attribute("height", "1").unwrap();
        rect.set_attribute("fill", &html_color(c)).unwrap();
        self.append(rect);
    }

    fn cls(&mut self) {
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

fn html_color(c: Color) -> String {
    let x = if c >= 1.0 {
        255
    } else if c <= 0.0 {
        0
    } else {
        (c * 255.0).round() as u8
    };
    format!("#{:02x}{:02x}{:02x}", x, x, x)
}
