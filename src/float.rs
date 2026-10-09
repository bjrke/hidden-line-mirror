pub type Float = f32;

type IntPrecision = i64;

pub const PI: Float = std::f32::consts::PI;
#[cfg(test)]
pub const MAX: Float = f32::MAX;
#[cfg(test)]
pub const MIN: Float = f32::MIN;

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

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::wasm_bindgen_test;

    use super::*;

    #[wasm_bindgen_test(unsupported = test)]
    fn round_for_eq_positive() {
        assert_eq!(1.5.round_for_eq(), 98304);
    }

    #[wasm_bindgen_test(unsupported = test)]
    fn round_for_eq_negative() {
        assert_eq!((-1.5).round_for_eq(), -98304);
    }

    #[wasm_bindgen_test(unsupported = test)]
    fn round_for_eq_zero() {
        assert_eq!(0.0.round_for_eq(), 0);
    }

    #[wasm_bindgen_test(unsupported = test)]
    fn sign_positive() {
        assert!(!(1.0).sign());
    }

    #[wasm_bindgen_test(unsupported = test)]
    fn sign_negative() {
        assert!((-1.0).sign());
    }

    #[wasm_bindgen_test(unsupported = test)]
    fn sign_zero() {
        assert!(!(0.0).sign());
    }

    #[wasm_bindgen_test(unsupported = test)]
    fn sign_negative_zero() {
        assert!((-0.0).sign());
    }
}
