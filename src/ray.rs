use crate::{Float, Matrix, Tuple};

#[derive(Clone, Debug, PartialEq)]
pub struct Ray {
    pub origin: Tuple,
    pub direction: Tuple,
}

impl Ray {
    pub fn new(origin: Tuple, direction: Tuple) -> Self {
        // TODO: remove asserts for optimization?
        assert!(origin.is_point());
        assert!(direction.is_vector());
        Self { origin, direction }
    }

    pub fn position(&self, t: Float) -> Tuple {
        self.origin.clone() + t * self.direction.clone()
    }

    pub fn transform(&self, m: &Matrix<4>) -> Self {
        Self {
            origin: m.clone() * self.origin.clone(),
            direction: m.clone() * self.direction.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Ray;
    use crate::{transform, Tuple};

    #[test]
    fn test_ray_new_valid() {
        let ray = Ray::new(
            Tuple::new_point(1.0, 2.0, 3.0),
            Tuple::new_vector(-4.0, 5.5, 6.0),
        );
        assert_eq!(ray.origin.x, 1.0);
        assert_eq!(ray.origin.y, 2.0);
        assert_eq!(ray.origin.z, 3.0);
        assert_eq!(ray.origin.w, 1.0);
        assert_eq!(ray.direction.x, -4.0);
        assert_eq!(ray.direction.y, 5.5);
        assert_eq!(ray.direction.z, 6.0);
        assert_eq!(ray.direction.w, 0.0);
    }

    #[test]
    #[should_panic]
    fn test_ray_new_invalid() {
        Ray::new(
            Tuple::new_point(1.0, 2.0, 3.0),
            Tuple::new_point(-4.0, 5.5, 6.0),
        );
    }

    #[test]
    fn test_ray_position() {
        let ray = Ray::new(
            Tuple::new_point(1.0, 2.0, 3.0),
            Tuple::new_vector(-4.0, 5.5, 6.0),
        );
        assert_eq!(ray.position(-1.5), Tuple::new_point(7.0, -6.25, -6.0));
        assert_eq!(ray.position(-1.0), Tuple::new_point(5.0, -3.5, -3.0));
        assert_eq!(ray.position(-0.5), Tuple::new_point(3.0, -0.75, 0.0));
        assert_eq!(ray.position(0.0), Tuple::new_point(1.0, 2.0, 3.0));
        assert_eq!(ray.position(0.5), Tuple::new_point(-1.0, 4.75, 6.0));
        assert_eq!(ray.position(1.0), Tuple::new_point(-3.0, 7.5, 9.0));
        assert_eq!(ray.position(1.5), Tuple::new_point(-5.0, 10.25, 12.0));
    }

    #[test]
    fn test_ray_translate() {
        assert_eq!(
            Ray::new(
                Tuple::new_point(1.0, 2.0, 3.0),
                Tuple::new_vector(0.0, 1.0, 0.0)
            )
            .transform(&transform::translate(3.0, 4.0, 5.0)),
            Ray::new(
                Tuple::new_point(4.0, 6.0, 8.0),
                Tuple::new_vector(0.0, 1.0, 0.0)
            )
        );
    }

    #[test]
    fn test_ray_scale() {
        assert_eq!(
            Ray::new(
                Tuple::new_point(1.0, 2.0, 3.0),
                Tuple::new_vector(0.0, 1.0, 0.0)
            )
            .transform(&transform::scale(2.0, 3.0, 4.0)),
            Ray::new(
                Tuple::new_point(2.0, 6.0, 12.0),
                Tuple::new_vector(0.0, 3.0, 0.0)
            )
        );
    }
}
