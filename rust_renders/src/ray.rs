use crate::{Color, Point3, Vec3};
use crate::hittable::Hittable;

#[derive(Debug, Clone)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }

    pub fn at(&self, t: f32) -> Point3 {
        self.origin + self.direction * t
    }

    pub fn ray_color<T: Hittable>(&self,
                                  world: &T,
                                  color_scale: f32,
                                  depth: u32)
                                  -> Color
    {
        if depth == 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        match world.hit(&self, 0.00001, f32::INFINITY) {
            Some(rec) => {
                let mut scattered: Ray = Ray::new(Point3::zero(), Vec3::zero());
                let mut attenuation: Color = Color::new(0.0, 0.0, 0.0);

                if rec.material.scatter(self, &rec, &mut attenuation, &mut scattered) {
                    return scattered.ray_color(world, color_scale, depth - 1) * attenuation;
                }
                attenuation
            }
            None => {
                let unit_direction = self.direction.unit_vector();
                let t = 0.5 * (unit_direction[1] + 1.0);

                Color::as_color(Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t)
            }
        }
    }
}
