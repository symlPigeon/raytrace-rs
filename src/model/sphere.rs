use std::sync::Arc;

use crate::hit::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::materials::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Arc<dyn Material>
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Arc<dyn Material>) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(&ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;
        if !interval.contains(root) {
            root = (-half_b + sqrtd) / a;
            if !interval.contains(root) {
                return None;
            }
        }

        let mut result = Box::new(HitRecord::new(
            ray.at(root),
            (ray.at(root) - self.center) / self.radius,
            root,
            Arc::clone(&self.material),
        ));
        
        let outward_normal = (result.p - self.center) / self.radius;
        result.set_face_normal(ray, &outward_normal);

        Some(*result)
    }
}
