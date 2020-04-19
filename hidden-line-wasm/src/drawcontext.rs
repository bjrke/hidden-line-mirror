use crate::float::*;
use crate::vec2::Vector2;

pub type Color = Float;

pub trait DrawContext {
    fn line(&mut self, start: &Vector2, end: &Vector2, c: Color);

    fn finish(&mut self);

    fn cls(&mut self);
}
