use image::RgbaImage;
use imageproc::geometric_transformations::translate;
use crate::util::{Point2D, bezier_y_from_x};
use crate::effects::VisualEffect;

pub struct Movement {
    xpoints: Vec<Point2D>,
    ypoints: Vec<Point2D>,
}
impl Movement {
    pub fn new(xpoints: Vec<Point2D>, ypoints: Vec<Point2D>) -> Self {
        return Self {
            xpoints: xpoints,
            ypoints: ypoints
        };
    }
}
impl VisualEffect for Movement {
    fn applying(&self, image: &mut RgbaImage,  frame: u32) -> RgbaImage {
        let x_pos = bezier_y_from_x(&self.xpoints, frame as f64);
        let y_pos = bezier_y_from_x(&self.ypoints, frame as f64);
        // panic!("X: {}, Y: {}", x_pos, y_pos);
        return translate(image, (x_pos as i32, y_pos as i32));
    }
}
