use ray_trace::{
    camera::Camera,
    hit::HittableList,
    material::materials::{Dielectric, Lambertian, Metal, Mirror},
    model::{rectangle::Rectangle, sphere::Sphere, triangle::Triangle},
    vec3::Vec3,
};

fn main() {
    let mut world = HittableList::new();

    let material_ground = Lambertian::new(Vec3::new(0.8, 0.8, 0.0));
    let material_center = Lambertian::new(Vec3::new(0.1, 0.2, 0.5));
    let material_left = Dielectric::new(1.5);
    let material_right = Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.0);

    let l1 = Vec3::new(-1.0, 0.0, -1.0);
    let l2 = Vec3::new(1.0, 0.0, -1.0);
    let l3 = Vec3::new(0.0, 0.0, 1.0);
    let l4 = Vec3::new(0.0, 1.0, 0.0);

    world.add(Box::new(Triangle::new(
        l1,
        l2,
        l3,
        material_center.clone(),
    )));
    world.add(Box::new(Triangle::new(
        l1,
        l2,
        l4,
        material_center.clone(),
    )));
    world.add(Box::new(Triangle::new(
        l1,
        l3,
        l4,
        material_center.clone(),
    )));
    world.add(Box::new(Triangle::new(
        l2,
        l3,
        l4,
        material_center.clone(),
    )));

    let world = world;

    let lookfrom = Vec3::new(-2.0, 2.0, 1.0);
    let lookat = Vec3::new(0.0, 0.0, -1.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);

    let defocus = (0.6, 3.4);

    let mut camera = Camera::new(
        (1920, 1080),
        "output.png",
        50,
        100,
        90.0,
        (lookfrom, lookat, vup),
        defocus,
    );

    let start_time = std::time::Instant::now();
    camera.render(&world, true);
    let end_time = std::time::Instant::now();
    println!(
        "Render time: {} seconds",
        (end_time - start_time).as_secs_f64()
    );

    camera.output();
}
