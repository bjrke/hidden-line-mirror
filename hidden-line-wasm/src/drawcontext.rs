use crate::float::*;

pub type Color = u8;

pub trait DrawContext {
    fn circle(&mut self, x: Float, y: Float, r: Float, c: Color);

    fn line(&mut self, xa: Float, ya: Float, xe: Float, ye: Float, c: Color);

    fn poly(&mut self, coordinates: &[Float], c: Color);

    fn putpixel(&mut self, x: i32, y: i32, c: Color);
}
