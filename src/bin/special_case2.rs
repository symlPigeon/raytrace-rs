use ray_trace::{camera, material::materials::Lambertian, model::sphere::Sphere, vec3::Vec3};

fn main() {
    let mut world = ray_trace::hit::HittableList::new();
    let material_left = Lambertian::new(Vec3::new(0.0, 0.0, 1.0));
    let material_right = Lambertian::new(Vec3::new(1.0, 0.0, 0.0));

    let r = (std::f64::consts::PI / 4.0).cos();
    world.add(Box::new(Sphere::new(
        Vec3::new(-r, 0.0, -1.0),
        r,
        material_left.clone(),
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(r, 0.0, -1.0),
        r,
        material_right.clone(),
    )));

    let lookat = Vec3::new(0.0, 0.0, -1.0);
    let lookfrom = Vec3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);

    let defocus = (10.0, 3.4);

    let mut camera = camera::Camera::new(
        (1920, 1080),
        "fov-test.png",
        50,
        100,
        90.0,
        (lookfrom, lookat, vup),
        defocus
    );
    camera.render(&world, true);
    camera.output();
}
