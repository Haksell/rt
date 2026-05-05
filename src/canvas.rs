use std::ops::{Index, IndexMut};

use crate::color::Color;

#[derive(Debug)]
pub struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<Color>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        debug_assert!(width > 0);
        debug_assert!(height > 0);
        Self {
            width,
            height,
            pixels: vec![Color::black(); width * height],
        }
    }

    pub fn to_buffer(&self) -> Vec<u32> {
        let mut buffer = Vec::with_capacity(self.width * self.height);
        buffer.extend(self.pixels.iter().map(|color| color.to_u32()));
        buffer
    }

    pub const fn width(&self) -> usize {
        self.width
    }

    pub const fn height(&self) -> usize {
        self.height
    }
}

impl Index<(usize, usize)> for Canvas {
    type Output = Color;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        debug_assert!(x < self.width);
        debug_assert!(y < self.height);
        &self.pixels[y * self.width + x]
    }
}

impl IndexMut<(usize, usize)> for Canvas {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        debug_assert!(x < self.width);
        debug_assert!(y < self.height);
        &mut self.pixels[y * self.width + x]
    }
}
