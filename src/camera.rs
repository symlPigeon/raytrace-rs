use image::{ImageBuffer, Rgb};

use crate::{
    hit::HittableList,
    render::{multithread_render, singlethread_render},
    vec3::Vec3,
};

pub struct CameraConfig {
    pub pixel00_loc: Vec3,
    pub pixel_delta_u: Vec3,
    pub pixel_delta_v: Vec3,
    pub samples_per_pixel: u32,
    pub center: Vec3,
    pub max_depth: u32,
    pub aspect_ratio: f64,
    pub image_size: (u32, u32),
    pub vfov: f64,
    pub defocus_u: Vec3,
    pub defocus_v: Vec3,
    pub defocus_angle: f64,
    pub disable_progress_bar: bool,
}

impl Clone for CameraConfig {
    fn clone(&self) -> Self {
        CameraConfig {
            pixel00_loc: self.pixel00_loc,
            pixel_delta_u: self.pixel_delta_u,
            pixel_delta_v: self.pixel_delta_v,
            samples_per_pixel: self.samples_per_pixel,
            center: self.center,
            max_depth: self.max_depth,
            aspect_ratio: self.aspect_ratio,
            image_size: self.image_size,
            vfov: self.vfov,
            defocus_u: self.defocus_u,
            defocus_v: self.defocus_v,
            defocus_angle: self.defocus_angle,
            disable_progress_bar: self.disable_progress_bar,
        }
    }
}

pub struct Camera {
    output_handler: ImageBuffer<Rgb<u8>, Vec<u8>>,
    pub filepath: String,
    pub image: Vec<Vec<[u8; 3]>>,
    pub config: CameraConfig,
}

impl Camera {
    pub fn new(
        image_size: (u32, u32),
        filepath: &str,
        depth: u32,
        samples_per_pixel: u32,
        vfov: f64,
        view_position: (Vec3, Vec3, Vec3),
        defocus: (f64, f64)
    ) -> Self {
        let (width, height) = image_size;
        let (lookfrom, lookat, vup) = (view_position.0, view_position.1, view_position.2);
        let (defocus_angle, focus_dist) = (defocus.0, defocus.1);

        assert!(width > 0 && height > 0);

        let aspect_ratio = width as f64 / height as f64;
        let center = lookfrom;
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * focus_dist;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).unit();
        let u = vup.cross(&w).unit();
        let v = w.cross(&u);

        let viewport_u = u * viewport_width;
        let viewport_v = v * -viewport_height;
        let pixel_delta_u = viewport_u / width as f64;
        let pixel_delta_v = viewport_v / height as f64;
        let viewport_upper_left =
            center - w * focus_dist - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + pixel_delta_u / 2.0 + pixel_delta_v / 2.0;

        let output_handler = ImageBuffer::new(width, height);
        let filepath = filepath.to_string();
        let image = vec![vec![[0u8; 3]; width as usize]; height as usize];

        let defocus_radius = focus_dist * (defocus_angle.to_radians() / 2.0).tan();
        let defocus_u = u * defocus_radius;
        let defocus_v = v * defocus_radius;

        Camera {
            output_handler,
            filepath,
            image,
            config: CameraConfig {
                pixel00_loc,
                pixel_delta_u,
                pixel_delta_v,
                samples_per_pixel,
                center,
                max_depth: depth,
                aspect_ratio,
                image_size: (width, height),
                vfov,
                defocus_u,
                defocus_v,
                defocus_angle,
                disable_progress_bar: false,
            },
        }
    }

    pub fn disable_progress_bar(&mut self) {
        self.config.disable_progress_bar = true;
    }

    pub fn render(&mut self, world: &HittableList, multithread: bool) {
        if multithread {
            self.image = multithread_render(world, &self.config);
        } else {
            self.image = singlethread_render(world, &self.config)
        }

    }

    pub fn output(&mut self) {
        for row in 0..self.config.image_size.1 {
            for col in 0..self.config.image_size.0 {
                self.output_handler
                    .put_pixel(col, row, Rgb(self.image[row as usize][col as usize]));
            }
        }
        self.output_handler.save(&self.filepath).unwrap();
    }
}
