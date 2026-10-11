use wide::{f32x4, CmpLt};

/// A bundle of 4 Vec3s processed in parallel using SIMD.
#[derive(Clone, Copy, Debug)]
pub struct Vec3x4 { pub x: f32x4, pub y: f32x4, pub z: f32x4 }

impl Vec3x4 {
    /// Transposes 4 standard Vec3s (AoS) into a Vec3x4 (SoA).
    #[inline]
    pub fn transpose_aos(aos: &[crate::Vec3<f32>; 4]) -> Self {
        Self {
            x: f32x4::from([aos[0].x, aos[1].x, aos[2].x, aos[3].x]),
            y: f32x4::from([aos[0].y, aos[1].y, aos[2].y, aos[3].y]),
            z: f32x4::from([aos[0].z, aos[1].z, aos[2].z, aos[3].z]),
        }
    }

    /// Normalizes 4 vectors simultaneously. Handles zero-lengths safely.
    #[inline]
    pub fn normalize(self) -> Self {
        let len_sq = self.x * self.x + self.y * self.y + self.z * self.z;
        let inv_len = len_sq.fast_recip();
        let mask = len_sq.cmp_lt(f32x4::splat(1e-8));
        let safe_inv_len = inv_len.blend(f32x4::ZERO, mask);

        Self { x: self.x * safe_inv_len, y: self.y * safe_inv_len, z: self.z * safe_inv_len }
    }
}
