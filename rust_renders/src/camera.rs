pub mod camera {
    use crate::vec3::vec3::{Point3, Vec3};
    use crate::ray::ray::Ray;
    use crate::image_config::image_config::ImageConfig;

    #[derive(Debug, Clone)]
    pub struct Camera {
        origin: Point3,
        lower_left_corner: Point3,
        horizontal: Vec3,
        vertical: Vec3,
        w: Vec3,
        u: Vec3,
        v: Vec3,
        lens_radius: f32
    }

    impl Camera {
        pub fn new(look_from: Point3,
                   look_at: Point3,
                   vup: Vec3,
                   vfov: f32,
                   aspect_ratio: f32,
                   aperture: f32,
                   focus_dist: f32)
                   -> Camera
        {
            let theta = vfov.to_radians();
            let h = (theta / 2.0).tan();

            let viewport_height = 2.0f32 * h;
            let viewport_width = aspect_ratio * viewport_height;

            let w = (look_from - look_at).unit_vector();
            let u = vup.cross(&w).unit_vector();
            let v = w.cross(&u);

            let origin = look_from;
            let horizontal = u * viewport_width * focus_dist;
            let vertical = v * viewport_height * focus_dist;
            let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w * focus_dist;

            let lens_radius = aperture / 2.;
            Camera{origin, lower_left_corner, horizontal, vertical, w, u, v, lens_radius}
        }

        pub fn get_ray(&self, u: f32, v: f32) -> Ray {
            let rd = Vec3::rand_unit() * self.lens_radius;
            let offset = self.u * rd[0] + self.v * rd[1];

            let direction = self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin - offset;
            Ray::new(self.origin + offset, direction)
        }
    }
}