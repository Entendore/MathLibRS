use crate::Scalar;
#[repr(C, align(16))] #[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Vec4<T: Scalar = f32> { pub x: T, pub y: T, pub z: T, pub w: T }
impl<T: Scalar> Vec4<T> { 
    #[inline] pub const fn new(x: T, y: T, z: T, w: T) -> Self { Self { x, y, z, w } }
    #[inline] pub fn xyz(self) -> crate::Vec3<T> { crate::Vec3::new(self.x, self.y, self.z) }
}
