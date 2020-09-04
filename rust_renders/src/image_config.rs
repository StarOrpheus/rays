pub mod image_config {
    #[derive(Debug, Clone)]
    pub struct ImageConfig {
        pub aspect_ratio: f32,
        pub width: u32,
        pub height: u32,
        pub samples_per_pixel: u32,
        pub depth: u32,
        pub color_scale: f32
    }

    impl ImageConfig {
        pub fn default_config() -> &'static ImageConfig {
            static aspect_ratio: f32 = 16.0 / 9.0;
            static samples_per_pixel: u32 = 100;
            static width: u32 = 1920;
            static height: u32 = (width as f32 / aspect_ratio) as u32;
            static depth: u32 = 30;
            static color_scale: f32 = 1.0 / samples_per_pixel as f32;

            static IMPLEMENTATION: ImageConfig
                = ImageConfig{aspect_ratio, width, height, samples_per_pixel, depth, color_scale};

            &IMPLEMENTATION
        }
    }
}