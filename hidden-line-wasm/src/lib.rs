mod appcontext;
mod drawcontext;
mod dreidext;
mod float;
mod line;
mod mat2;
mod mat3;
mod plot;
mod point;
mod polygon;
mod polysweep;
mod svgcontext;
mod time;
mod triangle;
mod vec2;
mod vec3;

use crate::drawcontext::*;
use crate::float::*;
use crate::svgcontext::*;
use crate::vec2::*;
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

#[wasm_bindgen]
pub fn lets_go(svg: web_sys::SvgElement) -> HiddenLine {
    svg.set_attribute("viewBox", "-128 -128 256 256").unwrap();
    let mut svgcontext = SvgContext::new(svg);

    let mut result = HiddenLine {
        radius: 100.0,
        svgcontext,
    };

    result.draw();
    result
}

#[wasm_bindgen]
pub struct HiddenLine {
    svgcontext: SvgContext,
    radius: Float,
}

#[wasm_bindgen]
impl HiddenLine {
    pub fn on_click(&mut self) {
        self.radius -= 1.0;
        self.draw();
    }

    pub fn draw(&mut self) {
        self.svgcontext.cls();
        self.svgcontext.poly(
            &[
                Vector2::new(-self.radius, -self.radius),
                Vector2::new(self.radius, -self.radius),
                Vector2::new(self.radius, self.radius),
                Vector2::new(-self.radius, self.radius),
            ],
            0,
        );
        self.svgcontext.circle(0.0, 0.0, self.radius, 5);
        self.svgcontext.line(0.0, 0.0, self.radius, self.radius, 3);
        self.svgcontext
            .putpixel((self.radius / 2.0) as i32, (self.radius / 2.0) as i32, 15);
    }
}
