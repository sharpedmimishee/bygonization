use image::{self, RgbaImage};
use crate::shape_types::ShapeType;
use crate::element_types::{ElementType, VisualElementType};
use crate::effects::VisualEffect;

/// One of [`ElementType`].
/// It has `shape_type`, `color`, `magnification` and `pos` fields.
pub struct Shape<T: ShapeType> {
    pub shape_type: T,
    pub color: (u8, u8, u8, u8),
    pub magnification: (u32, u32),
    pub pos: (i32, i32),
    pub effects: Vec<Box<dyn VisualEffect + 'static>>
}

impl<T: ShapeType> Shape<T> {
    fn get_color(&self) -> (u8, u8, u8, u8) {
        return self.color;
    }
    pub fn new(shape_type: T, color: (u8, u8, u8, u8), magnification: (u32, u32), pos: (i32, i32), effects: Vec<Box<dyn VisualEffect>>) -> Self {
        return Self {
            shape_type,
            color,
            magnification,
            pos,
            effects
        };
    }
}
impl<T: ShapeType + 'static> ElementType for Shape<T> {
    /// For getting image of element.
    fn drawing(&self, width: u32, height: u32, frame: u32) -> RgbaImage {
        let length: (u32, u32) = self.get_magnification();
        let color: (u8, u8, u8, u8) = self.get_color();
        let mut base_image = RgbaImage::new(length.0, length.1);
        base_image = self.shape_type.shape_drawing(&base_image, length, color);
        let pos = self.get_pos();
        let mut result_image = RgbaImage::new(width, height);
        image::imageops::overlay(&mut result_image, &base_image, pos.0 as i64, pos.1 as i64);
        let effects = &self.effects;
        for effect in effects {
            result_image = effect.applying(&mut result_image, frame);
        }
        return result_image;
    }
}
impl<T: ShapeType> VisualElementType for Shape<T> {
    fn get_magnification(&self) -> (u32, u32) {
        return self.magnification;
    }
    fn get_pos(&self) -> (i32, i32) {
        return (self.pos.0, self.pos.1);
    }
}
