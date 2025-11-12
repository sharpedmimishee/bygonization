use std::path::PathBuf;
use image::{self, RgbaImage};
use crate::element_types::{ElementType, VisualElementType};
use imageproc::{drawing, geometric_transformations::{warp, Interpolation, Projection}};
use ab_glyph::FontRef;
use std::fs::File;
use std::io::Read;

pub struct BasicText {
    font: PathBuf,
    content: String,
    size: f32,
    color: (u8, u8, u8, u8),
    
    //TODO: MAKE BasicText USE THESE FIELDS.
    letter_spacing: f32,
    line_spacing: f32,
    vertical: bool,

    bold: bool,
    italic: bool,
    pos: (i32, i32),
}

impl BasicText {
    pub fn new(font: PathBuf, content: String,size: f32, color: (u8, u8, u8, u8), letter_spacing: f32, line_spacing: f32, vertical: bool, bold: bool, italic: bool,pos: (i32, i32)) -> Self {
        return Self {
            font,
            content,
            size,
            color,
            letter_spacing,
            line_spacing,
            vertical,
            bold,
            italic,
            pos
        }
    }
}

impl ElementType for BasicText {
    fn drawing(&self, width: u32, height: u32, frame: u32) -> RgbaImage {
        let base_image = RgbaImage::new(width, height);
        let color_allay = Into::<[u8; 4]>::into(self.color);
        let color = image::Rgba::from(color_allay);
        let mut font = File::open(&self.font).unwrap();
        let mut font_vec: Vec<u8> = Vec::new();
        font.read_to_end(&mut font_vec).unwrap();
        let font = FontRef::try_from_slice(&font_vec).unwrap();
        let mut drawn_image = drawing::draw_text(&base_image, color, self.pos.0, self.pos.1, self.size, &font.clone(), &self.content);
        // drawn_image.save("drawn_text_image.png").unwrap();
        if self.bold {
            drawn_image = drawing::draw_text(&drawn_image, color, self.pos.0+1, self.pos.1-1, self.size, &font.clone(), &self.content);
            drawn_image = drawing::draw_text(&drawn_image, color, self.pos.0+2, self.pos.1-2, self.size, &font.clone(), &self.content);
        }
        if self.italic {
            let matrix = Projection::from_matrix([
                1.0, -0.3, 0.0,
                0.0, 1.0, 0.0,
                0.0, 0.0, 1.0
            ]);
            drawn_image = warp(
                &drawn_image,
                &matrix.unwrap(),
                Interpolation::Bilinear,
                image::Rgba::from([0, 0, 0, 0]),
            );
        }
        // drawn_image.save("result_text.png").unwrap();
        return drawn_image;
    }
}

impl VisualElementType for BasicText {
    fn get_magnification(&self) -> (u32, u32) {
        todo!();
    }
    fn get_pos(&self) -> (i32, i32) {
        return (self.pos.0, self.pos.1);
    }
}
