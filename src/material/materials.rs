use std::sync::Arc;

use rand::random;

use crate::{ray::Ray, hit::HitRecord, vec3::Vec3};

pub trait Material: Send + Sync {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)>;
}

pub struct Lambertian {
    pub albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Arc<Self> {
        Arc::new(Lambertian { albedo })
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)> {
        let _ = ray_in;
        let mut scatter_direction = hit_record.normal + Vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal;
        }
        let scattered = Ray::new(hit_record.p, scatter_direction);
        let attenuation = self.albedo;
        Some((attenuation, scattered))
    }   
}

pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f64
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f64) -> Arc<Self> {
        Arc::new(Metal { 
            albedo,  
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 }
        })
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)> {
        let reflected = ray_in.direction.unit().reflect(&hit_record.normal);
        let scattered = Ray::new(hit_record.p, reflected + Vec3::random_in_unit_sphere() * self.fuzz);
        let attenuation = self.albedo;
        if scattered.direction.dot(&hit_record.normal) > 0.0 {
            return Some((attenuation, scattered))
        }
        None        
    }
}

pub struct Dielectric {
    pub ir: f64
}

impl Dielectric {
    pub fn new(ir: f64) -> Arc<Self> {
        Arc::new(Dielectric { ir })
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)> {
        let attenuation = Vec3::new(1.0, 1.0, 1.0);
        let refraction_ratio = if hit_record.front_face { 1.0 / self.ir } else { self.ir };
        let unit_direction = ray_in.direction.unit();

        let cos_theta = (unit_direction * -1.0).dot(&hit_record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let reflectance = || {
            let r0 = (1.0 - refraction_ratio) / (1.0 + refraction_ratio);
            let r0 = r0 * r0;
            r0 + (1.0 - r0) * (1.0 - cos_theta).powi(5)
        };

        let direction = if refraction_ratio * sin_theta > 1.0 || reflectance() > random::<f64>() {
            unit_direction.reflect(&hit_record.normal)
        } else {
            unit_direction.refract(&hit_record.normal, refraction_ratio)
        };
        
        let scattered = Ray::new(hit_record.p, direction);
        Some((attenuation, scattered))
    }
}

pub struct Mirror {}

impl Mirror {
    pub fn new() -> Arc<Self> {
        Arc::new(Mirror {})
    }
}

impl Material for Mirror {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)> {
        let reflected = ray_in.direction.unit().reflect(&hit_record.normal);
        let scattered = Ray::new(hit_record.p, reflected);
        let attenuation = Vec3::new(1.0, 1.0, 1.0);
        if scattered.direction.dot(&hit_record.normal) > 0.0 {
            return Some((attenuation, scattered))
        }
        None
    }
}