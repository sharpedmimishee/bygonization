use crate::shape_types::ShapeType;
use image::RgbaImage;
use imageproc::{drawing};

pub struct Circle;
impl ShapeType for Circle {
    fn shape_drawing(&self, image: &RgbaImage, length: (u32, u32), color: (u8, u8, u8, u8)) -> RgbaImage {
        let mut image = image.clone();
        let color_allay = Into::<[u8; 4]>::into(color);
        let color = image::Rgba::from(color_allay);
        // You need to know about the standard form of an ellipse because this code took advantage of it.
        // To draw a filled ellipse/circle, use calculating the minimum and maximum values of `x` for `y` each `y` coordinates and draw lines.
        // Calculate the following two `x` coordinates when a `y` coordinate is `y`:
        //note: `cx` and `cy` mean centre of `x`/`y`. `rx` and `ry` mean radius of `x`/`y`.
        //
        // x1 = cx - rx * sqrt(1 - ((y - cy)^2 / ry^2))
        // x2 = cx + rx * sqrt(1 - ((y - cy)^2 / ry^2))
        //
        // If it draws a line between `x1` and `x2`, it can draw width of ellipse at its height of `y`.
        // Repeat this every time which `y` is changed, and draw pixels throughout a image.

        let cx = (length.0 as f32) / 2.0;
        let cy = (length.1 as f32) / 2.0;

        let max = cx + cx * (1.0 - ((((length.1/2) as f32 - cy).powf(2.0)) / cy.powf(2.0))).sqrt();
        // let cy = max / 2.0;
        for y in 0 .. length.1 as u32 {
            // centre and radius are same value, so use cx/cy.
            let x1 = cx - cx * (1.0 - (((y as f32 - cy).powf(2.0)) / cy.powf(2.0))).sqrt();
            let x2 = cx + cx * (1.0 - (((y as f32 - cy).powf(2.0)) / cy.powf(2.0))).sqrt();
            drawing::draw_line_segment_mut(&mut image, (x1, y as f32), (x2, y as f32), color);
        }
        // return drawing::draw_filled_circle(&mut image, , rect::Rect::at(0,0).of_size(length.0, length.1), color);
        return image;
    }
}
