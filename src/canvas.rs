use crate::Color;

#[derive(Debug, PartialEq)]
pub struct Canvas {
    pub width: usize,
    pub height: usize,
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

    pub fn set_pixel(&mut self, x: usize, y: usize, c: Color) {
        assert!(x < self.width);
        assert!(y < self.height);
        self.pixels[y][x] = c;
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Color {
        assert!(x < self.width);
        assert!(y < self.height);
        self.pixels[y][x]
    }

    pub fn to_ppm(&self) -> String {
        let mut ppm = format!("P3\n{} {}\n255\n", self.width, self.height);
        for row in &self.pixels {
            for pix in row {
                ppm.push_str(&format!(
                    "{} {} {}",
                    (pix.r.clamp(0., 1.) * 255.).round() as usize,
                    (pix.g.clamp(0., 1.) * 255.).round() as usize,
                    (pix.b.clamp(0., 1.) * 255.).round() as usize,
                ));
                ppm.push('\n');
            }
        }
        ppm
    }
}

#[cfg(test)]
mod tests {
    use super::Canvas;
    use crate::Color;

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
    fn test_set_pixel_valid() {
        let mut canvas = Canvas::new(3, 2);
        canvas.set_pixel(0, 0, Color::red());
        canvas.set_pixel(1, 0, Color::green());
        canvas.set_pixel(2, 1, Color::blue());
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
    fn test_set_pixel_invalid() {
        let mut canvas = Canvas::new(3, 2);
        canvas.set_pixel(3, 0, Color::red());
    }

    #[test]
    fn test_to_ppm() {
        let mut canvas = Canvas::new(3, 2);
        canvas.set_pixel(0, 0, Color::red());
        canvas.set_pixel(1, 0, Color::green());
        canvas.set_pixel(2, 0, Color::new(0.333, 0.667, 1.));
        canvas.set_pixel(2, 1, Color::blue());
        assert_eq!(
            canvas.to_ppm(),
            String::from("P3\n3 2\n255\n255 0 0\n0 255 0\n85 170 255\n0 0 0\n0 0 0\n0 0 255\n")
        );
    }
}
