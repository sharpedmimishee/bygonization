use crate::element_types;

pub struct audio {
    path: String,
    volume: u32,
}

impl element_types::ElementType for audio {}
