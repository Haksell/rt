use std::ops::{Add, Sub};

// TODO: SIMD
#[derive(Debug, PartialEq)]
pub struct Tuple {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

impl Tuple {
    pub fn new_point(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z, w: 1.0 }
    }

    pub fn new_vector(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z, w: 0.0 }
    }

    pub fn is_point(&self) -> bool {
        match self.w {
            0.0 => false,
            1.0 => true,
            _ => panic!("Tuple::w is invalid: {}", self.w),
        }
    }

    pub fn is_vector(&self) -> bool {
        match self.w {
            0.0 => true,
            1.0 => false,
            _ => panic!("Tuple::w is invalid: {}", self.w),
        }
    }
}

impl Add for Tuple {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl Sub for Tuple {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Tuple;

    #[test]
    fn test_point() {
        let point_manual = Tuple {
            x: 4.3,
            y: -4.2,
            z: 3.1,
            w: 1.0,
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
            w: 0.0,
        };
        let vector_constructor = Tuple::new_vector(4.3, -4.2, 3.1);
        assert_eq!(vector_manual, vector_constructor);
        assert!(vector_constructor.is_vector());
        assert!(!vector_constructor.is_point());
    }

    #[test]
    fn test_addition() {
        assert_eq!(
            Tuple::new_point(3.0, -2.0, 5.0) + Tuple::new_vector(-2.0, 3.0, 1.0),
            Tuple::new_point(1.0, 1.0, 6.0)
        );
    }

    #[test]
    fn test_subtraction() {
        assert_eq!(
            Tuple::new_point(3.0, 2.0, 1.0) - Tuple::new_point(5.0, 6.0, 7.0),
            Tuple::new_vector(-2.0, -4.0, -6.0)
        );
        assert_eq!(
            Tuple::new_vector(3.0, 2.0, 1.0) - Tuple::new_vector(5.0, 6.0, 7.0),
            Tuple::new_vector(-2.0, -4.0, -6.0)
        );
        assert_eq!(
            Tuple::new_point(3.0, 2.0, 1.0) - Tuple::new_vector(5.0, 6.0, 7.0),
            Tuple::new_point(-2.0, -4.0, -6.0)
        );
    }
}
