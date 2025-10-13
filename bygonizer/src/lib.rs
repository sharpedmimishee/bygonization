use image::{self, RgbaImage, imageops};

mod element_types;
mod effect;
mod element;
mod layer;
mod shapes;

///Just overlaying iamges
fn overlay(base_image: &mut RgbaImage, overlay_image: &RgbaImage, x: i64, y: i64) {
    imageops::overlay(base_image, overlay_image, x, y);
}

fn png_encode<E: effect::Effect, T: element_types::ElementType + element_types::VisualElement>() {
    let width: u32 = 1920;
    let height: u32 = 1080;
    
    let layers: Vec<layer::Layer<E, T>> = Vec::<layer::Layer<E, T>>::new();

    let mut base: RgbaImage = RgbaImage::new(width, height);
    
    for i in layers {
        let elements: Vec<element::Element<_, _>> = i.elements;
        for element in elements {
            //Convert the element to RgbaImage
            let converted: RgbaImage = element.convert_to_rgbaimage();
            let element_type: T = element.element_type;
            let pos = element_type.get_pos();

            //Apply effects

            //Overlay the converted element
            overlay(&mut base, &converted, pos.0.into(), pos.1.into());
        }
    }
}
