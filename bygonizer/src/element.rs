use crate::element_types::ElementType;

// #[derive(Clone)]
pub struct Element<T: ElementType + ?Sized> {
    pub element_type: Box<T>,
    pub begin_time: u32,
    pub end_time: u32,
    // Effects should be splited as Visual and Auditory.
    // effects: Vec<E>,
}

impl<T: ElementType + ?Sized> Element<T> {
    pub fn get_time(&self) -> (u32, u32) {
        return (self.begin_time, self.end_time);
    }
    // ///`convert_to_RgbaImage()` is for converting a element of a visual element type to RgbaImage format.
    // ///Trait border is set as ElementType only because if the trait border is set as VisualElement,
    // ///`specific_encode()` cannot use this method because `elements` field of `Layer` has any type
    // ///element.
    // pub fn convert_to_RgbaImage(&self) -> RgbaImage{
    //     let element_type = &self.element_type;
    //     let image = element_type.drawing();
    //     return image;
    // }
}
