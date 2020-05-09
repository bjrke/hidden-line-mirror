use crate::float::*;
use crate::vec2::Vector2;

pub type Color = u8;

pub trait DrawContext {
    fn line(&mut self, start: &Vector2, end: &Vector2, c: Color);

    fn finish(&mut self);

    fn cls(&mut self);
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
