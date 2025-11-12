use crate::shape_types::ShapeType;
use image::RgbaImage;
use imageproc::{drawing, rect};

pub struct Square;
impl ShapeType for Square {
    fn shape_drawing(&self, image: &RgbaImage, length: (u32, u32), color: (u8, u8, u8, u8)) -> RgbaImage {
        let mut image = image.clone();
        let color_allay = Into::<[u8; 4]>::into(color);
        let color = image::Rgba::from(color_allay);

        return drawing::draw_filled_rect(&mut image, rect::Rect::at(0,0).of_size(length.0, length.1), color);
        // return image;
    }
}
