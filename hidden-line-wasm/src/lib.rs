mod appcontext;
mod drawcontext;
mod float;
mod line;
mod mat2;
mod mat3;
mod point;
mod polygon;
mod polysweep;
mod svgcontext;
mod time;
mod triangle;
mod vec2;
mod vec3;

use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    Ok(())
}

fn circle(document: web_sys::Document) -> web_sys::Element {
    let circle = document
        .create_element_ns(Some("http://www.w3.org/2000/svg"), "circle")
        .unwrap();
    circle.set_attribute("cx", "0").unwrap();
    circle.set_attribute("cy", "0").unwrap();
    circle.set_attribute("r", "100").unwrap();
    circle.set_attribute("stroke", "black").unwrap();
    circle.set_attribute("fill", "blue").unwrap();
    circle
}

#[wasm_bindgen]
pub fn lets_go(svg: web_sys::SvgElement) -> HiddenLine {
    let document = svg.owner_document().unwrap();
    let circle = circle(document);
    svg.append_child(&circle).unwrap();

    svg.set_attribute("viewBox", "-128 -128 256 256").unwrap();

    HiddenLine {
        radius: 100,
        circle,
    }
}

#[wasm_bindgen]
pub struct HiddenLine {
    radius: i32,
    circle: web_sys::Element,
}

#[wasm_bindgen]
impl HiddenLine {
    pub fn on_click(&mut self) {
        self.radius += 1;
        self.circle
            .set_attribute("r", &self.radius.to_string())
            .unwrap();
    }
}
