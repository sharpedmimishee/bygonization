use crate::element_types;
use crate::effect;

pub struct Element<E: effect::Effect, T: element_types::ElementType> {
    element_type: T,
    effects: Vec<E>,
    layer: u32,
    time_begin: u32,
    time_end: u32
}
