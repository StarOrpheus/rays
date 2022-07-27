use std::borrow::{Borrow, BorrowMut};
use std::sync::{Arc, Mutex};

use threadpool::ThreadPool;

use crate::{Camera, Color, HittableArray, ImageConfig};

pub fn render_fn(config: Arc<ImageConfig>,
                 camera: Arc<Camera>,
                 world: Arc<HittableArray>)
                 -> Vec<Color> {
    let color_scale_factor = 1.0 / config.samples_per_pixel as f32;
    let pool = ThreadPool::with_name("raytracer worker".to_string(),
                                     num_cpus::get());

    let mut result = Vec::new();
    let _ = (0..(config.height * config.width)).for_each(|_x| { result.push(Color::new(0., 0., 0.)) });
    let result = Arc::new(Mutex::new(result));
    // let parts = (0..config.height).ch
    for j in 0..config.height {
        let config = config.clone();
        let world = world.clone();
        let camera = camera.clone();
        let result = result.clone();
        pool.execute(move || {
            let config: &ImageConfig = config.borrow();
            let world: &HittableArray = world.borrow();
            let camera: &Camera = camera.borrow();

            let mut temp_result = Vec::new();
            for i in 0..config.width {
                let mut color = Color::new(0.0, 0.0, 0.0);
                for _s in 0..config.samples_per_pixel {
                    let u = ((i as f32) + rand::random::<f32>()) / ((config.width - 1) as f32);
                    let v = ((j as f32) + rand::random::<f32>()) / ((config.height - 1) as f32);

                    let r = camera.get_ray(u, v);
                    let borrowed_world: &HittableArray = world.borrow();
                    let new_color = r.ray_color(borrowed_world, color_scale_factor, config.depth);
                    color += new_color;
                }

                temp_result.push(color);
            }

            let mut result_lock = result.lock().unwrap();
            let result: &mut Vec<Color> = result_lock.borrow_mut();
            for i in 0..config.width {
                result[(j * config.width + i) as usize] = temp_result[i as usize].clone();
            }
        });
    }

    pool.join();

    let result: Vec<Color> = Arc::try_unwrap(result).unwrap().into_inner().unwrap();
    debug_assert_eq!(result.len(), (config.height * config.width) as usize);
    result
}


mod tests {
    use std::sync::Arc;

    use crate::{Camera, Color, Glass, HittableArray, ImageConfig, Lambertian, Light, Metal, Point3, render_fn, Sphere, Vec3};

    #[test]
    fn render_sample() {
        let mut config = Arc::new(ImageConfig::default_config());
        // config.height = 480;
        // config.width = 640;

        let look_from = Point3::new(-2.0, 2.0, 1.0);
        let look_at = Point3::new(0.0, 0.0, -1.0);

        let camera = Arc::new(Camera::new(
            look_from,
            look_at,
            Vec3::new(0., 1., 0.),
            70.,
            config.aspect_ratio(),
            1.0 / 8.,
            (look_from - Vec3::new(0.0, 0.0, -1.0)).length(),
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
        world.add(Arc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, center_sphere_material)));
        world.add(Arc::new(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, left_sphere_material)));
        world.add(Arc::new(Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, right_sphere_material)));
        world.add(Arc::new(Sphere::new(Point3::new(3.0, 0.0, 1.0), 0.5, light_source_material)));
        let world = Arc::new(world);

        let _result = render_fn(config, camera, world);
    }
}
