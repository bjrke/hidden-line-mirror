use crate::float::*;
use crate::vec2::Vector2;

pub type Color = u8;

pub trait DrawContext<C: ColorContext> {
    fn draw(&mut self, color_ctx: C);

    fn finish(&mut self);

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
