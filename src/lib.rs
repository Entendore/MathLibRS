#![warn(missing_docs)]
#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

//! # math_rs
//!
//! A complete, high-performance, and idiomatic math library for Rust, designed for 
//! real-time graphics, physics engines, and game development. 

pub mod approx;
pub mod scalar;
pub mod vec2;
pub mod vec3;
pub mod vec4;
pub mod mat3;
pub mod mat4;
pub mod quat;
pub mod geometry;
pub mod intersection;
pub mod simd;

pub use approx::ApproxEq;
pub use scalar::Scalar;
pub use vec2::Vec2;
pub use vec3::Vec3;
pub use vec4::Vec4;
pub use mat3::Mat3;
pub use mat4::Mat4;
pub use quat::Quat;
pub use geometry::{Aabb, Sphere, Plane, Ray};
pub use intersection::Hit;
pub use simd::Vec3x4;

/// Single-precision 2D vector.
pub type Vec2f = Vec2<f32>;
/// Single-precision 3D vector.
pub type Vec3f = Vec3<f32>;
/// Single-precision 4D vector.
pub type Vec4f = Vec4<f32>;
/// Single-precision 4×4 matrix.
pub type Mat4f = Mat4<f32>;
/// Single-precision quaternion.
pub type Quatf = Quat<f32>;
