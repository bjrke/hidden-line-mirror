use crate::float::*;

pub type Color = u8;

pub trait DrawContext {
    fn circle(&mut self, x: Float, y: Float, r: Float, c: Color);

    fn line(&mut self, xa: Float, ya: Float, xe: Float, ye: Float, c: Color);
}
