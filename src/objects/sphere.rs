use {
    crate::{matrix::Matrix, objects::Object, ray::Ray, tuple::Tuple, vector},
    std::cmp::Ordering,
};

#[derive(Debug)]
pub struct Sphere {
    pub inverse_transform: Matrix,
}

// TODO: make constructors part of Object trait
// TODO: accept inverse transform directly
impl Sphere {
    pub fn new(transform: Matrix) -> Self {
        Self {
            inverse_transform: transform.inverse(),
        }
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Self {
            inverse_transform: Matrix::identity(),
        }
    }
}

impl Object for Sphere {
    fn get_inverse_transform(&self) -> &Matrix {
        &self.inverse_transform
    }

    fn local_intersect(&self, object_ray: &Ray) -> Vec<f64> {
        let a = object_ray.direction.magnitude_squared();
        let b = 2. * object_ray.direction.dot(&object_ray.origin);
        let c = object_ray.origin.magnitude_squared() - 1.;
        let discriminant = b * b - 4. * a * c;
        match discriminant.total_cmp(&0.) {
            Ordering::Less => vec![],
            Ordering::Equal => vec![-b / (2. * a)],
            Ordering::Greater => {
                let sqrt_discriminant = discriminant.sqrt();
                let factor = 0.5 / a;
                vec![
                    (-b - sqrt_discriminant) * factor,
                    (-b + sqrt_discriminant) * factor,
                ]
            }
        }
    }

    fn local_normal_at(&self, object_point: &Tuple) -> Tuple {
        vector![object_point.x, object_point.y, object_point.z]
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::{point, transform, vector},
    };

    #[test]
    fn test_sphere_constructors() {
        let default = Sphere::default();
        assert_eq!(default.inverse_transform, Matrix::identity());
        let big = Sphere::new(transform::scale_constant(4.0));
        assert_eq!(big.inverse_transform, transform::scale_constant(0.25));
    }

    #[test]
    fn test_sphere_unit_intersect() {
        let sphere = Sphere::default();

        let ray = Ray::new(point![0., 0., -5.], vector![0., 0., 1.]);
        assert_eq!(sphere.intersect(&ray), vec![4., 6.]);

        let ray = Ray::new(point![0., 1., -5.], vector![0., 0., 1.]);
        assert_eq!(sphere.intersect(&ray), vec![5.]);

        let ray = Ray::new(point![0., 2., -5.], vector![0., 0., 1.]);
        assert_eq!(sphere.intersect(&ray), vec![]);

        let ray = Ray::new(point![0., 0., 0.], vector![0., 0., 1.]);
        assert_eq!(sphere.intersect(&ray), vec![-1., 1.]);

        let ray = Ray::new(point![0., 0., 0.5], vector![0., 0., 1.]);
        assert_eq!(sphere.intersect(&ray), vec![-1.5, 0.5]);

        let ray = Ray::new(point![0., 0., 5.], vector![0., 0., 1.]);
        assert_eq!(sphere.intersect(&ray), vec![-6., -4.]);
    }

    #[test]
    fn test_sphere_transformed_intersect() {
        let sphere = Sphere::new(transform::scale_constant(2.));
        assert_eq!(
            sphere.intersect(&Ray::new(point![0., 0., -5.], vector![0., 0., 1.],)),
            vec![3., 7.]
        );
        let sphere = Sphere::new(transform::translate(5., 0., 0.));
        assert_eq!(
            sphere.intersect(&Ray::new(point![0., 0., -5.], vector![0., 0., 1.],)),
            vec![]
        );
    }
}
