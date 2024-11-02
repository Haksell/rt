use super::Pattern;
use crate::{color::Color, matrix::Matrix, tuple::Tuple};

#[derive(Clone, Debug, PartialEq)]
pub struct Solid {
    c: Color,
    inverse_transform: Matrix,
}

impl Solid {
    pub fn new(c: Color) -> Self {
        Self {
            c,
            inverse_transform: Matrix::identity(),
        }
    }
}

impl Pattern for Solid {
    fn color_at(&self, _: &Tuple) -> Color {
        self.c
    }

    fn get_inverse_transform(&self) -> &Matrix {
        &self.inverse_transform
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solid() {
        assert_eq!(
            Solid::new(Color::red()).color_at(&Tuple::new_point(-2.5, 0., 0.)),
            Color::red()
        );
        assert_eq!(
            Solid::new(Color::red()).color_at(&Tuple::new_point(1.2, 3.4, 5.6)),
            Color::red()
        );
        assert_eq!(
            Solid::new(Color::white()).color_at(&Tuple::new_point(1.2, 3.4, 5.6)),
            Color::white()
        );
    }
}
