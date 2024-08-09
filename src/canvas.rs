use crate::Color;

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
}
