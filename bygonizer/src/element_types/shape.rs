use crate::element_types::{self, VisualElement};
use image::RgbaImage;
use crate::shapes;

pub struct Shape<T: shapes::ShapeType> {
    magnification: (u32, u32),
    shapetype: T,
    color: (u8, u8, u8, u8),
    pos: (i32, i32),
}

impl<T: shapes::ShapeType> element_types::ElementType for Shape<T> {
    fn type_drawing(&self) -> RgbaImage {
        let length: (u32, u32) = self.get_magnification();
        let color: (u8, u8, u8, u8) = self.get_color();
        let image = RgbaImage::new(length.0, length.1);
        self.shapetype.shape_drawing(&image, length, color);
        return image;
    }
}
impl<T: shapes::ShapeType> element_types::VisualElement for Shape<T> {
    fn get_pos(&self) -> (i32, i32) {
        return (self.pos.0, self.pos.1);
    }
    fn get_magnification(&self) -> (u32, u32) {
        return self.magnification;
    }
    fn get_size_length(&self) -> (u32, u32) { return (100, 100); }
}
impl<T: shapes::ShapeType> Shape<T> {
    fn get_color(&self) -> (u8, u8, u8, u8) {
        return (self.color.0, self.color.1, self.color.2, self.color.3)
    }
}
