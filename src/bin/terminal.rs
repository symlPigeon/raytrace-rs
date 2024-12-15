use std::{f64::consts::PI, io::{stdout, Write}};

use libc::{ioctl, winsize, STDOUT_FILENO, TIOCGWINSZ};
use ray_trace::{
    camera::Camera,
    hit::HittableList,
    material::materials::{Dielectric, Lambertian, Metal},
    model::sphere::Sphere,
    vec3::Vec3,
};

fn get_terminal_size() -> Option<(u16, u16)> {
    unsafe {
        let mut size: winsize = std::mem::zeroed();
        if ioctl(STDOUT_FILENO, TIOCGWINSZ, &mut size) == 0 {
            Some((size.ws_col, size.ws_row))
        } else {
            None
        }
    }
}

fn clear_screen() {
    print!("\x1B[2J\x1B[H");
    stdout().flush().unwrap();
}

fn main() {
    let mut world = HittableList::new();

    let material_ground = Lambertian::new(Vec3::new(0.5, 0.5, 0.5));
    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        material_ground.clone(),
    )));
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rand::random::<f64>();
            let center = Vec3::new(
                a as f64 + 0.9 * rand::random::<f64>(),
                0.2,
                b as f64 + 0.9 * rand::random::<f64>(),
            );

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = Vec3::random_vec() * Vec3::random_vec();
                    let sphere_material = Lambertian::new(albedo);
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material.clone())));
                } else if choose_mat < 0.95 {
                    let albedo = Vec3::random(0.5, 1.0);
                    let fuzz = rand::random::<f64>() * 0.5;
                    let sphere_material = Metal::new(albedo, fuzz);
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material.clone())));
                } else {
                    let sphere_material = Dielectric::new(1.5);
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material.clone())));
                }
            }
        }
    }

    let material_left = Dielectric::new(1.5);
    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        material_left.clone(),
    )));
    let material_center = Lambertian::new(Vec3::new(0.4, 0.2, 0.1));
    world.add(Box::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        material_center.clone(),
    )));
    let material_right = Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0);
    world.add(Box::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        material_right.clone(),
    )));

    let mut angle: f64 = 0.0;
    while true {

        let image_size = get_terminal_size().unwrap();
        let image_size = (image_size.0 as u32 / 2, image_size.1 as u32 - 2_u32); 

        let start_time = std::time::Instant::now();
        let mut camera = Camera::new(
            image_size,
            "",
            5,
            10,
            40.0,
            (
                Vec3::new(10.0 * angle.cos(), 2.0, 10.0 * angle.sin()),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
            ),
            (0.6, 10.0),
        );
        camera.disable_progress_bar();
        camera.render(&world, true);
        let elapsed = start_time.elapsed().as_millis();
        angle += PI * elapsed as f64 / 1000.0 / 10.0;

        clear_screen();
        println!("Rendered in {:.3}ms, fps: {}", elapsed, 1000.0 / elapsed as f64);

        let image = camera.image;
        let mut buffer = String::new();
        for row in image.iter() {
            for pixel in row.iter() {
                // buffer.push_str(&format!("\x1B[48;2;{};{};{}m  ", pixel[0], pixel[1], pixel[2]));
                // // reset color
                // buffer.push_str("\x1B[0m");
                let platte = "@%#*+=-:.   ";
                let luminance = 0.299 * pixel[0] as f64 + 0.587 * pixel[1] as f64 + 0.114 * pixel[2] as f64;
                let index = (luminance * (platte.len() - 1) as f64 / 255.0) as usize;
                buffer.push(platte.chars().nth(index).unwrap());
                buffer.push(platte.chars().nth(index).unwrap());
            }
            buffer.push('\n');
        }
        print!("{}", buffer);
        stdout().flush().unwrap();
    }
}
