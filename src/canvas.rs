use crate::color::Color;

#[derive(Debug, PartialEq)]
pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pixels: Vec<Vec<Color>>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![vec![Color::black(); width]; height],
        }
    }

    pub fn to_buffer(&self) -> Vec<u32> {
        let mut buffer = Vec::with_capacity(self.width * self.height);
        for row in &self.pixels {
            for &color in row {
                buffer.push(color.to_u32());
            }
        }
        buffer
    }
}
