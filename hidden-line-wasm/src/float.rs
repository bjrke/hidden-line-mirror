pub type Float = f32;
pub const PI: Float = std::f32::consts::PI;
pub const MAX: Float = std::f32::MAX;
pub const EPSILON0: Float = 0.0001;
pub const EPSILON1: Float = EPSILON0 * EPSILON0;
pub const EPSILON2: Float = EPSILON1 * EPSILON1;

pub trait FloatExt {
    fn sqr(&self) -> Self;
    fn inv_sqrt(&self) -> Self;
}

impl FloatExt for Float {
    #[inline]
    fn sqr(&self) -> Self {
        self * self
    }

    #[inline]
    fn inv_sqrt(&self) -> Self {
        1.0 / self.sqrt()
    }
}

/*
function invsqrt(number: double): double;
var
  y: double;
var
  i: int64;
begin
  i := $5fe6eb50c7b537a9 - ((int64(number)) div 2);
  y := double(i);
  y := y * (1.5 - (number * 0.5 * y * y));   // 1st iteration
  //  y := y * (1.5 - (number * 0.5 * y * y));   // 2nd iteration, this can be removed
  exit(y);
end;
*/
