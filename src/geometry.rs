use crate::{Scalar, Vec3};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Aabb<T: Scalar = f32> { pub min: Vec3<T>, pub max: Vec3<T> }
impl<T: Scalar> Aabb<T> {
    #[inline] pub const fn new(min: Vec3<T>, max: Vec3<T>) -> Self { Self { min, max } }
    #[inline] pub fn center(self) -> Vec3<T> { (self.min + self.max) * (T::ONE / T::TWO) }
    #[inline] pub fn fast_min(a: f32, b: f32) -> f32 {
        let mask = ((a - b).to_bits() & 0x8000_0000) >> 31; let inv_mask = !mask;
        f32::from_bits((a.to_bits() & inv_mask) | (b.to_bits() & mask))
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Sphere<T: Scalar = f32> { pub center: Vec3<T>, pub radius: T }
impl<T: Scalar> Sphere<T> { #[inline] pub const fn new(center: Vec3<T>, radius: T) -> Self { Self { center, radius } } }

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Plane<T: Scalar = f32> { pub normal: Vec3<T>, pub distance: T }
impl<T: Scalar> Plane<T> {
    #[inline] pub fn from_point(normal: Vec3<T>, point: Vec3<T>) -> Self { Self { normal, distance: normal.dot(point) } }
    #[inline] pub fn distance_to_point(self, point: Vec3<T>) -> T { self.normal.dot(point) - self.distance }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ray<T: Scalar = f32> { pub origin: Vec3<T>, pub dir: Vec3<T> }
impl<T: Scalar> Ray<T> {
    #[inline] pub const fn new(origin: Vec3<T>, dir: Vec3<T>) -> Self { Self { origin, dir } }
    #[inline] pub fn at(self, t: T) -> Vec3<T> { self.origin + self.dir * t }
}
