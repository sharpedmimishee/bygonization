use crate::element_types;

pub struct video {
    path: String,
    volume: u32,
    size: u32
}

impl element_types::ElementType for video {}
