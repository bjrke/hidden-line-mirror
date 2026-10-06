use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::appcontext::AppContext;
use crate::drawcontext::Frame;
use crate::float::Float;
use crate::plot;
use crate::svgcontext::SvgContext;

#[derive(Clone, Copy)]
pub struct ViewBox {
    pub x: f64,
    pub y: f64,
    pub size: f64,
}

impl ViewBox {
    pub fn new() -> ViewBox {
        ViewBox {
            x: -1000.0,
            y: -1000.0,
            size: 2000.0,
        }
    }

    pub fn pan(&mut self, dx: f64, dy: f64, rect_size: f64) {
        let f = self.size / rect_size;
        self.x -= dx * f;
        self.y -= dy * f;
    }

    pub fn zoom(&mut self, delta: f64, px: f64, py: f64, rect_width: f64, rect_height: f64) {
        let rect_size = rect_width.min(rect_height);
        let additional = self.size * delta / 100.0;
        let f = additional / (2.0 * rect_size);
        self.x -= (2.0 * px + rect_size - rect_width) * f;
        self.y -= (2.0 * py + rect_size - rect_height) * f;
        self.size += additional;
    }
}

impl Default for ViewBox {
    fn default() -> Self {
        ViewBox::new()
    }
}

struct Origin {
    rect_size: f64,
    x: f64,
    y: f64,
    base: ViewBox,
}

pub struct App {
    pub svg_ctx: SvgContext,
    pub app_ctx: AppContext,
    pub frame: Frame,

    svg: web_sys::SvgElement,
    formula: web_sys::HtmlTextAreaElement,
    submit: web_sys::HtmlInputElement,
    view_box: ViewBox,
    origin: Option<Origin>,

    redraw: js_sys::Function,
    rebuild: js_sys::Function,
}

impl App {
    pub fn new(
        svg: web_sys::SvgElement,
        redraw: js_sys::Function,
        rebuild: js_sys::Function,
    ) -> Result<Rc<RefCell<App>>, JsValue> {
        let document = web_sys::window().unwrap().document().unwrap();
        let formula = element_by_id::<web_sys::HtmlTextAreaElement>(&document, "formula")?;
        let submit = element_by_id::<web_sys::HtmlInputElement>(&document, "submitFormula")?;
        let form = element_by_id::<web_sys::HtmlFormElement>(&document, "formulaForm")?;

        let app = Rc::new(RefCell::new(App {
            svg_ctx: SvgContext::new(svg.clone()),
            app_ctx: AppContext::new(),
            frame: 0,
            svg,
            formula,
            submit,
            view_box: ViewBox::new(),
            origin: None,
            redraw,
            rebuild,
        }));

        Self::load_initial_formula(&app);
        {
            let app = app.borrow();
            app.apply_view_box();
            app.resize_formula();
            app.hide_submit();
            let _ = app.svg.focus();
        }
        Self::setup_listeners(&app, &document, &form);

        Ok(app)
    }

    fn load_initial_formula(app: &Rc<RefCell<App>>) {
        let mut app = app.borrow_mut();

        if let Some(window) = web_sys::window() {
            let hash = window.location().hash().unwrap_or_default();
            if hash.len() > 1 {
                if let Ok(decoded) = js_sys::decode_uri_component(&hash[1..]) {
                    if let Some(value) = decoded.as_string() {
                        app.formula.set_value(&value);
                    }
                }
            }
        }

        let src = app.formula.value();
        app.set_formula_source(&src);
    }

    pub fn set_formula_source(&mut self, src: &str) -> bool {
        let f = match eval_formula(src) {
            Ok(f) => f,
            Err(e) => {
                alert_error(&e);
                return false;
            }
        };

        self.app_ctx.scene3 = plot::init_scene(|x, y| {
            f.call2(&JsValue::NULL, &JsValue::from(x), &JsValue::from(y))
                .ok()
                .and_then(|v| v.as_f64())
                .unwrap_or(0.0) as Float
        });
        true
    }

