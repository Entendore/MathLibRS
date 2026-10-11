use crate::{Scalar, Vec3, Vec4};
use crate::approx::ApproxEq;
use wide::f32x4;

#[repr(C, align(16))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Mat4<T: Scalar = f32> {
    pub x_axis: Vec4<T>, pub y_axis: Vec4<T>, pub z_axis: Vec4<T>, pub w_axis: Vec4<T>,
}

impl<T: Scalar> Default for Mat4<T> { #[inline] fn default() -> Self { Self::IDENTITY } }

impl<T: Scalar> Mat4<T> {
    pub const IDENTITY: Self = Self {
        x_axis: Vec4::new(T::ONE, T::ZERO, T::ZERO, T::ZERO),
        y_axis: Vec4::new(T::ZERO, T::ONE, T::ZERO, T::ZERO),
        z_axis: Vec4::new(T::ZERO, T::ZERO, T::ONE, T::ZERO),
        w_axis: Vec4::new(T::ZERO, T::ZERO, T::ZERO, T::ONE),
    };

    #[inline]
    pub fn from_translation(t: Vec3<T>) -> Self {
        Self { x_axis: Vec4::new(T::ONE, T::ZERO, T::ZERO, T::ZERO), y_axis: Vec4::new(T::ZERO, T::ONE, T::ZERO, T::ZERO), z_axis: Vec4::new(T::ZERO, T::ZERO, T::ONE, T::ZERO), w_axis: Vec4::new(t.x, t.y, t.z, T::ONE) }
    }

    #[inline]
    pub fn transform_point(self, v: Vec3<T>) -> Vec3<T> {
        let r = self * Vec4::new(v.x, v.y, v.z, T::ONE);
        r.xyz()
    }

    /// Ultra-fast SIMD Matrix-Vector multiplication for f32.
    #[inline]
    pub fn mul_vec4_simd(self, v: Vec4<f32>) -> Vec4<f32> {
        let v_x = f32x4::splat(v.x); let v_y = f32x4::splat(v.y);
        let v_z = f32x4::splat(v.z); let v_w = f32x4::splat(v.w);

        let x_axis = f32x4::from([self.x_axis.x, self.x_axis.y, self.x_axis.z, self.x_axis.w]);
        let y_axis = f32x4::from([self.y_axis.x, self.y_axis.y, self.y_axis.z, self.y_axis.w]);
        let z_axis = f32x4::from([self.z_axis.x, self.z_axis.y, self.z_axis.z, self.z_axis.w]);
        let w_axis = f32x4::from([self.w_axis.x, self.w_axis.y, self.w_axis.z, self.w_axis.w]);

        let res = x_axis * v_x + y_axis * v_y + z_axis * v_z + w_axis * v_w;
        let mut arr = [0.0f32; 4];
        res.to_array(&mut arr);
        Vec4::from(arr)
    }
}

impl<T: Scalar> Mul<Vec4<T>> for Mat4<T> {
    type Output = Vec4<T>;
    #[inline]
    fn mul(self, v: Vec4<T>) -> Vec4<T> {
        self.x_axis * v.x + self.y_axis * v.y + self.z_axis * v.z + self.w_axis * v.w
    }
}
