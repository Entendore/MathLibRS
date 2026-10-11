use crate::Scalar;
#[repr(C)] #[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Vec2<T: Scalar = f32> { pub x: T, pub y: T }
impl<T: Scalar> Vec2<T> { #[inline] pub const fn new(x: T, y: T) -> Self { Self { x, y } } }
