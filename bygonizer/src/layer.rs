use crate::element::Element;
use crate::element_types::ElementType;

pub struct Layer {
    pub elements: Vec<Element<dyn ElementType>>,
    // name: String
}

impl Layer {
    // fn get_elements(&self) -> Vec<Element<dyn ElementType>> {
    //     let elements = &self.elements;
    //     return *elements.clone();
    // }
    pub fn new(elements: Vec<Element<dyn ElementType>>) -> Self {
        return Self {
            elements,
        };
    }
}
