use crate::methods::{self, Method, FrameMethod};
use crate::layer::Layer;
use crate::element::Element;
use image::{self, imageops, RgbaImage};

pub struct ToPng {
    pub extension: String,
    pub method_name: String
}
impl FrameMethod for ToPng {
    fn frame_encode(&self, width: u32, height: u32, layers: &Vec<Layer>, frame: u32) {
        let mut base_image = image::RgbaImage::new(width, height);
        for layer in layers {
            let target_element: &Element<_> = match methods::get_element_from_time(&layer.elements, frame) {
                Ok(x) => x,
                _ => continue
            };
            let target_type = &target_element.element_type;
            let target_image: RgbaImage = target_type.drawing(width, height, frame);
            //TODO: we ought to make a system which apply effects to target_image.
            imageops::overlay(&mut base_image, &target_image, 0, 0);
        }
        if !std::fs::exists("output/").unwrap() { std::fs::create_dir("output/").unwrap(); }
        base_image.save_with_format(format!("output/{}.png", frame), image::ImageFormat::Png).unwrap();
    }
}
impl Method for ToPng {
    fn get_extension(&self) -> String {
        return self.extension.clone();
    }
    fn get_method_name(&self) -> String {
        return self.method_name.clone();
    }

    //TODO: MAKE SPECIFIC_ENCODE
    fn specific_encode(&self, width: u32, height: u32, layers: Vec<Layer>) {
        let base_image = image::RgbaImage::new(width, height);
    }
}
