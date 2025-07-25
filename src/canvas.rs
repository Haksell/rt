use {
    crate::color::Color,
    core::ops::{Index, IndexMut},
};

#[derive(Debug, PartialEq)]
pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pixels: Vec<Color>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![Color::black(); width * height],
        }
    }

    pub fn to_buffer(&self) -> Vec<u32> {
        let mut buffer = Vec::with_capacity(self.width * self.height);
        buffer.extend(self.pixels.iter().map(Color::to_u32));
        buffer
    }

    pub fn to_ppm(&self) -> String {
        let mut ppm = format!("P3\n{} {}\n255\n", self.width, self.height);
        for pixel in &self.pixels {
            ppm += &format!(
                "{} {} {}\n",
                (pixel.r.clamp(0., 1.) * 255.).round() as u8,
                (pixel.g.clamp(0., 1.) * 255.).round() as u8,
                (pixel.b.clamp(0., 1.) * 255.).round() as u8,
            );
        }
        ppm
    }
}

impl Index<(usize, usize)> for Canvas {
    type Output = Color;

    fn index(&self, (y, x): (usize, usize)) -> &Self::Output {
        debug_assert!(x < self.width);
        debug_assert!(y < self.height);
        &self.pixels[y * self.width + x]
    }
}

impl IndexMut<(usize, usize)> for Canvas {
    fn index_mut(&mut self, (y, x): (usize, usize)) -> &mut Self::Output {
        debug_assert!(x < self.width);
        debug_assert!(y < self.height);
        &mut self.pixels[y * self.width + x]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let canvas = Canvas::new(10, 20);
        assert_eq!(canvas.width, 10);
        assert_eq!(canvas.height, 20);
        assert_eq!(canvas.pixels.len(), 200);
        assert!(canvas.pixels.iter().all(|c| *c == Color::black()));
    }

    #[test]
    fn test_set_pixel_valid() {
        let mut canvas = Canvas::new(3, 2);
        canvas[(0, 0)] = Color::red();
        canvas[(0, 1)] = Color::green();
        canvas[(1, 2)] = Color::blue();
        assert_eq!(canvas[(0, 0)], Color::red());
        assert_eq!(canvas[(0, 1)], Color::green());
        assert_eq!(canvas[(0, 2)], Color::black());
        assert_eq!(canvas[(1, 0)], Color::black());
        assert_eq!(canvas[(1, 1)], Color::black());
        assert_eq!(canvas[(1, 2)], Color::blue());
        assert_eq!(
            canvas.pixels,
            vec![
                Color::red(),
                Color::green(),
                Color::black(),
                Color::black(),
                Color::black(),
                Color::blue()
            ]
        );
    }

    #[test]
    fn test_set_pixel_invalid() {
        if cfg!(debug_assertions) {
            let result = std::panic::catch_unwind(|| {
                let mut canvas = Canvas::new(3, 2);
                canvas[(0, 3)] = Color::red();
            });
            assert!(result.is_err(), "Expected panic in debug mode");
        }
    }

    #[test]
    fn test_to_ppm() {
        let mut canvas = Canvas::new(3, 2);
        canvas[(0, 0)] = Color::red();
        canvas[(0, 1)] = Color::green();
        canvas[(0, 2)] = Color::new(0.333, 0.667, 1.);
        canvas[(1, 2)] = Color::blue();
        assert_eq!(
            canvas.to_ppm(),
            String::from("P3\n3 2\n255\n255 0 0\n0 255 0\n85 170 255\n0 0 0\n0 0 0\n0 0 255\n")
        );
    }
}
