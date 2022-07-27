use std::borrow::Borrow;
use std::sync::Arc;
use crate::camera::Camera;
use crate::hittable::{HittableArray};
use crate::image_config::ImageConfig;
use crate::material::{Glass, Lambertian, Light, Material, Metal};
use crate::render::render_fn;
use crate::sphere::Sphere;
use crate::vec3::{Color, Point3, Vec3};

mod vec3;
mod ray;
mod hittable;
mod sphere;
mod camera;
mod image_config;
mod material;
mod render;

fn main() {
    let mut config = Arc::new(ImageConfig::default_config());

    let look_from = Point3::new(-2.0, 2.0, 1.0);
    let look_at = Point3::new(0.0, 0.0, -1.0);

    let mut camera = Arc::new(Camera::new(
        look_from,
        look_at,
        Vec3::new(0., 1., 0.),
        70.,
        config.aspect_ratio(),
        1.0 / 6.,
        (look_from - Vec3::new(0.0, 0.0, -1.0)).length()
    ));

    let ground_material
        = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let center_sphere_material
        = Arc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let left_sphere_material
        = Arc::new(Glass::new(1.5));
    let right_sphere_material
        = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.3), 1.0));
    let light_source_material
        = Arc::new(Light::new(Color::new(0.999, 0.996, 0.95)));

    let mut world = HittableArray::new();
    world.add(Arc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, ground_material)));
    world.add(Arc::new(Sphere::new(Point3::new(0.0,    0.0, -1.0), 0.5, center_sphere_material)));
    world.add(Arc::new(Sphere::new(Point3::new(-1.0,   0.0, -1.0), 0.5, left_sphere_material)));
    world.add(Arc::new(Sphere::new(Point3::new(1.0,    0.0, -1.0), 0.5, right_sphere_material)));
    world.add(Arc::new(Sphere::new(Point3::new(3.0,    0.0,  1.0), 0.5, light_source_material)));
    let world = Arc::new(world);

    let result = render_fn(config.clone(), camera, world);
    let config: &ImageConfig = config.borrow();
    print!("P3\n{} {}\n255\n", config.width, config.height);
    for j in (0..config.height).rev() {
        for i in 0..config.width {
            println!("{}", result[(j * config.width + i) as usize]);
        }
    }
}
