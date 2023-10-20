use crate::{camera::CameraConfig, hit::HittableList, interval::Interval, ray::Ray, vec3::Vec3};
use indicatif::ProgressBar;
use rand::random;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

pub fn render_pixel(col: u32, row: u32, world: &HittableList, config: &CameraConfig) -> [u8; 3] {
    let mut pixel_color = Vec3::new_zero();

    // repeat random sample to achieve anti-aliasing
    for _ in 0..config.samples_per_pixel {
        let ray = get_ray(col, row, config);
        pixel_color += ray_color(&ray, config.max_depth, world);
    }

    let this = &pixel_color;
    let sample_per_pixel = config.samples_per_pixel;
    let (mut r, mut g, mut b) = (this.x, this.y, this.z);

    let scale = 1.0 / sample_per_pixel as f64;
    r *= scale;
    g *= scale;
    b *= scale;

    let intv = Interval::new(0.0, 0.999);
    [
        (256.0 * intv.clamp(r)) as u8,
        (256.0 * intv.clamp(g)) as u8,
        (256.0 * intv.clamp(b)) as u8,
    ]
}

fn defocus_disk_sample(config: &CameraConfig) -> Vec3 {
    let p = Vec3::random_in_unit_disk();
    config.center + (config.defocus_u * p.x) + (config.defocus_v * p.y)
}

pub fn get_ray(u: u32, v: u32, config: &CameraConfig) -> Ray {
    let mut pixel_center =
        config.pixel00_loc + (config.pixel_delta_u * u as f64) + (config.pixel_delta_v * v as f64);
    let (px, py) = (-0.5 + random::<f64>(), -0.5 + random::<f64>());
    pixel_center += (config.pixel_delta_u * px) + (config.pixel_delta_v * py);

    let ray_origin = if config.defocus_angle <= 0.0 {
        config.center
    } else {
        defocus_disk_sample(config)
    };
    let ray_direction = pixel_center - ray_origin;

    Ray::new(ray_origin, ray_direction)
}

pub fn ray_color(ray: &Ray, depth: u32, world: &HittableList) -> Vec3 {
    if depth == 0 {
        // exceeded recursion
        return Vec3::new_zero();
    }
    if let Some(hit_record) = world.hit(ray, &Interval::new(0.001, f64::INFINITY)) {
        if let Some((attenutation, scatter)) = hit_record.material.scatter(ray, &hit_record) {
            return ray_color(&scatter, depth - 1, world) * attenutation;
        }
        return Vec3::new_zero();
    }

    // background color
    let unit_direction = ray.direction.unit();
    let a = 0.5 * (unit_direction.y + 1.0);
    Vec3::new(1.0, 1.0, 1.0) * (1.0 - a) + Vec3::new(0.5, 0.7, 1.0) * a
}

pub fn multithread_render(world: &HittableList, config: &CameraConfig) -> Vec<Vec<[u8; 3]>> {
    let mut image =
        vec![vec![[0u8; 3]; config.image_size.0 as usize]; config.image_size.1 as usize];
    let row_iter: Vec<_> = image.chunks_exact_mut(1).enumerate().collect();
    let progress_bar = ProgressBar::new(config.image_size.1 as u64);
    row_iter.into_par_iter().for_each(|(row, chunk)| {
        for col in 0..config.image_size.0 {
            let pixel_color = render_pixel(col, row as u32, world, config);
            chunk[0][col as usize] = pixel_color;
        }
        progress_bar.inc(1);
    });
    progress_bar.finish();
    image
}

pub fn singlethread_render(world: &HittableList, config: &CameraConfig) -> Vec<Vec<[u8; 3]>> {
    let mut image =
        vec![vec![[0u8; 3]; config.image_size.0 as usize]; config.image_size.1 as usize];
    let progress_bar = ProgressBar::new(config.image_size.1 as u64);
    for row in 0..config.image_size.1 {
        for col in 0..config.image_size.0 {
            let pixel_color = render_pixel(col, row, world, config);
            image[row as usize][col as usize] = pixel_color;
        }
        progress_bar.inc(1);
    }
    progress_bar.finish();
    image
}
