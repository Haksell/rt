use crate::{Matrix, Tuple};

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

    pub fn position(&self, t: f64) -> Tuple {
        self.origin.clone() + t * self.direction.clone()
    }

    pub fn transform(&self, m: &Matrix) -> Self {
        Self {
            origin: m.clone() * self.origin.clone(),
            direction: m.clone() * self.direction.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::transform;

    use super::*;

    #[test]
    fn test_ray_new_valid() {
        let ray = Ray::new(
            Tuple::new_point(1., 2., 3.),
            Tuple::new_vector(-4., 5.5, 6.),
        );
        assert_eq!(ray.origin.x, 1.);
        assert_eq!(ray.origin.y, 2.);
        assert_eq!(ray.origin.z, 3.);
        assert_eq!(ray.origin.w, 1.);
        assert_eq!(ray.direction.x, -4.);
        assert_eq!(ray.direction.y, 5.5);
        assert_eq!(ray.direction.z, 6.);
        assert_eq!(ray.direction.w, 0.);
    }

    #[test]
    #[should_panic]
    fn test_ray_new_invalid() {
        Ray::new(Tuple::new_point(1., 2., 3.), Tuple::new_point(-4., 5.5, 6.));
    }

    #[test]
    fn test_ray_position() {
        let ray = Ray::new(
            Tuple::new_point(1., 2., 3.),
            Tuple::new_vector(-4., 5.5, 6.),
        );
        assert_eq!(ray.position(-1.5), Tuple::new_point(7., -6.25, -6.));
        assert_eq!(ray.position(-1.), Tuple::new_point(5., -3.5, -3.));
        assert_eq!(ray.position(-0.5), Tuple::new_point(3., -0.75, 0.));
        assert_eq!(ray.position(0.), Tuple::new_point(1., 2., 3.));
        assert_eq!(ray.position(0.5), Tuple::new_point(-1., 4.75, 6.));
        assert_eq!(ray.position(1.), Tuple::new_point(-3., 7.5, 9.));
        assert_eq!(ray.position(1.5), Tuple::new_point(-5., 10.25, 12.));
    }

    #[test]
    fn test_ray_translate() {
        assert_eq!(
            Ray::new(Tuple::new_point(1., 2., 3.), Tuple::up())
                .transform(&transform::translate(3., 4., 5.)),
            Ray::new(Tuple::new_point(4., 6., 8.), Tuple::up())
        );
    }

    #[test]
    fn test_ray_scale() {
        assert_eq!(
            Ray::new(Tuple::new_point(1., 2., 3.), Tuple::up())
                .transform(&transform::scale(2., 3., 4.)),
            Ray::new(Tuple::new_point(2., 6., 12.), Tuple::new_vector(0., 3., 0.))
        );
    }
}
