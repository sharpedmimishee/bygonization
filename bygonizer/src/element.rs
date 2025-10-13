use crate::element_types;
use crate::effect;
use image::{self, RgbaImage};

pub struct Element<E: effect::Effect, T: element_types::ElementType> {
    pub element_type: T,
    pub effects: Vec<E>,
    pub layer: u32,
    pub time_begin: u32,
    pub time_end: u32
}
//Need to match element_types() using VisualElement trait. (Exempli gratia, type of Audio doesn't need to convert to rgbaImage.)
impl<E: effect::Effect, T: element_types::ElementType + element_types::VisualElement> Element<E, T> {
    pub fn convert_to_rgbaimage(&self) -> RgbaImage {
        //Get self scale.
        let element_type = &self.element_type;
        let image = element_type.type_drawing();
        //Apply effects of a list of a element.
        //Return Value
        return image;
    }
}
