use minifb::{Key, Window, WindowOptions};

#[derive(Debug, PartialEq)]
struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<Vec<Color>>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Color {
    r: f64,
    g: f64,
    b: f64,
}

impl Color {
    fn red() -> Self {
        Self {
            r: 1.,
            g: 0.,
            b: 0.,
        }
    }

    fn to_u32(&self) -> u32 {
        let r = (self.r.clamp(0., 1.) * 255.).round() as u32;
        let g = (self.g.clamp(0., 1.) * 255.).round() as u32;
        let b = (self.b.clamp(0., 1.) * 255.).round() as u32;
        (r << 16) | (g << 8) | b
    }
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![vec![Color::red(); width]; height],
        }
    }

    fn to_u32_buffer(&self) -> Vec<u32> {
        let mut buffer = Vec::with_capacity(self.width * self.height);
        for row in &self.pixels {
            for &color in row {
                buffer.push(color.to_u32());
            }
        }
        buffer
    }
}

fn main() {
    let canvas = Canvas::new(600, 400);
    let buffer = canvas.to_u32_buffer();

    let mut window =
        Window::new("rt", canvas.width, canvas.height, WindowOptions::default()).unwrap();
    window.set_target_fps(30);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window
            .update_with_buffer(&buffer, canvas.width, canvas.height)
            .unwrap();
    }
}
