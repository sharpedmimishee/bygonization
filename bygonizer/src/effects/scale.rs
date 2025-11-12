use crate::util::{Point2D, bezier_y_from_x};
use crate::effects::VisualEffect;
use image::RgbaImage;

pub struct Scale {
    x_size: Vec<Point2D>,
    y_size: Vec<Point2D>,
    filter_type: image::imageops::FilterType
}

impl VisualEffect for Scale {
    fn applying(&self, image: &mut RgbaImage, frame: u32) -> RgbaImage {
        let x = bezier_y_from_x(&self.x_size, frame as f64);
        let y = bezier_y_from_x(&self.y_size, frame as f64);
        return image::imageops::resize(image, x as u32, y as u32, self.filter_type);
    }
}
