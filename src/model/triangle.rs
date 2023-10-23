use std::sync::Arc;

use crate::hit::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::materials::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Triangle {
    pub a: Vec3,
    pub b: Vec3,
    pub c: Vec3,
    pub material: Arc<dyn Material>,
    pub normal: Vec3,
}

impl Triangle {
    pub fn new(a: Vec3, b: Vec3, c: Vec3, material: Arc<dyn Material>) -> Triangle {
        // Normal Vector obeys the right hand rule
        // a x b = c
        let normal = (b - a).cross(&(c - a));
        Triangle {
            a,
            b,
            c,
            material,
            normal,
        }
    }
}

impl Hittable for Triangle {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitRecord> {
        // Moller-Trumbore Algorithm
        // check if the ray is parallel to the triangle
        let edge1 = self.b - self.a;
        let edge2 = self.c - self.a;
        let h = ray.direction.cross(&edge2);
        let a = edge1.dot(&h);
        if a > -0.0001 && a < 0.0001 {
            return None;
        }
        let f = 1.0 / a;
        let s = ray.origin - self.a;
        let u = f * s.dot(&h);
        if u < 0.0 || u > 1.0 {
            return None;
        }
        let q = s.cross(&edge1);
        let v = f * ray.direction.dot(&q);
        if v < 0.0 || u + v > 1.0 {
            return None;
        }
        let t = f * edge2.dot(&q);
        if !interval.contains(t) {
            return None;
        }
        let result = HitRecord::new(
            ray.at(t),
            self.normal,
            t,
            Arc::clone(&self.material),
        );
        Some(result)
    }
}
