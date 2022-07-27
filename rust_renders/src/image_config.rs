#[derive(Debug, Clone)]
pub struct ImageConfig {
    pub width: u32,
    pub height: u32,
    pub samples_per_pixel: u32,
    pub depth: u32,
    pub color_scale: f32,
}

impl ImageConfig {
    pub fn aspect_ratio(&self) -> f32 {
        return (self.width as f32) / (self.height as f32);
    }

    pub fn default_config() -> ImageConfig {
        static WIDTH: u32 = 1920;
        static HEIGHT: u32 = 1080;//(WIDTH as f32 / ASPECT_RATIO) as u32;
        static SAMPLES_PER_PIXEL: u32 = 100;
        static DEPTH: u32 = 5;
        static COLOR_SCALE: f32 = 1.0 / SAMPLES_PER_PIXEL as f32;

        ImageConfig {
            width: WIDTH,
            height: HEIGHT,
            samples_per_pixel: SAMPLES_PER_PIXEL,
            depth: DEPTH,
            color_scale: COLOR_SCALE,
        }
    }
}
