use image::RgbaImage;
pub mod square;
pub mod circle;
pub mod triangle;

pub trait ShapeType {
    /// Draw [`Shape`][crate::element_types::shape::Shape] element as its [`ShapeType`].
    /// Implemented to all of [`ShapeType`]s.
    fn shape_drawing(&self, image: &RgbaImage, length: (u32, u32), color: (u8, u8, u8, u8)) -> RgbaImage;
}
