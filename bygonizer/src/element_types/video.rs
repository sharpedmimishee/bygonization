use crate::element_types;
use image::RgbaImage;

pub struct Video {
    path: String,
    volume: u32,
    size: u32,
    pos: (i32, i32)
}

impl element_types::ElementType for Video {
    fn type_drawing(&self) -> RgbaImage {
        //Must change this behavior
        return RgbaImage::new(1, 1);
    }
}
