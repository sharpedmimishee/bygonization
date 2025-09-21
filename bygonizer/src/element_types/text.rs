use crate::element_types;

pub struct text {
    text: String,
    font: String,
    size: u32,
    italic: bool,
    bold: bool,
    pos: (i32, i32)
}

impl element_types::ElementType for text {}
