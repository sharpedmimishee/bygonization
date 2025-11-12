use crate::effects::VisualEffect;
use image::RgbaImage;

pub struct Blur {
    level: f32
}
impl VisualEffect for Blur {
    fn applying(&self, image: &mut RgbaImage, frame: u32) -> RgbaImage {
        return image::imageops::blur(image, self.level);
    }
}
