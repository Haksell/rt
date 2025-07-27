use {
    super::Pattern,
    crate::{
        color::Color,
        math::{Matrix, Tuple, transform},
    },
};

#[derive(Clone, Debug)]
pub struct Checker {
    a: Color,
    b: Color,
    inverse_transform: Matrix,
}

impl Checker {
    pub fn new(a: Color, b: Color, transform: Matrix) -> Self {
        Self {
            a,
            b,
            inverse_transform: transform.inverse(),
        }
    }
}

impl Default for Checker {
    fn default() -> Self {
        Self {
            a: Color::white(),
            b: Color::black(),
            inverse_transform: Matrix::identity(),
        }
    }
}

impl Pattern for Checker {
    fn color_at(&self, point: &Tuple) -> Color {
        if (point.x.floor() + point.y.floor() + point.z.floor()) as i64 & 1 == 0 {
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
    fn test_checker() {
        let pattern = Checker::default();
        assert_eq!(pattern.color_at(&point![0.5, 0.5, 0.5]), Color::white());
        assert_eq!(pattern.color_at(&point![0.5, 0.5, -0.5]), Color::black());
        assert_eq!(pattern.color_at(&point![0.5, -0.5, 0.5]), Color::black());
        assert_eq!(pattern.color_at(&point![0.5, -0.5, -0.5]), Color::white());
        assert_eq!(pattern.color_at(&point![-0.5, 0.5, 0.5]), Color::black());
        assert_eq!(pattern.color_at(&point![-0.5, 0.5, -0.5]), Color::white());
        assert_eq!(pattern.color_at(&point![-0.5, -0.5, 0.5]), Color::white());
        assert_eq!(pattern.color_at(&point![-0.5, -0.5, -0.5]), Color::black());
        assert_eq!(pattern.color_at(&point![42.5, -0.5, 0.5]), Color::black());
    }
}
