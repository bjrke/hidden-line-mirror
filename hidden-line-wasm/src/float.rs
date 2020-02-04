pub type Float = f32;
pub const PI: Float = std::f32::consts::PI;
pub const MAX: Float = std::f32::MAX;
pub const MIN: Float = std::f32::MIN;
pub const epsilon0: Float = 0.0001;
pub const epsilon1: Float = epsilon0 * epsilon0;
pub const epsilon2: Float = epsilon1 * epsilon1;

pub trait FloatExt {
  fn sqr(&self) -> Float;
  fn inv_sqrt(&self) -> Float;
}

impl FloatExt for Float {
  fn sqr(&self) -> Float {
    self * self
  }

  fn inv_sqrt(&self) -> Float {
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
