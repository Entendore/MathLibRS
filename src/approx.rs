use crate::Scalar;

pub trait ApproxEq: Sized {
    type Tolerance: Scalar;
    fn approx_eq(&self, other: &Self, tolerance: Self::Tolerance) -> bool;
    
    #[inline]
    fn approx_eq_eps(&self, other: &Self) -> bool {
        self.approx_eq(other, Self::Tolerance::EPSILON)
    }
}

impl ApproxEq for f32 {
    type Tolerance = f32;
    #[inline] fn approx_eq(&self, other: &Self, tolerance: f32) -> bool { (*self - *other).abs() <= tolerance }
}
impl ApproxEq for f64 {
    type Tolerance = f64;
    #[inline] fn approx_eq(&self, other: &Self, tolerance: f64) -> bool { (*self - *other).abs() <= tolerance }
}
