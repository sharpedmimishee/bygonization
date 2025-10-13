use crate::element_types;
use image::RgbaImage;

pub struct Text {
    text: String,
    font: String,
    size: u32,
    italic: bool,
    bold: bool,
    pos: (i32, i32)
}

impl element_types::ElementType for Text {
    fn type_drawing(&self) -> RgbaImage {
        //Must change this behavior
        return RgbaImage::new(1, 1);
    }
}
