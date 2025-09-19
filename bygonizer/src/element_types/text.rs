use crate::element_types;

pub struct text {
    text: String,
    font: String,
    size: u32,
}

impl element_types::ElementType for text {}
