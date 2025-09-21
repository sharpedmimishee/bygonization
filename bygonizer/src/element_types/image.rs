use crate::element_types;

pub struct image {
    path: String,
    size: u32,
    pos: (i32, i32),
}

impl element_types::ElementType for image {}

