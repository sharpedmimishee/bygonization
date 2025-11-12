use image::RgbaImage;
pub mod shape;
pub mod basictext;

pub trait ElementType {
    fn drawing(&self, width: u32, height: u32, frame: u32) -> RgbaImage;
}

pub trait VisualElementType {
    fn get_magnification(&self) -> (u32, u32);
    fn get_pos(&self) -> (i32, i32);
}
pub trait AuditoryElementType {}
