//! # Bygonizer
//! Bygonizer is a backend system of Bygonization Video Editor, which provides things like API for
//! extensibility. It currently uses some crates which is not developed by developers of Bygonization.

pub mod element_types;
pub mod shape_types;
pub mod element;
pub mod methods;
pub mod layer;
pub mod effects;
pub mod util;

/// For encoding
/// It will be called when Bygonization want Bygonizer to encode for frame preview or video encoding.
/// called -> drawing -> applying effects -> merging(overlaying) images.
fn encode(width: u32, height: u32, fps: u32, method: &dyn methods::Method, layers: Vec<layer::Layer>) {
}
#[cfg(test)]
mod tests {
    use super::*;
    use layer::Layer;
    use element::Element;
    use element_types::ElementType;
    use element_types::shape::Shape;
    use element_types::basictext::BasicText;
    use methods::FrameMethod;
    use std::path::PathBuf;
    use effects::{ movement::Movement, alpha::Alpha};
    use util::Point2D;
    #[test]
    fn run() {
        let width = 1920;
        let height = 1080;
        let mut layers: Vec<Layer> = Vec::new();
        let mut elements_1: Vec<Element<dyn ElementType>> = Vec::new();
        let element_1: Element<dyn ElementType> = Element {
            element_type: Box::<dyn ElementType>::from(Box::new(Shape::new(
                shape_types::square::Square,
                (255, 255, 255, 255),
                (1920, 1080),
                (50, 50),
                vec![]
            ))),
            begin_time: 0,
            end_time: 20,
        };
        elements_1.push(element_1);
        let mut elements_2: Vec<Element<dyn ElementType>> = Vec::new();
        let xpoints: Vec<Point2D> = vec![
            Point2D::new(0.0, 0.0),
            Point2D::new(20., 90.),
            Point2D::new(30., 100.),
            Point2D::new(40., 1000.),
        ];
        let ypoints: Vec<Point2D> = vec![
            Point2D::new(0.0, 0.0),
            Point2D::new(0., 0.),
            Point2D::new(0., 0.),
            Point2D::new(0., 0.),
        ];

        let effect_1 = Movement::new(xpoints ,ypoints);
        let value: Vec<Point2D> = vec![
            Point2D::new(0., 200.),
            Point2D::new(20., 200.)
        ];
        let effect_2 = Alpha::new(value);

        let element_1: Element<dyn ElementType> = Element {
            element_type: Box::<dyn ElementType>::from(Box::new(Shape::new(
                shape_types::square::Square,
                (200, 100, 200, 255),
                (300, 200),
                (100, 150),
                vec![Box::new(effect_1), Box::new(effect_2)]
            ))),
            begin_time: 0,
            end_time: 20
        };
        elements_2.push(element_1);
        let mut elements_3: Vec<Element<dyn ElementType>> = Vec::new();
        let element_1: Element<dyn ElementType> = Element {
            element_type: Box::<dyn ElementType>::from(Box::new(BasicText::new(
                PathBuf::from("c:/users/nishi/documents/devs/sadameiri-arcchival/a-lightengazer/arcchival-lightengazer.ttf"),
                String::from("BasicText applied Bold and Italic"),
                100_f32,
                (0, 0, 0, 255),
                0.0,
                0.0,
                false,
                true,
                true,
                (200, 150)
            ))),
            begin_time: 0,
            end_time: 20
        };
        elements_3.push(element_1);

        let layer_1: Layer = Layer::new(elements_1);
        let layer_2: Layer = Layer::new(elements_2);
        let layer_3: Layer = Layer::new(elements_3);
        layers.push(layer_1);
        layers.push(layer_2);
        layers.push(layer_3);
        for i in 0..20 {
            methods::to_png::ToPng{extension: String::from(".png"), method_name: String::from("ToPng")}
                .frame_encode(width, height, &layers, i);
        }
    }
    #[test]
    fn effect() {
        println!("effect!");
        let xpoints: Vec<Point2D> = vec![
            Point2D::new(0.0, 0.0),
            Point2D::new(2., 90.),
            Point2D::new(8., 100.),
            Point2D::new(4., 100.0),
        ];
        let ypoints: Vec<Point2D> = vec![
            Point2D::new(0.0, 0.0),
            Point2D::new(2., 90.),
            Point2D::new(3., 10.),
            Point2D::new(10., 100.0),
        ];

        let _effect = Movement::new(xpoints ,ypoints);
    }
}
