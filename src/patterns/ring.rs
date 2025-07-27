use {
    super::Pattern,
    crate::{
        color::Color,
        math::{Matrix, Tuple, transform},
    },
};

#[derive(Clone, Debug, PartialEq)]
pub struct Ring {
    a: Color,
    b: Color,
    inverse_transform: Matrix,
}

impl Ring {
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

impl Pattern for Ring {
    fn color_at(&self, point: &Tuple) -> Color {
        if f64::hypot(point.x, point.z).rem_euclid(2.0) < 1.0 {
            self.a
        } else {
            self.b
        }
    }

    fn get_inverse_transform(&self) -> &Matrix {
        &self.inverse_transform
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ring() {
        let ring = Ring::default();
        assert!(ring.color_at(&point![0., 0., 0.]).is_close(&Color::white()));
        assert!(ring.color_at(&point![1., 0., 0.]).is_close(&Color::black()));
        assert!(ring.color_at(&point![0., 0., 1.]).is_close(&Color::black()));
        assert!(
            ring.color_at(&point![0.71, 0., 0.71])
                .is_close(&Color::black())
        );
    }
}
