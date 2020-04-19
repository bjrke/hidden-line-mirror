use crate::float::*;

pub type Color = Float;

pub trait DrawContext {
    fn line(&mut self, xa: Float, ya: Float, xe: Float, ye: Float, c: Color);

    fn finish(&mut self);

    fn cls(&mut self);
}
