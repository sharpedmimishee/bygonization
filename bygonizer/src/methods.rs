use crate::layer::Layer;
use crate::element::Element;
use crate::element_types::ElementType;
pub mod to_png;

/// For deciding target extension type of encoding.
pub trait Method {
    fn get_extension(&self) -> String;
    fn get_method_name(&self) -> String;
    /// Do method-specific processing.
    fn specific_encode(&self, width: u32, height: u32, layers: Vec<Layer>);
}
/// A trait for methods which can be encoded only a frame.
pub trait FrameMethod {
    /// For encoding only a frame.
    /// You ought to use [`Method::specific_encode()`] if you want to encode all of a Bygonization project.
    fn frame_encode(&self, width: u32, height: u32, layers: &Vec<Layer>, frame: u32);
}

/// Return a element from specified target time.
pub fn get_element_from_time(elements: &Vec<Element<dyn ElementType>>, target_time: u32) -> Result<&Element<dyn ElementType>, &'static str> {
    for element in elements {
        let time: (u32, u32) = element.get_time();
        if time.0 <= target_time && time.1 >= target_time {
            return Ok(element);
        }
    }
    return Err("Could not find a element at the target time!");
}
