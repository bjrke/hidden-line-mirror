use crate::float::*;
use crate::vec2::Vector2;

pub type Color = u8;

pub type Frame = u32;

pub trait DrawContext<C: ColorContext> {
    fn draw(&mut self, _frame: Frame, _color_ctx: C) {}

    fn finish(&mut self, _frame: Frame) {}

    fn color_context(&mut self, color: Color) -> C;
}

pub trait ColorContext {
    fn line(&mut self, p1: Vector2, p2: Vector2);
}

#[inline]
pub fn float_to_color(c: Float) -> Color {
    if c >= 1.0 {
        255
    } else if c <= 0.0 {
        0
    } else {
        (c * 255.0).round() as u8
    }
}

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::wasm_bindgen_test;

    use super::*;

    #[wasm_bindgen_test(unsupported = test)]
    fn one_maps_to_255() {
        assert_eq!(float_to_color(1.0), 255);
    }

    #[wasm_bindgen_test(unsupported = test)]
    fn zero_maps_to_0() {
        assert_eq!(float_to_color(0.0), 0);
    }

    #[wasm_bindgen_test(unsupported = test)]
    fn half_maps_to_128() {
        assert_eq!(float_to_color(0.5), 128);
    }

    #[wasm_bindgen_test(unsupported = test)]
    fn above_one_clamps() {
        assert_eq!(float_to_color(2.0), 255);
    }

    #[wasm_bindgen_test(unsupported = test)]
    fn below_zero_clamps() {
        assert_eq!(float_to_color(-1.0), 0);
    }

    #[wasm_bindgen_test(unsupported = test)]
    fn rounds_intermediate() {
        assert_eq!(float_to_color(0.4), 102);
    }
}
