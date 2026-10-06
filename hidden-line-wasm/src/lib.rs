#[cfg(target_arch = "wasm32")]
#[macro_export]
macro_rules! console_log {
    ($($arg:tt)*) => {{
        #[cfg(debug_assertions)]
        web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&format!($($arg)*)));
    }};
}

#[cfg(not(target_arch = "wasm32"))]
#[macro_export]
macro_rules! console_log {
    ($($arg:tt)*) => {{
        #[cfg(debug_assertions)]
        std::println!($($arg)*);
    }};
}

mod appcontext;
mod calcctontext;
mod drawcontext;
mod dreidext;
mod float;
mod matrix;
mod plot;
mod quadtree;
mod range;
mod rangeset;
mod shape;
mod svgcontext;
mod vec2;
mod vec3;
mod worker;

use crate::appcontext::*;
use crate::drawcontext::*;
use crate::float::*;
use crate::svgcontext::*;
use crate::vec2::Vector2;
use js_sys;
use std::collections::BTreeMap;
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
        svg_ctx: SvgContext::new(svg),
        app_ctx: AppContext::new(),
        frame: 0,
    }
}

#[wasm_bindgen]
pub struct HiddenLine {
    svg_ctx: SvgContext,
    app_ctx: AppContext,
    frame: Frame,
}

#[wasm_bindgen]
impl HiddenLine {
    pub fn on_key(&mut self, ch: char) -> bool {
        self.app_ctx.on_key(ch)
    }

    pub fn set_function(&mut self, f: &js_sys::Function) {
        self.app_ctx.scene3 = plot::init_scene(|x, y| {
            f.call2(&JsValue::NULL, &JsValue::from(x), &JsValue::from(y))
                .unwrap()
                .as_f64()
                .unwrap() as Float
        });
    }

    pub fn mesh_points(&self) -> Vec<f32> {
        self.app_ctx
            .scene3
            .points
            .iter()
            .flat_map(|p| [p.x, p.y, p.z])
            .collect()
    }

    pub fn mesh_triangles(&self) -> Vec<u32> {
        self.app_ctx
            .scene3
            .triangles
            .iter()
            .flat_map(|t| [t.p1 as u32, t.p2 as u32, t.p3 as u32, t.lset as u32])
            .collect()
    }

    pub fn camera(&mut self) -> Vec<f32> {
        let c = self.app_ctx.camera();
        vec![
            c.eye.x,
            c.eye.y,
            c.eye.z,
            c.view.x,
            c.view.y,
            c.view.z,
            c.iv.x,
            c.iv.y,
            c.iv.z,
            c.jv.x,
            c.jv.y,
            c.jv.z,
            if c.back_face { 1.0 } else { 0.0 },
        ]
    }

    pub fn draw_records(&mut self, records: &[f32]) {
        let mut groups: BTreeMap<Color, Vec<(Vector2, Vector2)>> = BTreeMap::new();

        for r in records.chunks_exact(5) {
            groups
                .entry(r[0] as u8)
                .or_default()
                .push((Vector2::new(r[1], r[2]), Vector2::new(r[3], r[4])));
        }

        for (color, segments) in groups {
            let mut color_context = self.svg_ctx.color_context(color);
            for (p1, p2) in segments {
                color_context.line(p1, p2);
            }
            self.svg_ctx.draw(self.frame, color_context);
        }

        let frame = self.frame;
        self.svg_ctx.finish(frame);
        self.frame = frame.wrapping_add(1);
    }
}
