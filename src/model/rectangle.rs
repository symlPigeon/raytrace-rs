use std::sync::Arc;

use crate::hit::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::materials::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Rectangle {
    pub normal: Vec3,
    pub base: Vec3,
    pub left_up: Vec3,
    pub material: Arc<dyn Material>,

    a: f64,
    b: f64,
    c: f64,
    d: f64,
}

impl Rectangle {
    pub fn new(left_up: Vec3, right_down: Vec3, normal: Vec3, material: Arc<dyn Material>) -> Self {
        // get surface equation
        let center = (left_up + right_down) / 2.0;
        let (a_, b_, c_) = (normal.x, normal.y, normal.z);
        let d_ = -a_ * center.x - b_ * center.y - c_ * center.z;
        let (a, b, c, d) = (a_, b_, c_, d_);

        Rectangle {
            normal: normal.unit(),
            base: left_up - right_down,
            left_up,
            material,
            a,
            b,
            c,
            d,
        }
    }
}

impl Hittable for Rectangle {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitRecord> {
        let t = -(self.a * ray.origin.x + self.b * ray.origin.y + self.c * ray.origin.z - self.d)
            / (self.a * ray.direction.x + self.b * ray.direction.y + self.c * ray.direction.z);
        let (x, y, z) = (
            ray.origin.x + ray.direction.x * t,
            ray.origin.y + ray.direction.y * t,
            ray.origin.z + ray.direction.z * t,
        );
        // check if the point is in the rectangle
        let root = Vec3::new(x, y, z);
        let norm_coor = (self.left_up - root) / self.base;
        if norm_coor.x < 0.0
            || norm_coor.x > 1.0
            || norm_coor.y < 0.0
            || norm_coor.y > 1.0
            || norm_coor.z < 0.0
            || norm_coor.z > 1.0
        {
            //return None;
        }

        if !interval.contains((root - ray.origin).length()) {
            return None;
        }

        let mut result = Box::new(HitRecord::new(
            root,
            self.normal,
            (root - ray.origin).length(),
            Arc::clone(&self.material),
        ));

        let outward_normal = self.normal;
        result.set_face_normal(ray, &outward_normal);

        Some(*result)
    }
}
