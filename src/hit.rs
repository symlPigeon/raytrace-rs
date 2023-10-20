use std::sync::Arc;

use crate::{vec3::Vec3, ray::Ray, interval::Interval, material::materials::Material};

pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub material: Arc<dyn Material>
}

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>
}

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitRecord>;
}


impl HitRecord {
    pub fn new(p: Vec3, normal: Vec3, t: f64, material: Arc<dyn Material>) -> Self {
        HitRecord { p, normal, t, front_face: false, material }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = ray.direction.dot(outward_normal) < 0.0;
        self.normal = if self.front_face { *outward_normal } else { *outward_normal * -1.0 };
    }
}


impl HittableList {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        HittableList { objects: Vec::new() }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitRecord> {
        let mut hit_record: Option<HitRecord> = None;
        let mut closest_so_far = interval.max;

        for object in self.objects.iter() {
            if let Some(temp_record) = object.hit(ray, &Interval::new(interval.min, closest_so_far)) {
                closest_so_far = temp_record.t;
                hit_record = Some(temp_record);
            }
        }

        hit_record
    }
}