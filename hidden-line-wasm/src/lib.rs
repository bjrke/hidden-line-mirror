mod appcontext;
mod calcctontext;
mod drawcontext;
mod dreidext;
mod float;
mod line;
mod linesegment;
mod mat2;
mod mat3;
mod maxxqueue;
mod minxqueue;
mod plot;
mod point;
mod polygon;
mod polysweep;
mod quadtree;
mod range;
mod rangeset;
mod shape;
mod svgcontext;
mod t2;
mod time;
mod triangle;
mod vec2;
mod vec3;

use crate::appcontext::*;
use crate::drawcontext::*;
use crate::float::*;
use crate::svgcontext::*;
use js_sys;
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
    HiddenLine {
        svgcontext: SvgContext::new(svg),
        app_ctx: plot::init(),
    }
}

#[wasm_bindgen]
pub struct HiddenLine {
    svgcontext: SvgContext,
    app_ctx: AppContext,
}

#[wasm_bindgen]
impl HiddenLine {
    pub fn on_key(&mut self, ch: char) {
        if self.app_ctx.on_key(ch) {
            self.draw();
        }
    }

    pub fn set_function(&mut self, f: &js_sys::Function) {
        self.app_ctx.sceneBuilder = plot::init_scene(|x, y| {
            f.call2(&JsValue::NULL, &JsValue::from(x), &JsValue::from(y))
                .unwrap()
                .as_f64()
                .unwrap() as Float
        });

        self.draw();
    }

    fn draw(&mut self) {
        self.svgcontext.cls();

        plot::darstellung(&mut self.svgcontext, &mut self.app_ctx);
    }
}
