use image::RgbaImage;
pub mod movement;
pub mod blur;
pub mod pan;
pub mod scale;
pub mod alpha;

/// For effects which affect [`VisualElementType`][crate::element_types::VisualElementType], such as
/// [`Movement`][crate::effects::movement::Movement].
pub trait VisualEffect {
    fn applying(&self, image: &mut RgbaImage, frame: u32) -> RgbaImage;

}
/// For effects which affect [`AuditoryElementType`][crate::element_types::AuditoryElementType], such as [`Pan`][crate::effects::pan::Pan].
pub trait AuditoryEffect {}
