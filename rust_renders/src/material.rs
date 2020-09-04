pub mod material {
    use crate::ray::ray::Ray;
    use crate::hittable::hittable::HitRecord;
    use crate::vec3::vec3::{Color, Vec3};
    use std::fmt::Debug;
    use rand::random;

    pub trait Material : Debug + Send {
        fn scatter(&self,
                   r_in: &Ray,
                   hit_rec: &HitRecord,
                   attenuation: &mut Color,
                   r_out: &mut Ray)
                   -> bool;
    }

    #[derive(Debug, Clone)]
    pub struct Lambertian {
        albedo: Color
    }

    impl Lambertian {
        pub fn new(albedo: Color) -> Lambertian {
            Lambertian { albedo }
        }
    }

    impl Material for Lambertian {
        fn scatter(&self,
                   r_in: &Ray,
                   hit_rec: &HitRecord,
                   attenuation: &mut Color,
                   r_out: &mut Ray)
                   -> bool
        {
            let scatter_direction = hit_rec.normal + Vec3::rand_unit_sphere();
            *r_out = Ray::new(hit_rec.p, scatter_direction);
            *attenuation = self.albedo.clone();
            true
        }
    }

    #[derive(Debug, Clone)]
    pub struct Metal {
        albedo: Color,
        fuzz: f32,
    }

    impl Metal {
        pub fn new(albedo: Color, fuzz: f32) -> Metal {
            Metal { albedo, fuzz }
        }
    }

    impl Material for Metal {
        fn scatter(&self,
                   r_in: &Ray,
                   hit_rec: &HitRecord,
                   attenuation: &mut Color,
                   r_out: &mut Ray)
                   -> bool
        {
            let reflected = Vec3::reflect(&r_in.direction.unit_vector(), &hit_rec.normal);
            *r_out = Ray::new(hit_rec.p, reflected + Vec3::rand_unit_sphere() * self.fuzz);
            *attenuation = self.albedo.clone();
            r_out.direction.dot(hit_rec.normal) > 0.0
        }
    }

    #[derive(Debug, Clone)]
    pub struct Glass {
        /// Refraction index
        ref_idx: f32
    }

    impl Glass {
        pub fn new(ref_idx: f32) -> Glass {
            Glass { ref_idx }
        }

        fn shlick_probability(cosine: f32, ref_idx: f32) -> f32 {
            let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
            r0 *= r0;
            r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
        }
    }

    impl Material for Glass {
        fn scatter(&self,
                   r_in: &Ray,
                   hit_rec: &HitRecord,
                   attenuation: &mut Color,
                   r_out: &mut Ray)
                   -> bool
        {
            *attenuation = Color::new(1.0, 1.0, 1.0);
            let etai_over_etat = if hit_rec.front_face { 1.0 / self.ref_idx } else { self.ref_idx };
            let unit_direction = r_in.direction.unit_vector();

            let cos_theta = (-unit_direction).dot(hit_rec.normal).min(1.0);
            let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

            if etai_over_etat * sin_theta > 1.0
                || random::<f32>() < Glass::shlick_probability(cos_theta, self.ref_idx) {
                let reflected = Vec3::reflect(&unit_direction, &hit_rec.normal);
                *r_out = Ray::new(hit_rec.p, reflected);
                return true;
            }

            let refracted = Vec3::refract(&unit_direction, &hit_rec.normal, etai_over_etat);
            *r_out = Ray::new(hit_rec.p, refracted);

            true
        }
    }

    #[derive(Debug, Clone)]
    pub struct Light {
        col: Color
    }

    impl Material for Light {
        fn scatter(&self, r_in: &Ray, hit_rec: &HitRecord, attenuation: &mut Color, r_out: &mut Ray) -> bool {
            *attenuation = self.col.clone();
            false
        }
    }

    impl Light {
        pub fn new(col: Color) -> Light {
            Light { col }
        }
    }
}