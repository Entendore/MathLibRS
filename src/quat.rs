use crate::{Scalar, Vec3};

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Quat<T: Scalar = f32> { pub x: T, pub y: T, pub z: T, pub w: T }

impl<T: Scalar> Quat<T> {
    pub const IDENTITY: Self = Self { x: T::ZERO, y: T::ZERO, z: T::ZERO, w: T::ONE };

    pub fn from_axis_angle(axis: Vec3<T>, angle: T) -> Self {
        let axis = axis.normalize();
        let half = angle / T::TWO;
        let s = half.sin();
        Self { x: axis.x * s, y: axis.y * s, z: axis.z * s, w: half.cos() }
    }

    pub fn rotate_vec3(self, v: Vec3<T>) -> Vec3<T> {
        let qv = Vec3::new(self.x, self.y, self.z);
        let t = (qv.cross(v)) * T::TWO;
        v + t * self.w + qv.cross(t)
    }
}
