pub type Float = f32;

type IntPrecision = i64;

pub const PI: Float = std::f32::consts::PI;
pub const MAX: Float = std::f32::MAX;
pub const MIN: Float = std::f32::MIN;

pub const PRECISION: Float = 65536.0;
pub const EPSILON0: Float = 1.0 / PRECISION;
pub const EPSILON1: Float = EPSILON0 * EPSILON0;
pub const EPSILON2: Float = EPSILON1 * EPSILON1;

pub trait FloatExt {
    fn round_for_eq(&self) -> IntPrecision;

    fn sign(&self) -> bool;
}

impl FloatExt for Float {
    #[inline]
    fn round_for_eq(&self) -> IntPrecision {
        (self * PRECISION).round() as IntPrecision
    }

    #[inline]
    fn sign(&self) -> bool {
        self.is_sign_negative()
    }
}
