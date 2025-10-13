use crate::element_types;
use image::RgbaImage;

pub struct Audio {
    path: String,
    volume: u32,
    pos: (i32, i32)
}

impl element_types::ElementType for Audio {
    fn type_drawing(&self) -> RgbaImage {
        //Must change this behavior
        return RgbaImage::new(1, 1);
    }
}
