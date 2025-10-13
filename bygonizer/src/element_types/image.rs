use crate::element_types;
use image::RgbaImage;

pub struct Image {
    path: String,
    size: u32,
    pos: (i32, i32),
}

impl element_types::ElementType for Image {
    fn type_drawing(&self) -> RgbaImage {
        //Must change this behavior
        return RgbaImage::new(1, 1);
    }
}