    fn apply_view_box(&self) {
        let vb = self.view_box;
        let _ = self.svg.set_attribute(
            "viewBox",
            &format!("{} {} {} {}", vb.x, vb.y, vb.size, vb.size),
        );
    }

    fn resize_formula(&self) {
        let style = self.formula.style();
        let _ = style.set_property("height", "auto");
        let height = self.formula.scroll_height();
        let _ = style.set_property("height", &format!("{}px", height));
    }

    fn hide_submit(&self) {
        let _ = self.submit.style().set_property("visibility", "hidden");
    }

    fn setup_listeners(
        app: &Rc<RefCell<App>>,
        document: &web_sys::Document,
        form: &web_sys::HtmlFormElement,
    ) {
        let svg = app.borrow().svg.clone();
        let formula = app.borrow().formula.clone();

        {
            let app = app.clone();
            let callback = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
                let key = event.key();
                if key.chars().count() != 1 {
                    return;
                }
                let ch = key.chars().next().unwrap();
                let changed = { app.borrow_mut().app_ctx.on_key(ch) };
                if changed {
                    let redraw = app.borrow().redraw.clone();
                    let _ = redraw.call0(&JsValue::NULL);
                }
            }) as Box<dyn FnMut(_)>);
            let _ = svg.add_event_listener_with_callback_and_bool(
                "keydown",
                callback.as_ref().unchecked_ref(),
                true,
            );
            callback.forget();
        }

        {
            let app = app.clone();
            let callback = Closure::wrap(Box::new(move |event: web_sys::PointerEvent| {
                let base = app.borrow().view_box;
                let svg = app.borrow().svg.clone();
                let rect = svg.get_bounding_client_rect();
                app.borrow_mut().origin = Some(Origin {
                    rect_size: rect.width().min(rect.height()),
                    x: event.client_x() as f64,
                    y: event.client_y() as f64,
                    base,
                });
            }) as Box<dyn FnMut(_)>);
            let _ = document
                .add_event_listener_with_callback("pointerdown", callback.as_ref().unchecked_ref());
            callback.forget();
        }

        {
            let app = app.clone();
            let callback = Closure::wrap(Box::new(move |event: web_sys::PointerEvent| {
                let origin = app
                    .borrow()
                    .origin
                    .as_ref()
                    .map(|o| (o.rect_size, o.x, o.y, o.base));
                let (rect_size, ox, oy, base) = match origin {
                    Some(v) => v,
                    None => return,
                };
                event.prevent_default();
                {
                    let mut app = app.borrow_mut();
                    app.view_box = base;
                    app.view_box.pan(
                        event.client_x() as f64 - ox,
                        event.client_y() as f64 - oy,
                        rect_size,
                    );
                }
                app.borrow().apply_view_box();
            }) as Box<dyn FnMut(_)>);
            let _ = document
                .add_event_listener_with_callback("pointermove", callback.as_ref().unchecked_ref());
            callback.forget();
        }

        for event in ["pointerup", "pointerleave"] {
            let app = app.clone();
            let callback = Closure::wrap(Box::new(move |_event: web_sys::Event| {
                app.borrow_mut().origin = None;
            }) as Box<dyn FnMut(_)>);
            let _ =
                document.add_event_listener_with_callback(event, callback.as_ref().unchecked_ref());
            callback.forget();
        }

        {
            let app = app.clone();
            let callback = Closure::wrap(Box::new(move |event: web_sys::WheelEvent| {
                event.prevent_default();
                let svg = app.borrow().svg.clone();
                let rect = svg.get_bounding_client_rect();
                {
                    let mut app = app.borrow_mut();
                    app.view_box.zoom(
                        event.delta_y(),
                        event.client_x() as f64,
                        event.client_y() as f64,
                        rect.width(),
                        rect.height(),
                    );
                }
                app.borrow().apply_view_box();
            }) as Box<dyn FnMut(_)>);
            let options = web_sys::AddEventListenerOptions::new();
            options.set_passive(false);
            let _ = document.add_event_listener_with_callback_and_add_event_listener_options(
                "wheel",
                callback.as_ref().unchecked_ref(),
                &options,
            );
            callback.forget();
        }

        {
            let app = app.clone();
            let callback = Closure::wrap(Box::new(move |_event: web_sys::Event| {
                let app = app.borrow();
                app.resize_formula();
                let _ = app.submit.style().set_property("visibility", "visible");
            }) as Box<dyn FnMut(_)>);
            let _ = formula
                .add_event_listener_with_callback("input", callback.as_ref().unchecked_ref());
            callback.forget();
        }

        {
            let app = app.clone();
            let callback = Closure::wrap(Box::new(move |event: web_sys::Event| {
                event.prevent_default();

                let (ok, src, svg, rebuild) = {
                    let mut app = app.borrow_mut();
                    let src = app.formula.value();
                    let ok = app.set_formula_source(&src);
                    if ok {
                        app.hide_submit();
                    }
                    (ok, src, app.svg.clone(), app.rebuild.clone())
                };

                if !ok {
                    return;
                }

                if let Some(window) = web_sys::window() {
                    let encoded = js_sys::encode_uri_component(&src);
                    if let Some(hash) = encoded.as_string() {
                        let _ = window.location().set_hash(&hash);
                    }
                }
                let _ = svg.focus();
                let _ = rebuild.call0(&JsValue::NULL);
            }) as Box<dyn FnMut(_)>);
            let _ =
                form.add_event_listener_with_callback("submit", callback.as_ref().unchecked_ref());
            callback.forget();
        }
    }
}

