use crate::RgbaImage;

pub mod text;
pub mod video;
pub mod audio;
pub mod image;
pub mod shape;

pub trait ElementType {
    fn type_drawing(&self) -> RgbaImage;
}
/// `VisualElement` is for visual element such as [`Text`] and [`Video`],
pub trait VisualElement {
    /// Get positions as tuple.
    fn get_pos(&self) -> (i32, i32);
    /// Get aspect of width and height as tuple
    fn get_magnification(&self) -> (u32, u32);
    /// Get size length of width and height as tuple.
    fn get_size_length(&self) -> (u32, u32);
}

