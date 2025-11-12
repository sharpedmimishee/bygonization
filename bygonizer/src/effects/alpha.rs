use crate::util::{Point2D, bezier_y_from_x};
use crate::effects::VisualEffect;
use image::RgbaImage;

pub struct Alpha {
    value: Vec<Point2D>
}
impl Alpha {
    pub fn new(value: Vec<Point2D>) -> Self {
        return Self { value };
    }
}

impl VisualEffect for Alpha {
    fn applying(&self, image: &mut RgbaImage, frame: u32) -> RgbaImage {
        //TODO: WE NEED TO FIX THIS. IT WON'T SAVE A IMAGE WITH SPECIFIC ALPHA VALUE.
        let alpha = bezier_y_from_x(&self.value, frame as f64);
        let _ = image.pixels_mut().map(|x| x[3] = alpha as u8);
        // image.save("test.png");
        return image.clone();
    }
}
