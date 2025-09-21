use crate::element_types;

pub struct video {
    path: String,
    volume: u32,
    size: u32,
    pos: (i32, i32)
}

impl element_types::ElementType for video {}
