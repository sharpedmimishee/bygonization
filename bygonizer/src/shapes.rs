use image::RgbaImage;

pub mod square;

pub trait ShapeType {
    fn shape_drawing(&self, image: &RgbaImage, length: (u32, u32), color: (u8, u8, u8, u8));
}
