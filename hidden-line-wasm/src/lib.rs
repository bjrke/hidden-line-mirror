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

    let circle = circle(document);
    svg.append_child(&circle)?;

    svg.set_attribute("viewBox", "-128 -128 256 256")?;

    body.append_child(&svg)?;

    setup_clicker(&body, circle);

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

fn setup_clicker(body: &web_sys::HtmlElement, circle: web_sys::Element) {
    let mut clicks = 0;
    let a = Closure::wrap(Box::new(move || {
        clicks += 1;
        circle.set_attribute("r", &clicks.to_string());
    }) as Box<dyn FnMut()>);
    body.set_onclick(Some(a.as_ref().unchecked_ref()));

    a.forget();
}
