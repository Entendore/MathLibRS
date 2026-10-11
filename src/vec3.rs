use crate::approx::ApproxEq;
use crate::Scalar;
use core::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign};

#[repr(C, align(16))]
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Vec3<T: Scalar = f32> { pub x: T, pub y: T, pub z: T }

impl<T: Scalar> Vec3<T> {
    pub const ZERO: Self = Self { x: T::ZERO, y: T::ZERO, z: T::ZERO };
    pub const ONE: Self = Self { x: T::ONE, y: T::ONE, z: T::ONE };
    pub const UNIT_X: Self = Self { x: T::ONE, y: T::ZERO, z: T::ZERO };
    pub const UNIT_Y: Self = Self { x: T::ZERO, y: T::ONE, z: T::ZERO };
    pub const UNIT_Z: Self = Self { x: T::ZERO, y: T::ZERO, z: T::ONE };
    
    pub const UP: Self = Self::UNIT_Y;
    pub const FORWARD: Self = Self { x: T::ZERO, y: T::ZERO, z: -T::ONE };

    #[inline] pub const fn new(x: T, y: T, z: T) -> Self { Self { x, y, z } }
    #[inline] pub const fn splat(v: T) -> Self { Self { x: v, y: v, z: v } }

    #[inline] pub fn dot(self, other: Self) -> T { self.x * other.x + self.y * other.y + self.z * other.z }
    
    #[inline]
    pub fn cross(self, other: Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    #[inline] pub fn length_squared(self) -> T { self.dot(self) }
    #[inline] pub fn length(self) -> T { self.length_squared().sqrt() }

    #[inline]
    pub fn normalize(self) -> Self {
        let len = self.length();
        if len > T::ZERO { self / len } else { Self::ZERO }
    }

    #[inline] pub fn distance_squared(self, other: Self) -> T { (self - other).length_squared() }
    #[inline] pub fn distance(self, other: Self) -> T { self.distance_squared(other).sqrt() }

    #[inline]
    pub fn reflect(self, normal: Self) -> Self {
        self - normal * (T::TWO * self.dot(normal))
    }

    #[inline]
    pub fn orthonormal_basis(self) -> (Self, Self) {
        let n = self.normalize();
        let sign = if n.z >= T::ZERO { T::ONE } else { -T::ONE };
        let a = -T::ONE / (sign + n.z);
        let b = n.x * n.y * a;
        
        let tangent = Vec3::new(T::ONE + sign * n.x * n.x * a, sign * b, -sign * n.x);
        let bitangent = Vec3::new(b, sign + n.y * n.y * a, -n.y);
        (tangent, bitangent)
    }

    #[inline]
    pub fn fast_abs(self) -> Self {
        let mask = 0x7FFFFFFF;
        Self {
            x: f32::from_bits(self.x.to_bits() & mask),
            y: f32::from_bits(self.y.to_bits() & mask),
            z: f32::from_bits(self.z.to_bits() & mask),
        }
    }

    #[inline]
    pub fn hash(self) -> u32 {
        let ix = self.x.to_bits();
        let iy = self.y.to_bits();
        let iz = self.z.to_bits();
        let mut h = ix.wrapping_mul(0x1b873593);
        h = h.wrapping_add(iy.wrapping_mul(0x85ebca6b));
        h = h.wrapping_add(iz.wrapping_mul(0xc2b2ae35));
        h ^= h >> 16; h = h.wrapping_mul(0x85ebca6b);
        h ^= h >> 13; h = h.wrapping_mul(0xc2b2ae35);
        h ^= h >> 16; h
    }
}

impl<T: Scalar> ApproxEq for Vec3<T> {
    type Tolerance = T;
    #[inline] fn approx_eq(&self, other: &Self, tolerance: T) -> bool {
        self.x.approx_eq(&other.x, tolerance) && self.y.approx_eq(&other.y, tolerance) && self.z.approx_eq(&other.z, tolerance)
    }
}

impl<T: Scalar> Add for Vec3<T> { type Output = Self; #[inline] fn add(self, rhs: Self) -> Self { Self { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z } } }
impl<T: Scalar> Sub for Vec3<T> { type Output = Self; #[inline] fn sub(self, rhs: Self) -> Self { Self { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z } } }
impl<T: Scalar> Mul<T> for Vec3<T> { type Output = Self; #[inline] fn mul(self, s: T) -> Self { Self { x: self.x * s, y: self.y * s, z: self.z * s } } }
impl<T: Scalar> Div<T> for Vec3<T> { type Output = Self; #[inline] fn div(self, s: T) -> Self { Self { x: self.x / s, y: self.y / s, z: self.z / s } } }
impl<T: Scalar> Neg for Vec3<T> { type Output = Self; #[inline] fn neg(self) -> Self { Self { x: -self.x, y: -self.y, z: -self.z } } }
impl<T: Scalar> AddAssign for Vec3<T> { #[inline] fn add_assign(&mut self, rhs: Self) { *self = *self + rhs; } }
impl<T: Scalar> MulAssign<T> for Vec3<T> { #[inline] fn mul_assign(&mut self, s: T) { *self = *self * s; } }
