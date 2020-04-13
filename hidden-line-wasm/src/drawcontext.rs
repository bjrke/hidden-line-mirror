use crate::float::*;
use crate::vec2::*;

pub type Color = Float;

pub trait DrawContext {
    fn circle(&mut self, x: Float, y: Float, r: Float, c: Color);

    fn line(&mut self, xa: Float, ya: Float, xe: Float, ye: Float, c: Color);

    fn cls(&mut self);
}
