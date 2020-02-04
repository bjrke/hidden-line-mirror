pub type Float = f32;
pub const PI: Float = std::f32::consts::PI;

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
