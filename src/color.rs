#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Color {
    r: f64,
    g: f64,
    b: f64,
}

impl Color {
    fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b }
    }

    pub fn black() -> Self {
        Self::new(0., 0., 0.)
    }

    pub fn to_u32(&self) -> u32 {
        let r = (self.r.clamp(0., 1.) * 255.).round() as u32;
        let g = (self.g.clamp(0., 1.) * 255.).round() as u32;
        let b = (self.b.clamp(0., 1.) * 255.).round() as u32;
        (r << 16) | (g << 8) | b
    }
}
