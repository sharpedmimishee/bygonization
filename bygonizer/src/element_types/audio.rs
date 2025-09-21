use crate::element_types;

pub struct audio {
    path: String,
    volume: u32,
    pos: (i32, i32)
}

impl element_types::ElementType for audio {}
