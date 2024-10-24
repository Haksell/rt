use super::Pattern;
use crate::{color::Color, tuple::Tuple};

#[derive(Clone, Debug, PartialEq)]
pub struct Stripe {
    a: Color,
    b: Color,
}

impl Stripe {
    pub fn default() -> Self {
        Self {
            a: Color::white(),
            b: Color::black(),
        }
    }

    pub fn new(a: Color, b: Color) -> Self {
        Self { a, b }
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
