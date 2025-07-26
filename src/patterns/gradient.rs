use {
    super::Pattern,
    crate::{
        color::Color,
        math::{Matrix, Tuple, transform},
    },
};

#[derive(Clone, Debug, PartialEq)]
pub struct Gradient {
    a: Color,
    b: Color,
    inverse_transform: Matrix,
}

impl Gradient {
    pub fn default() -> Self {
        Self {
            a: Color::white(),
            b: Color::black(),
            inverse_transform: Matrix::identity(),
        }
    }

    pub fn new(a: Color, b: Color, transform: Matrix) -> Self {
        Self {
            a,
            b,
            inverse_transform: transform.inverse(),
        }
    }
}

impl Pattern for Gradient {
    fn color_at(&self, point: &Tuple) -> Color {
        self.a + (self.b - self.a) * point.x.rem_euclid(1.0)
    }

    fn get_inverse_transform(&self) -> &Matrix {
        &self.inverse_transform
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gradient() {
        let gradient = Gradient::default();
        assert!(
            gradient
                .color_at(&point![0., 0., 0.])
                .is_close(&Color::white())
        );
        assert!(
            gradient
                .color_at(&point![0.25, 0., 0.])
                .is_close(&Color::new(0.75, 0.75, 0.75))
        );
        assert!(
            gradient
                .color_at(&point![0.5, 0., 0.])
                .is_close(&Color::new(0.5, 0.5, 0.5))
        );
        assert!(
            gradient
                .color_at(&point![0.75, 0., 0.])
                .is_close(&Color::new(0.25, 0.25, 0.25))
        );
    }
}
