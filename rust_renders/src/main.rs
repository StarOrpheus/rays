use crate::vec3::vec3::{Vec3, Point3, Color};
use crate::sphere::sphere::Sphere;
use crate::hittable::hittable::{HittableArray};
use crate::camera::camera::Camera;
use crate::image_config::image_config::ImageConfig;
use crate::material::material::{Lambertian, Metal, Glass, Light};
use rayon::prelude::*;
use std::sync::{Mutex};
use std::rc::Rc;

mod vec3;
mod ray;
mod hittable;
mod sphere;
mod camera;
mod image_config;
mod material;

fn main() {
    let config = ImageConfig::default_config();

    let look_from = Point3::new(-2.0, 2.0, 1.0);
    let look_at = Point3::new(0.0, 0.0, -1.0);

    let camera = Camera::new(
        look_from,
        look_at,
        Vec3::new(0., 1., 0.),
        70.,
        ImageConfig::default_config().aspect_ratio,
        1.0 / 6.,
        (look_from - Vec3::new(0.0, 0.0, -1.0)).length()
    );

    let color_scale_factor = 1.0 / config.samples_per_pixel as f32;
    let ground_material
        = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let center_sphere_material
        = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let left_sphere_material
        = Rc::new(Glass::new(1.5));
    let right_sphere_material
        = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.3), 1.0));
    let light_source_material
        = Rc::new(Light::new(Color::new(0.999, 0.996, 0.95)));

    let mut world = HittableArray::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, ground_material)));
    world.add(Box::new(Sphere::new(Point3::new(0.0,    0.0, -1.0), 0.5, center_sphere_material)));
    world.add(Box::new(Sphere::new(Point3::new(-1.0,   0.0, -1.0), 0.5, left_sphere_material)));
    world.add(Box::new(Sphere::new(Point3::new(1.0,    0.0, -1.0), 0.5, right_sphere_material)));
    world.add(Box::new(Sphere::new(Point3::new(3.0,    0.0,  1.0), 0.5, light_source_material)));
    let world = world;

    print!("P3\n{} {}\n255\n", config.width, config.height);

    let mut result: Vec<(usize, Vec<Color>)> = Vec::new();

    for j in (0..config.height - 1).rev() {
        eprint!("\rScanlines remaining: {}   ", j);
        for i in 0..config.width {
            let mut color = Color::new(0.0, 0.0, 0.0);
            for s in 0..config.samples_per_pixel {
                let u = ((i as f32) + rand::random::<f32>()) / ((config.width - 1) as f32);
                let v = ((j as f32) + rand::random::<f32>()) / ((config.height - 1) as f32);

                let r = camera.get_ray(u, v);
                let new_color = r.ray_color(&world, color_scale_factor, config.depth);
                color += new_color;
            }
            println!("{}", color);
        }
    }
}
