use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();

    let svg = document
        .create_element_ns(Some("http://www.w3.org/2000/svg"), "svg")
        .unwrap()
        .dyn_into::<web_sys::SvgElement>()
        .unwrap();

    let circle = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "circle")?;
    circle.set_attribute("cx", "0")?;
    circle.set_attribute("cy", "0")?;
    circle.set_attribute("r", "128")?;
    circle.set_attribute("stroke", "black")?;
    circle.set_attribute("fill", "blue")?;
    svg.append_child(&circle)?;

    svg.set_attribute("viewBox", "-128 -128 256 256")?;

    body.append_child(&svg)?;

    Ok(())
}
