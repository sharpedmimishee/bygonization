use crate::element;
use crate::element_types;
use crate::effect;

pub struct layer<E: effect::Effect, T: element_types::ElementType> {
    number: u32,
    elements: Vec<element::Element<E, T>>
}
