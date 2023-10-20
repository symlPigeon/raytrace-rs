use crate::hit::HittableList;
use crate::interval::Interval;
use crate::vec3::Vec3;
use crate::ray::Ray;

pub fn ray_color(ray: &Ray, world: &HittableList) -> Vec3 {
    if let Some(hit_record) = world.hit(ray, &Interval::new(0.0, f64::INFINITY)) {
        return (hit_record.normal + Vec3::new(1.0, 1.0, 1.0)) * 0.5;
    }

    let unit_direction = ray.direction.unit();
    let a = 0.5 * (unit_direction.y + 1.0);
    Vec3::new(1.0, 1.0, 1.0) * (1.0 - a) + Vec3::new(0.5, 0.7, 1.0) * a
}