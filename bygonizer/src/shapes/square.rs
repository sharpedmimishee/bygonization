use crate::shapes::ShapeType;
use image::{self, RgbaImage};
use imageproc::{drawing, rect};

pub struct Square;
impl ShapeType for Square {
    fn shape_drawing(&self, image: &RgbaImage, length: (u32, u32), color: (u8, u8, u8, u8)) {
        let color_allay = Into::<[u8; 4]>::into(color);
        let color = image::Rgba::from(color_allay);
        drawing::draw_filled_rect(image, rect::Rect::at(0,0).of_size(length.0, length.1), color);
    }
}