fn element_by_id<T: JsCast>(document: &web_sys::Document, id: &str) -> Result<T, JsValue> {
    let element = document
        .get_element_by_id(id)
        .ok_or_else(|| JsValue::from_str(&format!("missing element #{}", id)))?;
    element
        .dyn_into::<T>()
        .map_err(|_| JsValue::from_str(&format!("element #{} has unexpected type", id)))
}

fn eval_formula(src: &str) -> Result<js_sys::Function, JsValue> {
    let wrapped = format!("((x,y)=>{{{}}})", src);
    js_sys::eval(&wrapped)?.dyn_into::<js_sys::Function>()
}

fn alert_error(error: &JsValue) {
    let message = if let Some(message) = error.as_string() {
        message
    } else if let Some(error) = error.dyn_ref::<js_sys::Error>() {
        error.message().into()
    } else {
        format!("{:?}", error)
    };
    if let Some(window) = web_sys::window() {
        let _ = window.alert_with_message(&message);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pan_moves_in_opposite_direction() {
        let mut vb = ViewBox::new();
        vb.pan(100.0, 50.0, 2000.0);
        assert_eq!(vb.x, -1100.0);
        assert_eq!(vb.y, -1050.0);
        assert_eq!(vb.size, 2000.0);
    }

    #[test]
    fn pan_scales_with_zoom() {
        let mut vb = ViewBox::new();
        vb.size = 4000.0;
        vb.pan(100.0, 0.0, 2000.0);
        assert_eq!(vb.x, -1200.0);
    }

    #[test]
    fn zoom_out_increases_size() {
        let mut vb = ViewBox::new();
        vb.zoom(100.0, 1000.0, 1000.0, 2000.0, 2000.0);
        assert_eq!(vb.size, 4000.0);
    }

    #[test]
    fn zoom_shifts_origin_towards_cursor() {
        let mut vb = ViewBox::new();
        vb.zoom(50.0, 1000.0, 1000.0, 2000.0, 2000.0);
        assert_eq!(vb.x, -1500.0);
        assert_eq!(vb.y, -1500.0);
        assert_eq!(vb.size, 3000.0);
    }
}
