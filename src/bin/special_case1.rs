use ray_trace::{
    camera,
    material::materials::{Metal, Mirror},
    model::{rectangle::Rectangle, sphere::Sphere},
    vec3::Vec3,
};

fn main() {
    let wall_material = Mirror::new();
    let ball_material = Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.0);

    let mut world = ray_trace::hit::HittableList::new();

    world.add(Box::new(Rectangle::new(
        Vec3::new(-6.0, 5.0, -1.0),
        Vec3::new(0.0, -1.0, -6.0),
        Vec3::new(1.0, 0.0, 1.0),
        wall_material.clone(),
    )));
    world.add(Box::new(Rectangle::new(
        Vec3::new(0.0, 5.0, -1.0),
        Vec3::new(6.0, -1.0, -1.0),
        Vec3::new(-1.0, 0.0, 1.0),
        wall_material.clone(),
    )));
    world.add(Box::new(Rectangle::new(
        Vec3::new(-6.0, 0.0, -1.0),
        Vec3::new(6.0, -1.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
        wall_material.clone(),
    )));

    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, -0.5, -1.0),
        0.5,
        ball_material.clone(),
    )));

    let lookfrom = Vec3::new(0.0, 1.0, 1.0);
    let lookat = Vec3::new(0.0, -0.5, -1.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);

    let defocus = (10.0, 3.4);

    let mut camera = camera::Camera::new(
        (1920, 1080),
        "output-case-1.png",
        5,
        100,
        90.0,
        (lookfrom, lookat, vup),
        defocus
    );
    camera.render(&world, true);
    camera.output();
}
