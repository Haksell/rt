use super::Pattern;
use crate::{color::Color, matrix::Matrix, transform, tuple::Tuple};

#[derive(Clone, Debug, PartialEq)]
pub struct Stripe {
    a: Color,
    b: Color,
    inverse_transform: Matrix,
}

impl Stripe {
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

impl Pattern for Stripe {
    fn color_at(&self, point: &Tuple) -> &Color {
        if point.x.rem_euclid(2.0) < 1.0 {
            &self.a
        } else {
            &self.b
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
    fn test_color_at_x() {
        let pattern = Stripe::default();
        assert_eq!(
            pattern.color_at(&Tuple::new_point(-2.5, 0., 0.)),
            &Color::black()
        );
        assert_eq!(
            pattern.color_at(&Tuple::new_point(-2.0, 0., 0.)),
            &Color::white()
        );
        assert_eq!(
            pattern.color_at(&Tuple::new_point(-1.5, 0., 0.)),
            &Color::white()
        );
        assert_eq!(
            pattern.color_at(&Tuple::new_point(-1.0, 0., 0.)),
            &Color::black()
        );
        assert_eq!(
            pattern.color_at(&Tuple::new_point(-0.5, 0., 0.)),
            &Color::black()
        );
        assert_eq!(
            pattern.color_at(&Tuple::new_point(0.0, 0., 0.)),
            &Color::white()
        );
        assert_eq!(
            pattern.color_at(&Tuple::new_point(0.5, 0., 0.)),
            &Color::white()
        );
        assert_eq!(
            pattern.color_at(&Tuple::new_point(1.0, 0., 0.)),
            &Color::black()
        );
        assert_eq!(
            pattern.color_at(&Tuple::new_point(1.5, 0., 0.)),
            &Color::black()
        );
        assert_eq!(
            pattern.color_at(&Tuple::new_point(2.0, 0., 0.)),
            &Color::white()
        );
        assert_eq!(
            pattern.color_at(&Tuple::new_point(2.5, 0., 0.)),
            &Color::white()
        );
        assert_eq!(
            pattern.color_at(&Tuple::new_point(3.0, 0., 0.)),
            &Color::black()
        );
    }

    #[test]
    fn test_color_at_y() {
        let pattern = Stripe::default();
        let w = &Color::white();
        assert_eq!(pattern.color_at(&Tuple::new_point(0., 0., 0.)), w);
        assert_eq!(pattern.color_at(&Tuple::new_point(0., 1., 0.)), w);
        assert_eq!(pattern.color_at(&Tuple::new_point(0., 2., 0.)), w);
    }

    #[test]
    fn test_color_at_z() {
        let pattern = Stripe::default();
        let w = &Color::white();
        assert_eq!(pattern.color_at(&Tuple::new_point(0., 0., 0.)), w);
        assert_eq!(pattern.color_at(&Tuple::new_point(0., 0., 1.)), w);
        assert_eq!(pattern.color_at(&Tuple::new_point(0., 0., 2.)), w);
    }
}
