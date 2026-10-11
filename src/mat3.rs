use crate::{Scalar, Vec3};
#[repr(C)] #[derive(Clone, Copy, Debug, PartialEq)]
pub struct Mat3<T: Scalar = f32> { pub x_axis: Vec3<T>, pub y_axis: Vec3<T>, pub z_axis: Vec3<T> }
