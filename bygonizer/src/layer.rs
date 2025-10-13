use crate::element;
use crate::element_types;
use crate::effect;

pub struct Layer<E: effect::Effect, T: element_types::ElementType> {
    pub number: u32,
    pub elements: Vec<element::Element<E, T>>
}

impl<E: effect::Effect, T: element_types::ElementType> Layer<E, T> {
    pub fn new(number: u32, elements: Vec<element::Element<E, T>>) -> Layer<E, T> {
        return Layer {number, elements}
    }
}
