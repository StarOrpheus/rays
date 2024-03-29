pub mod hittable {
    use crate::vec3::vec3::{Point3, Vec3};
    use crate::ray::ray::Ray;
    use crate::material::material::Material;
    use std::rc::Rc;
    use std::fmt::Debug;

    #[derive(Debug, Clone)]
    pub struct HitRecord {
        pub p: Point3,
        pub normal: Vec3,
        pub t: f32,
        pub material: Rc<dyn Material>,
        pub front_face: bool
    }

    pub trait Hittable {
        fn hit(&self,
               r: &Ray,
               t_min: f32,
               t_max: f32)
            -> Option<HitRecord>;
    }

    impl HitRecord {
        pub fn new(p: Point3,
                   normal: Vec3,
                   t: f32,
                   material: Rc<dyn Material>)
            -> HitRecord
        {
            HitRecord{p, normal, t, material, front_face: false}
        }

        pub fn set_face_normal(&mut self,
                               r: &Ray,
                               outward_normal: &Vec3)
        {
            self.front_face = r.direction.dot(*outward_normal) < 0.0;
            self.normal = if self.front_face { outward_normal.clone() } else { -*outward_normal };
        }
    }

    pub struct HittableArray {
        imp: Vec<Box<dyn Hittable>>
    }

    impl HittableArray {
        pub fn new() -> HittableArray {
            HittableArray{
                imp: Vec::new()
            }
        }

        pub fn len(&self) -> usize {
            self.imp.len()
        }

        pub fn empty(&self) -> bool {
            self.imp.is_empty()
        }

        pub fn clear(&mut self) {
            self.imp.clear();
        }

        pub fn add(&mut self, obj: Box<dyn Hittable>) {
            self.imp.push(obj);
        }
    }

    impl Hittable for HittableArray {
        fn hit(&self,
               r: &Ray,
               t_min: f32,
               t_max: f32)
            -> Option<HitRecord>
        {
            let mut closest_so_far = t_max;
            let mut result = None;

            for obj in &self.imp {
                match obj.hit(&r, t_min, closest_so_far) {
                    None => {},
                    Some(rec) => {
                        closest_so_far = rec.t;
                        result = Some(rec);
                    }
                }
            }

            result
        }
    }

    unsafe impl Send for HittableArray {}
}
