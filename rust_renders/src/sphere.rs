use std::rc::Rc;
use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::Point3;
use crate::ray::Ray;

#[derive(Debug, Clone)]
pub struct Sphere {
    center: Point3,
    radius: f32,
    material: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3,
               radius: f32,
               material: Rc<dyn Material>)
               -> Sphere
    {
        Sphere { center, radius, material }
    }
}

impl Hittable for Sphere {
    fn hit(&self,
           r: &Ray,
           t_min: f32,
           t_max: f32)
           -> Option<HitRecord>
    {
        let a = r.direction.length_squared();
        let oc = r.origin - self.center;
        let half_b = oc.dot(r.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let descriminant = half_b * half_b - a * c;

        if descriminant > 0.0 {
            let root = descriminant.sqrt();

            let temp = (-half_b - root) / a;
            if temp < t_max && temp > t_min {
                let p = r.at(temp);
                let outward_normal = (p - self.center) / self.radius;
                let mut result = HitRecord::new(p, (p - self.center) / self.radius, temp, self.material.clone());
                result.set_face_normal(r, &outward_normal);
                return Some(result);
            }

            let temp = (-half_b + root) / a;
            if temp < t_max && temp > t_min {
                let p = r.at(temp);
                let outward_normal = (p - self.center) / self.radius;
                let mut result = HitRecord::new(p, (p - self.center) / self.radius, temp, self.material.clone());
                result.set_face_normal(r, &outward_normal);
                return Some(result);
            }
        }

        return None;
    }
}

