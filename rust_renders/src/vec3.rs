pub mod vec3 {
    use std::ops::{Mul, Add, Div, MulAssign, AddAssign, DivAssign, Neg, Index, Sub, SubAssign};
    use std::fmt;
    use rand::{Rng, random};
    use crate::image_config::image_config::ImageConfig;

    #[cfg(target_arch = "x86")]
    use std::arch::x86::*;
    #[cfg(target_arch = "x86_64")]
    use std::arch::x86_64::*;

    #[cfg(feature = "enable_sse")]
    #[derive(Debug, Copy, Clone)]
    pub struct Vec3 {
        a: [f32; 4]
    }

    #[cfg(not(feature = "enable_sse"))]
    #[derive(Debug, Copy, Clone)]
    pub struct Vec3 {
        x: f32,
        y: f32,
        z: f32,
    }

    #[derive(Debug, Clone)]
    pub struct Color {
        imp: Vec3
    }

    impl Color {
        pub fn new(r: f32, g: f32, b: f32) -> Color {
            Color { imp: Vec3::new(r, g, b) }
        }

        pub fn as_color(v: Vec3) -> Color {
            Color { imp: v }
        }
    }

    impl Mul for Color {
        type Output = Color;

        fn mul(self, rhs: Self) -> Self::Output {
            Color { imp: self.imp * rhs.imp }
        }
    }

    impl Add for Color {
        type Output = Color;

        fn add(self, rhs: Self) -> Self::Output {
            Color { imp: self.imp + rhs.imp }
        }
    }

    impl AddAssign for Color {
        fn add_assign(&mut self, rhs: Self) {
            self.imp += rhs.imp;
        }
    }

    impl Mul<f32> for Color {
        type Output = Color;

        fn mul(self, rhs: f32) -> Self::Output {
            Color { imp: self.imp * rhs }
        }
    }

    impl Mul<f32> for Vec3 {
        type Output = Vec3;

        #[cfg(feature = "enable_sse")]
        fn mul(self, rhs: f32) -> Self::Output {
            let mut res = Vec3 { a: [0.; 4] };
            unsafe {
                let first_arg = _mm_loadu_ps(self.a.as_ptr());
                let sec_arg = _mm_set1_ps(rhs);
                let result = _mm_mul_ps(first_arg, sec_arg);
                _mm_storeu_ps(res.a.as_mut_ptr(), result);
            }
            res
        }

        #[cfg(not(feature = "enable_sse"))]
        fn mul(self, rhs: f32) -> Self::Output {
            Vec3 {
                x: self.x * rhs,
                y: self.y * rhs,
                z: self.z * rhs,
            }
        }
    }

    impl Mul for Vec3 {
        type Output = Vec3;

        #[cfg(feature = "enable_sse")]
        fn mul(self, rhs: Self) -> Self::Output {
            let mut res = Vec3 { a: [0.; 4] };
            unsafe {
                let first_arg = _mm_loadu_ps(self.a.as_ptr());
                let sec_arg = _mm_loadu_ps(rhs.a.as_ptr());
                let result = _mm_mul_ps(first_arg, sec_arg);
                _mm_storeu_ps(res.a.as_mut_ptr(), result);
            }
            res
        }

        #[cfg(not(feature = "enable_sse"))]
        fn mul(self, rhs: Self) -> Self::Output {
            Vec3 {
                x: self.x * rhs.x,
                y: self.y * rhs.y,
                z: self.z * rhs.z,
            }
        }
    }

    impl MulAssign<f32> for Vec3 {
        #[cfg(feature = "enable_sse")]
        fn mul_assign(&mut self, rhs: f32) {
            unsafe {
                let first_arg = _mm_loadu_ps(self.a.as_ptr());
                let sec_arg = _mm_set1_ps(rhs);
                let result = _mm_mul_ps(first_arg, sec_arg);
                _mm_storeu_ps(self.a.as_mut_ptr(), result);
            }
        }

        #[cfg(not(feature = "enable_sse"))]
        fn mul_assign(&mut self, rhs: f32) {
            self.x *= rhs;
            self.y *= rhs;
            self.z *= rhs;
        }
    }

    impl Add for Vec3 {
        type Output = Vec3;

        #[cfg(feature = "enable_sse")]
        fn add(self, rhs: Self) -> Self::Output {
            let mut res = Vec3 { a: [0.; 4] };
            unsafe {
                let first_arg = _mm_loadu_ps(self.a.as_ptr());
                let sec_arg = _mm_loadu_ps(rhs.a.as_ptr());
                let result = _mm_add_ps(first_arg, sec_arg);
                _mm_storeu_ps(res.a.as_mut_ptr(), result);
            }
            res
        }

        #[cfg(not(feature = "enable_sse"))]
        fn add(self, rhs: Self) -> Self::Output {
            Vec3 {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
                z: self.z + rhs.z,
            }
        }
    }

    impl AddAssign for Vec3 {
        #[cfg(feature = "enable_sse")]
        fn add_assign(&mut self, rhs: Self) {
            unsafe {
                let first_arg = _mm_loadu_ps(self.a.as_ptr());
                let sec_arg = _mm_loadu_ps(rhs.a.as_ptr());
                let result = _mm_add_ps(first_arg, sec_arg);
                _mm_storeu_ps(self.a.as_mut_ptr(), result);
            }
        }

        #[cfg(not(feature = "enable_sse"))]
        fn add_assign(&mut self, rhs: Self) {
            self.x += rhs.x;
            self.y += rhs.y;
            self.z += rhs.z;
        }
    }

    impl Sub for Vec3 {
        type Output = Vec3;

        #[cfg(feature = "enable_sse")]
        fn sub(self, rhs: Self) -> Self::Output {
            let mut res = Vec3 { a: [0.; 4] };
            unsafe {
                let first_arg = _mm_loadu_ps(self.a.as_ptr());
                let sec_arg = _mm_loadu_ps(rhs.a.as_ptr());
                let result = _mm_sub_ps(first_arg, sec_arg);
                _mm_storeu_ps(res.a.as_mut_ptr(), result);
            }
            res
        }

        #[cfg(not(feature = "enable_sse"))]
        fn sub(self, rhs: Self) -> Self::Output {
            Vec3 {
                x: self.x - rhs.x,
                y: self.y - rhs.y,
                z: self.z - rhs.z,
            }
        }
    }

    impl SubAssign for Vec3 {
        #[cfg(feature = "enable_sse")]
        fn sub_assign(&mut self, rhs: Self) {
            unsafe {
                let first_arg = _mm_loadu_ps(self.a.as_ptr());
                let sec_arg = _mm_loadu_ps(rhs.a.as_ptr());
                let result = _mm_sub_ps(first_arg, sec_arg);
                _mm_storeu_ps(self.a.as_mut_ptr(), result);
            }
        }

        #[cfg(not(feature = "enable_sse"))]
        fn sub_assign(&mut self, rhs: Self) {
            self.x -= rhs.x;
            self.y -= rhs.y;
            self.z -= rhs.z;
        }
    }

    impl Div<f32> for Vec3 {
        type Output = Vec3;

        #[cfg(feature = "enable_sse")]
        fn div(self, rhs: f32) -> Self::Output {
            let mut res = Vec3 { a: [0.; 4] };
            unsafe {
                let first_arg = _mm_loadu_ps(self.a.as_ptr());
                let sec_arg = _mm_set1_ps(rhs);
                let result = _mm_div_ps(first_arg, sec_arg);
                _mm_storeu_ps(res.a.as_mut_ptr(), result);
            }
            res
        }

        #[cfg(not(feature = "enable_sse"))]
        fn div(self, rhs: f32) -> Self::Output {
            Vec3 {
                x: self.x / rhs,
                y: self.y / rhs,
                z: self.z / rhs,
            }
        }
    }

    impl DivAssign<f32> for Vec3 {
        #[cfg(feature = "enable_sse")]
        fn div_assign(&mut self, rhs: f32) {
            unsafe {
                let first_arg = _mm_loadu_ps(self.a.as_ptr());
                let sec_arg = _mm_set1_ps(rhs);
                let result = _mm_div_ps(first_arg, sec_arg);
                _mm_storeu_ps(self.a.as_mut_ptr(), result);
            }
        }

        #[cfg(not(feature = "enable_sse"))]
        fn div_assign(&mut self, rhs: f32) {
            self.x /= rhs;
            self.y /= rhs;
            self.z /= rhs;
        }
    }

    impl Neg for Vec3 {
        type Output = Vec3;

        #[cfg(feature = "enable_sse")]
        fn neg(self) -> Self::Output {
            let mut res = Vec3 { a: [0.; 4] };
            unsafe {
                let first_arg = _mm_setzero_ps();
                let sec_arg = _mm_loadu_ps(self.a.as_ptr());
                let result = _mm_sub_ps(first_arg, sec_arg);
                _mm_storeu_ps(res.a.as_mut_ptr(), result);
            }
            res
        }

        #[cfg(not(feature = "enable_sse"))]
        fn neg(self) -> Self::Output {
            Vec3 {
                x: -self.x,
                y: -self.y,
                z: -self.z,
            }
        }
    }

    impl Index<usize> for Vec3 {
        type Output = f32;

        #[cfg(feature = "enable_sse")]
        #[track_caller]
        fn index(&self, index: usize) -> &Self::Output {
            &self.a[index]
        }

        #[cfg(not(feature = "enable_sse"))]
        #[track_caller]
        fn index(&self, index: usize) -> &Self::Output {
            match index {
                0 => &self.x,
                1 => &self.y,
                2 => &self.z,
                _ => panic!("Bad index")
            }
        }
    }

    impl fmt::Display for Vec3 {
        #[cfg(feature = "enable_sse")]
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{} {} {}", self.a[0], self.a[1], self.a[2])
        }

        #[cfg(not(feature = "enable_sse"))]
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{} {} {}", self.x, self.y, self.z)
        }
    }

    fn clamp(x: f32, minx: f32, maxx: f32) -> f32 {
        if x < minx {
            return minx;
        }

        if x > maxx {
            return maxx;
        }

        x
    }

    impl fmt::Display for Color {
        #[cfg(feature = "enable_sse")]
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let mut res: [f32; 4] = [0.; 4];
            unsafe {
                let arg = _mm_loadu_ps(self.imp.a.as_ptr());
                let mult = _mm_set1_ps(ImageConfig::default_config().color_scale);
                let arg = _mm_mul_ps(arg, mult);
                let arg = _mm_sqrt_ps(arg);     // Using gamma=2 correction

                let minx = _mm_set1_ps(0.0);
                let maxx = _mm_set1_ps(0.9999);
                let mult = _mm_set1_ps(256.0);

                let arg = _mm_max_ps(arg, minx);
                let arg = _mm_min_ps(arg, maxx);
                let arg = _mm_mul_ps(arg, mult);

                _mm_storeu_ps(res.as_mut_ptr(), arg);
            }

            write!(f,
                   "{} {} {}",
                   res[0] as i32,
                   res[1] as i32,
                   res[2] as i32)
        }

        #[cfg(not(feature = "enable_sse"))]
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let r = (self.imp.x * ImageConfig::default_config().color_scale).sqrt();
            let g = (self.imp.y * ImageConfig::default_config().color_scale).sqrt();
            let b = (self.imp.z * ImageConfig::default_config().color_scale).sqrt();

            let r = clamp(r, 0.0, 0.9999) * 256.0;
            let g = clamp(g, 0.0, 0.9999) * 256.0;
            let b = clamp(b, 0.0, 0.9999) * 256.0;

            write!(f, "{} {} {}", r as i32, g as i32, b as i32)
        }
    }

    impl Vec3 {
        #[cfg(feature = "enable_sse")]
        pub fn new(x: f32, y: f32, z: f32) -> Self {
            Vec3 { a: [x, y, z, 0.0] }
        }

        #[cfg(not(feature = "enable_sse"))]
        pub fn new(x: f32, y: f32, z: f32) -> Self {
            Vec3 { x, y, z }
        }

        pub fn zero() -> Self {
            Vec3::new(0., 0., 0.)
        }

        /// Cartesian vector length
        pub fn length(&self) -> f32 {
            self.length_squared().sqrt()
        }

        /// Squared Cartesian length
        #[cfg(feature = "enable_sse")]
        pub fn length_squared(mut self) -> f32 {
            // self.x * self.x + self.y * self.y + self.z * self.z
            let mut result: f32 = 0.;
            let mut temp: [f32; 4] = [0.0; 4];
            self.a[3] = 0.;
            unsafe {
                let arg = _mm_loadu_ps(self.a.as_ptr());
                let arg2 = arg.clone();
                let arg = _mm_mul_ps(arg, arg2);

                // [x * x] [y * y] [z * z] [0 * 0]

                let arg2 = _mm_shuffle_ps(arg, arg, 0xee); // 0 1 2 3 + 2 3 2 3 = !(0 + 2) (1 + 3)! (2 + 2) (3 + 3)
                let arg = _mm_add_ps(arg, arg2);

                let arg2 = _mm_shuffle_ps(arg, arg, 0b01010101); // (1 + 3) (1 + 3) (1 + 3) (1 + 3)
                let arg = _mm_add_ps(arg, arg2);

                _mm_store_ss(&mut result, arg);
            }
            // assert!((result - (self.a[0] * self.a[0] + self.a[1] * self.a[1] + self.a[2] * self.a[2])).abs() < 0.01);
            result
        }

        #[cfg(not(feature = "enable_sse"))]
        pub fn length_squared(mut self) -> f32 {
            self.x * self.x + self.y * self.y + self.z * self.z
        }

        /// Dot-product of two Vec3
        #[cfg(feature = "enable_sse")]
        pub fn dot(mut self, mut rhs: Vec3) -> f32 {
            // self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
            let mut result: f32 = 0.;
            self.a[3] = 0.;
            rhs.a[3] = 0.;
            unsafe {
                let arg = _mm_loadu_ps(self.a.as_ptr());
                let arg2 = _mm_loadu_ps(rhs.a.as_ptr());
                let arg = _mm_mul_ps(arg, arg2);

                let arg2 = _mm_shuffle_ps(arg, arg, 0xee); // 0 1 2 3 + 2 3 2 3 = !(0 + 2) (1 + 3)! (2 + 2) (3 + 3)
                let arg = _mm_add_ps(arg, arg2);

                let arg2 = _mm_shuffle_ps(arg, arg, 0b01010101); // (1 + 3) (1 + 3) (1 + 3) (1 + 3)
                let arg = _mm_add_ps(arg, arg2);

                _mm_store_ss(&mut result, arg);
            }
            // assert!((result - self.a[0] * rhs.a[0] - self.a[1] * rhs.a[1] - self.a[2] * rhs.a[2]).abs() < 0.01);
            result
        }

        #[cfg(not(feature = "enable_sse"))]
        pub fn dot(self, mut rhs: Vec3) -> f32 {
            self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
        }

        /// Cross-product of two Vec3
        pub fn cross(&self, rhs: &Vec3) -> Vec3 {
            Vec3::new(self[1] * rhs[2] - self[2] * rhs[1],
                      self[2] * rhs[0] - self[0] * rhs[2],
                      self[0] * rhs[1] - self[1] * rhs[0])
        }

        /// Returns normalized vector
        pub fn unit_vector(self) -> Vec3 {
            self / self.length()
        }

        pub fn random() -> Vec3 {
            Vec3::new(rand::random::<f32>(), rand::random::<f32>(), rand::random::<f32>())
        }

        pub fn rand_range(minx: f32, maxx: f32) -> Vec3 {
            let mut rng = rand::thread_rng();
            Vec3::new(rng.gen_range(minx, maxx), rng.gen_range(minx, maxx), rng.gen_range(minx, maxx))
        }

        pub fn rand_unit() -> Vec3 {
            loop {
                let v = Vec3::new(random(), random(), random());
                if (v.length_squared() > 1.) {
                    continue;
                }
                return v
            }
        }

        pub fn rand_unit_sphere() -> Vec3 {
            let mut rng = rand::thread_rng();
            let a = rng.gen_range(0.0, 2.0 * std::f32::consts::PI);
            let z = rng.gen_range(-1.0f32, 1.0f32);
            let r = (1.0 - z * z).sqrt();
            Vec3::new(r * a.cos(), r * a.sin(), z)
        }

        /// Reflect vector according to the normal of the surface
        pub fn reflect(v: &Vec3, normal: &Vec3) -> Vec3 {
            *v - (*normal * (v.dot(*normal) * 2.0))
        }

        /// Refract ray through the dielectric surface
        pub fn refract(v: &Vec3, n: &Vec3, etai_over_etat: f32) -> Vec3 {
            let cos_theta = (-*v).dot(*n);

            let r_out_perp = (*v + *n * cos_theta) * etai_over_etat;
            let r_out_parallel = *n * (-(1.0 - r_out_perp.length_squared()).abs().sqrt()); // -sqrt(fabs(1.0 - r_out_perp.length_squared())) * n

            r_out_perp + r_out_parallel
        }
    }

    /// 3D Point
    pub type Point3 = Vec3;
}

