use crate::{clamp, Color};

#[derive(Debug, PartialEq)]
pub struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<Vec<Color>>, // TODO: just Vec<Color>?
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![vec![Color::black(); width]; height],
        }
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, c: Color) {
        assert!(x < self.width);
        assert!(y < self.height);
        self.pixels[y][x] = c;
    }

    pub fn to_ppm(&self) -> String {
        let mut ppm = format!("P3\n{} {}\n255\n", self.width, self.height);
        for row in &self.pixels {
            for pix in row {
                ppm.push_str(&format!(
                    "{} {} {}",
                    clamp(pix.r * 255.0, 0.0, 255.0).round() as usize,
                    clamp(pix.g * 255.0, 0.0, 255.0).round() as usize,
                    clamp(pix.b * 255.0, 0.0, 255.0).round() as usize,
                ));
                ppm.push('\n');
            }
        }
        ppm
    }
}

#[cfg(test)]
mod tests {
    use crate::Color;

    use super::Canvas;

    #[test]
    fn test_new() {
        let canvas = Canvas::new(10, 20);
        assert_eq!(canvas.width, 10);
        assert_eq!(canvas.height, 20);
        assert_eq!(canvas.pixels.len(), 20);
        assert_eq!(canvas.pixels[0].len(), 10);
        let black = Color::black();
        assert!(canvas
            .pixels
            .iter()
            .all(|row| row.iter().all(|c| *c == black)));
    }

    #[test]
    fn test_write_pixel_valid() {
        let mut canvas = Canvas::new(3, 2);
        canvas.write_pixel(0, 0, Color::red());
        canvas.write_pixel(1, 0, Color::green());
        canvas.write_pixel(2, 1, Color::blue());
        assert_eq!(
            canvas.pixels,
            vec![
                vec![Color::red(), Color::green(), Color::black()],
                vec![Color::black(), Color::black(), Color::blue()]
            ]
        );
    }

    #[test]
    #[should_panic]
    fn test_write_pixel_invalid() {
        let mut canvas = Canvas::new(3, 2);
        canvas.write_pixel(3, 0, Color::red());
    }

    #[test]
    fn test_to_ppm() {
        let mut canvas = Canvas::new(3, 2);
        canvas.write_pixel(0, 0, Color::red());
        canvas.write_pixel(1, 0, Color::green());
        canvas.write_pixel(2, 0, Color::new(0.333, 0.667, 1.0));
        canvas.write_pixel(2, 1, Color::blue());
        assert_eq!(
            canvas.to_ppm(),
            String::from("P3\n3 2\n255\n255 0 0\n0 255 0\n85 170 255\n0 0 0\n0 0 0\n0 0 255\n")
        );
    }
}
