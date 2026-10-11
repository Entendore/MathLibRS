use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

pub trait Scalar:
    Copy + Clone + PartialEq + PartialOrd + Default + 'static
    + Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Div<Output = Self>
    + Neg<Output = Self> + AddAssign + SubAssign + MulAssign + DivAssign
    + core::fmt::Debug + core::fmt::Display
{
    const ZERO: Self; const ONE: Self; const TWO: Self; const FOUR: Self;
    const PI: Self; const TAU: Self; const HALF_PI: Self; const EPSILON: Self;
    const MAX: Self; const MIN: Self;

    fn sqrt(self) -> Self; fn sin(self) -> Self; fn cos(self) -> Self; fn tan(self) -> Self;
    fn asin(self) -> Self; fn acos(self) -> Self; fn atan(self) -> Self; fn atan2(self, other: Self) -> Self;
    fn abs(self) -> Self; fn signum(self) -> Self; fn floor(self) -> Self; fn ceil(self) -> Self;
    fn round(self) -> Self; fn to_degrees(self) -> Self; fn to_radians(self) -> Self;
    fn recip(self) -> Self; fn powi(self, n: i32) -> Self; fn powf(self, n: Self) -> Self;
    fn clamp_val(self, min: Self, max: Self) -> Self;

    #[inline]
    fn lerp(self, other: Self, t: Self) -> Self { self + (other - self) * t }
}

macro_rules! impl_scalar {
    ($($ty:ty),*) => {
        $(
            impl Scalar for $ty {
                const ZERO: Self = 0.0; const ONE: Self = 1.0; const TWO: Self = 2.0; const FOUR: Self = 4.0;
                const PI: Self = core::primitive::$ty::consts::PI;
                const TAU: Self = core::primitive::$ty::consts::TAU;
                const HALF_PI: Self = core::primitive::$ty::consts::FRAC_PI_2;
                const EPSILON: Self = 1e-6;
                const MAX: Self = core::primitive::$ty::MAX;
                const MIN: Self = core::primitive::$ty::MIN;

                #[inline] fn sqrt(self) -> Self { <$ty>::sqrt(self) }
                #[inline] fn sin(self) -> Self { <$ty>::sin(self) }
                #[inline] fn cos(self) -> Self { <$ty>::cos(self) }
                #[inline] fn tan(self) -> Self { <$ty>::tan(self) }
                #[inline] fn asin(self) -> Self { <$ty>::asin(self) }
                #[inline] fn acos(self) -> Self { <$ty>::acos(self) }
                #[inline] fn atan(self) -> Self { <$ty>::atan(self) }
                #[inline] fn atan2(self, other: Self) -> Self { <$ty>::atan2(self, other) }
                #[inline] fn abs(self) -> Self { <$ty>::abs(self) }
                #[inline] fn signum(self) -> Self { <$ty>::signum(self) }
                #[inline] fn floor(self) -> Self { <$ty>::floor(self) }
                #[inline] fn ceil(self) -> Self { <$ty>::ceil(self) }
                #[inline] fn round(self) -> Self { <$ty>::round(self) }
                #[inline] fn to_degrees(self) -> Self { <$ty>::to_degrees(self) }
                #[inline] fn to_radians(self) -> Self { <$ty>::to_radians(self) }
                #[inline] fn recip(self) -> Self { <$ty>::recip(self) }
                #[inline] fn powi(self, n: i32) -> Self { <$ty>::powi(self, n) }
                #[inline] fn powf(self, n: Self) -> Self { <$ty>::powf(self, n) }
                #[inline] fn clamp_val(self, min: Self, max: Self) -> Self { <$ty>::clamp(self, min, max) }
            }
        )*
    };
}

impl_scalar!(f32, f64);
