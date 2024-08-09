// TODO: reuse rt::Tuple or SIMD?

use crate::Float;

struct Color {
    r: Float,
    g: Float,
    b: Float,
}

impl Color {
    pub fn new(r: Float, g: Float, b: Float) -> Self {
        Self { r, g, b }
    }
}

#[cfg(test)]
mod tests {
    use super::Color;

    #[test]
    fn test_new() {
        let c = Color::new(-0.5, 0.4, 1.7);
        assert_eq!(c.r, -0.5);
        assert_eq!(c.g, 0.4);
        assert_eq!(c.b, 1.7);
    }
}
