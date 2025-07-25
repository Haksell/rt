use crate::floats::is_close;

#[derive(Debug, PartialEq, Clone)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Tuple {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self { x, y, z, w }
    }

    pub fn new_point(x: f64, y: f64, z: f64) -> Self {
        Self::new(x, y, z, 1.)
    }

    pub fn new_vector(x: f64, y: f64, z: f64) -> Self {
        Self::new(x, y, z, 0.)
    }

    pub fn zero_point() -> Self {
        Self::new_point(0., 0., 0.)
    }

    pub fn zero_vector() -> Self {
        Self::new_vector(0., 0., 0.)
    }

    pub fn up() -> Self {
        Self::new_vector(0., 1., 0.)
    }

    pub fn is_point(&self) -> bool {
        self.w == 1.
    }

    pub fn is_vector(&self) -> bool {
        self.w == 0.
    }

    pub fn is_close(&self, rhs: &Self) -> bool {
        debug_assert!(is_close(self.w, rhs.w));
        is_close(self.x, rhs.x) && is_close(self.y, rhs.y) && is_close(self.z, rhs.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point() {
        let point_manual = Tuple {
            x: 4.3,
            y: -4.2,
            z: 3.1,
            w: 1.,
        };
        let point_constructor = Tuple::new_point(4.3, -4.2, 3.1);
        assert_eq!(point_manual, point_constructor);
        assert!(point_constructor.is_point());
        assert!(!point_constructor.is_vector());
    }

    #[test]
    fn test_vector() {
        let vector_manual = Tuple {
            x: 4.3,
            y: -4.2,
            z: 3.1,
            w: 0.,
        };
        let vector_constructor = Tuple::new_vector(4.3, -4.2, 3.1);
        assert_eq!(vector_manual, vector_constructor);
        assert!(vector_constructor.is_vector());
        assert!(!vector_constructor.is_point());
    }
}
