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
mod webapp;
mod worker;

use crate::drawcontext::*;
use crate::vec2::Vector2;
use crate::webapp::App;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    Ok(())
}

#[wasm_bindgen]
pub fn lets_go(
    svg: web_sys::SvgElement,
    redraw: js_sys::Function,
    rebuild: js_sys::Function,
) -> Result<HiddenLine, JsValue> {
    let app = App::new(svg, redraw, rebuild)?;
    Ok(HiddenLine { app })
}

#[wasm_bindgen]
pub struct HiddenLine {
    app: Rc<RefCell<App>>,
}

#[wasm_bindgen]
impl HiddenLine {
    pub fn mesh_points(&self) -> Vec<f32> {
        self.app
            .borrow()
            .app_ctx
            .scene3
            .points
            .iter()
            .flat_map(|p| [p.x, p.y, p.z])
            .collect()
    }

    pub fn mesh_triangles(&self) -> Vec<u32> {
        self.app
            .borrow()
            .app_ctx
            .scene3
            .triangles
            .iter()
            .flat_map(|t| [t.p1 as u32, t.p2 as u32, t.p3 as u32, t.lset as u32])
            .collect()
    }

    pub fn camera(&mut self) -> Vec<f32> {
        let c = self.app.borrow_mut().app_ctx.camera();
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
        let mut app = self.app.borrow_mut();
        let frame = app.frame;

        let mut groups: BTreeMap<Color, Vec<(Vector2, Vector2)>> = BTreeMap::new();

        for r in records.chunks_exact(5) {
            groups
                .entry(r[0] as u8)
                .or_default()
                .push((Vector2::new(r[1], r[2]), Vector2::new(r[3], r[4])));
        }

        for (color, segments) in groups {
            let mut color_context = app.svg_ctx.color_context(color);
            for (p1, p2) in segments {
                color_context.line(p1, p2);
            }
            app.svg_ctx.draw(frame, color_context);
        }

        app.svg_ctx.finish(frame);
        app.frame = frame.wrapping_add(1);
    }
}
