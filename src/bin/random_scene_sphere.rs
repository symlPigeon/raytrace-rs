use ray_trace::{camera::Camera, material::materials::{Lambertian, Metal, Dielectric}, model::sphere::Sphere, vec3::Vec3, hit::HittableList};


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
                    world.add(Box::new(Sphere::new(
                        center,
                        0.2,
                        sphere_material.clone(),
                    )));
                } else if choose_mat < 0.95 {
                    let albedo = Vec3::random(0.5, 1.0);
                    let fuzz = rand::random::<f64>() * 0.5;
                    let sphere_material = Metal::new(albedo, fuzz);
                    world.add(Box::new(Sphere::new(
                        center,
                        0.2,
                        sphere_material.clone(),
                    )));
                } else {
                    let sphere_material = Dielectric::new(1.5);
                    world.add(Box::new(Sphere::new(
                        center,
                        0.2,
                        sphere_material.clone(),
                    )));
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

    let image_size = (1920_u32, 1080_u32);
    let samples_per_pixel = 100;
    let max_depth = 50;
    let vfov = 20.0;
    let lookfrom = Vec3::new(13.0, 2.0, 3.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let defocus = (0.6, 10.0);
    let mut camera = Camera::new(
        image_size,
        "random_scene_sphere_1.png",
        max_depth,
        samples_per_pixel,
        vfov,
        (lookfrom, lookat, vup),
        defocus
    );

    let start_time = std::time::Instant::now();
    camera.render(&world, false);
    let end_time = std::time::Instant::now();
    println!(
        "Single Thread render time: {} seconds",
        (end_time - start_time).as_secs_f64()
    );
    camera.output();

    let mut camera = Camera::new(
        image_size,
        "random_scene_sphere_2.png",
        max_depth,
        samples_per_pixel,
        vfov,
        (lookfrom, lookat, vup),
        defocus
    );

    let start_time = std::time::Instant::now();
    camera.render(&world, true);
    let end_time = std::time::Instant::now();
    println!(
        "Multithread render time: {} seconds",
        (end_time - start_time).as_secs_f64()
    );
    camera.output();
}