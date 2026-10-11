use crate::{Scalar, Vec3};
use crate::geometry::{Aabb, Ray, Sphere};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Hit<T: Scalar> {
    pub distance: T, pub point: Vec3<T>, pub normal: Vec3<T>, pub uv: (T, T),
}

impl<T: Scalar> Ray<T> {
    pub fn intersect_aabb(self, aabb: Aabb<T>) -> Option<T> {
        let inv_d = Vec3::new(T::ONE / self.dir.x, T::ONE / self.dir.y, T::ONE / self.dir.z);
        let t1 = (aabb.min.x - self.origin.x) * inv_d.x;
        let t2 = (aabb.max.x - self.origin.x) * inv_d.x;
        let t3 = (aabb.min.y - self.origin.y) * inv_d.y;
        let t4 = (aabb.max.y - self.origin.y) * inv_d.y;
        let t5 = (aabb.min.z - self.origin.z) * inv_d.z;
        let t6 = (aabb.max.z - self.origin.z) * inv_d.z;

        let tmin = t1.min(t2).max(t3.min(t4)).max(t5.min(t6));
        let tmax = t1.max(t2).min(t3.max(t4)).min(t5.max(t6));

        if tmax < T::ZERO || tmin > tmax { return None; }
        Some(if tmin < T::ZERO { tmax } else { tmin })
    }

    pub fn intersect_sphere(self, sphere: Sphere<T>) -> Option<Hit<T>> {
        let oc = self.origin - sphere.center;
        let a = self.dir.dot(self.dir);
        let half_b = oc.dot(self.dir);
        let c = oc.dot(oc) - sphere.radius * sphere.radius;
        let discriminant = half_b * half_b - a * c;
        
        if discriminant < T::ZERO { return None; }
        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root < T::ZERO {
            root = (-half_b + sqrtd) / a;
            if root < T::ZERO { return None; }
        }

        let point = self.at(root);
        Some(Hit { distance: root, point, normal: (point - sphere.center) / sphere.radius, uv: (T::ZERO, T::ZERO) })
    }

    pub fn intersect_triangle(self, v0: Vec3<T>, v1: Vec3<T>, v2: Vec3<T>) -> Option<Hit<T>> {
        let edge1 = v1 - v0; let edge2 = v2 - v0;
        let h = self.dir.cross(edge2);
        let a = edge1.dot(h);
        if a > -T::EPSILON && a < T::EPSILON { return None; }

        let f = T::ONE / a;
        let s = self.origin - v0;
        let u = f * s.dot(h);
        if u < T::ZERO || u > T::ONE { return None; }

        let q = s.cross(edge1);
        let v = f * self.dir.dot(q);
        if v < T::ZERO || u + v > T::ONE { return None; }

        let t = f * edge2.dot(q);
        if t > T::EPSILON {
            Some(Hit { distance: t, point: self.at(t), normal: edge1.cross(edge2).normalize(), uv: (u, v) })
        } else { None }
    }
}
