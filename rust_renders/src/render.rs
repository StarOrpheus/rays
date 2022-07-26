use crate::{Camera, Color, HittableArray, ImageConfig};
use crate::progress_listener::ProgressListener;

pub fn render_fn<T: ProgressListener>(progress: &mut T,
                                      config: &ImageConfig,
                                      camera: &Camera,
                                      world: &HittableArray)
                                      -> Vec<Color> {
    let color_scale_factor = 1.0 / config.samples_per_pixel as f32;

    let mut result = Vec::new();
    for j in (0..config.height).rev() {
        progress.update(j as usize, config.height as usize);
        for i in 0..config.width {
            let mut color = Color::new(0.0, 0.0, 0.0);
            for _s in 0..config.samples_per_pixel {
                let u = ((i as f32) + rand::random::<f32>()) / ((config.width - 1) as f32);
                let v = ((j as f32) + rand::random::<f32>()) / ((config.height - 1) as f32);

                let r = camera.get_ray(u, v);
                let new_color = r.ray_color(world, color_scale_factor, config.depth);
                color += new_color;
            }

            result.push(color);
        }
    }

    debug_assert_eq!(result.len(), (config.height * config.width) as usize);

    result
}


mod tests {
    use std::rc::Rc;
    use crate::{Camera, Color, Glass, HittableArray, ImageConfig, Lambertian, Light, Metal, Point3, render_fn, Sphere, StderrListener, Vec3};

    #[test]
    fn render_sample() {
        let mut config = ImageConfig::default_config();
        // config.height = 480;
        // config.width = 640;

        let look_from = Point3::new(-2.0, 2.0, 1.0);
        let look_at = Point3::new(0.0, 0.0, -1.0);

        let camera = Camera::new(
            look_from,
            look_at,
            Vec3::new(0., 1., 0.),
            70.,
            config.aspect_ratio(),
            1.0 / 6.,
            (look_from - Vec3::new(0.0, 0.0, -1.0)).length()
        );

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

        let result = render_fn(&mut StderrListener{}, &config, &camera, &world);
    }
}
