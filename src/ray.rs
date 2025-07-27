use crate::{math::Matrix, math::Tuple};

#[derive(Clone, Debug, PartialEq)]
pub struct Ray {
    pub origin: Tuple,
    pub direction: Tuple,
}

impl Ray {
    pub fn new(origin: Tuple, direction: Tuple) -> Self {
        debug_assert!(origin.is_point());
        debug_assert!(direction.is_vector());
        Self { origin, direction }
    }

    pub fn at(&self, t: f64) -> Tuple {
        self.origin + t * self.direction
    }

    pub fn transform(&self, m: &Matrix) -> Self {
        Self {
            origin: m * self.origin,
            direction: m * self.direction,
        }
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::{
            math::transform::{scale_xyz, translate},
            point, vector,
        },
    };

    #[test]
    fn test_ray_new_valid() {
        let ray = Ray::new(point![1., 2., 3.], vector![-4., 5.5, 6.]);
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
    fn test_ray_new_invalid() {
        if cfg!(debug_assertions) {
            let result =
                std::panic::catch_unwind(|| Ray::new(point![1., 2., 3.], point![-4., 5.5, 6.]));
            assert!(result.is_err(), "Expected panic in debug mode");
        }
    }

    #[test]
    fn test_ray_at() {
        let ray = Ray::new(point![1., 2., 3.], vector![-4., 5.5, 6.]);
        assert_eq!(ray.at(-1.5), point![7., -6.25, -6.]);
        assert_eq!(ray.at(-1.), point![5., -3.5, -3.]);
        assert_eq!(ray.at(-0.5), point![3., -0.75, 0.]);
        assert_eq!(ray.at(0.), point![1., 2., 3.]);
        assert_eq!(ray.at(0.5), point![-1., 4.75, 6.]);
        assert_eq!(ray.at(1.), point![-3., 7.5, 9.]);
        assert_eq!(ray.at(1.5), point![-5., 10.25, 12.]);
    }

    #[test]
    fn test_ray_translate() {
        assert_eq!(
            Ray::new(point![1., 2., 3.], Tuple::up()).transform(&translate(3., 4., 5.)),
            Ray::new(point![4., 6., 8.], Tuple::up())
        );
    }

    #[test]
    fn test_ray_scale() {
        assert_eq!(
            Ray::new(point![1., 2., 3.], Tuple::up()).transform(&scale_xyz(2., 3., 4.)),
            Ray::new(point![2., 6., 12.], vector![0., 3., 0.])
        );
    }
}
