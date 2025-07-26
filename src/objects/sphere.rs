use {
    crate::{material::Material, matrix::Matrix, objects::Object, ray::Ray, tuple::Tuple, vector},
    std::cmp::Ordering,
};

#[derive(Debug, Clone)]
pub struct Sphere {
    inverse_transform: Matrix, // TODO: &'a Matrix ?
    material: Material,        // TODO: &Material ?
}

// TODO: make constructors part of Object trait
// TODO: accept inverse transform directly
impl Sphere {
    pub fn new(transform: Matrix, material: Material) -> Self {
        Self {
            inverse_transform: transform.inverse(),
            material,
        }
    }

    pub fn unit(material: Material) -> Self {
        Self {
            inverse_transform: Matrix::identity(),
            material,
        }
    }

    // Is it really plastic though?
    pub fn plastic(transform: Matrix) -> Self {
        Self {
            inverse_transform: transform.inverse(),
            material: Material::default(),
        }
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Self {
            inverse_transform: Matrix::identity(),
            material: Material::default(),
        }
    }
}

impl Object for Sphere {
    fn get_inverse_transform(&self) -> &Matrix {
        &self.inverse_transform
    }

    fn get_material(&self) -> &Material {
        &self.material
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
        crate::{color::Color, point, transform, vector},
        std::f64::consts::{FRAC_1_SQRT_2, TAU},
    };

    #[test]
    fn test_sphere_constructors() {
        let default = Sphere::default();
        let red = Sphere::unit(Material {
            color: Color::red(),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.,
        });
        let squashed = Sphere::plastic(transform::scale(1., 0.3, 1.));
        assert_eq!(default.material.ambient, squashed.material.ambient);
        assert_eq!(default.material.diffuse, squashed.material.diffuse);
        assert_eq!(default.material.shininess, squashed.material.shininess);
        assert_eq!(default.material.specular, squashed.material.specular);
        assert_eq!(default.inverse_transform, red.inverse_transform);
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
        let sphere = Sphere::plastic(transform::scale_constant(2.));
        assert_eq!(
            sphere.intersect(&Ray::new(point![0., 0., -5.], vector![0., 0., 1.],)),
            vec![3., 7.]
        );
        let sphere = Sphere::plastic(transform::translate(5., 0., 0.));
        assert_eq!(
            sphere.intersect(&Ray::new(point![0., 0., -5.], vector![0., 0., 1.],)),
            vec![]
        );
    }

    #[test]
    fn test_sphere_unit_normal_at() {
        let s = Sphere::default();
        assert_eq!(s.normal_at(&point![1., 0., 0.]), vector![1., 0., 0.]);
        assert_eq!(s.normal_at(&point![0., 1., 0.]), Tuple::up());
        assert_eq!(s.normal_at(&point![0., 0., 1.]), vector![0., 0., 1.]);
        let sqrt3_third = 3.0f64.sqrt() / 3.;
        assert!(
            s.normal_at(&point![sqrt3_third, sqrt3_third, sqrt3_third])
                .is_close(&vector![sqrt3_third, sqrt3_third, sqrt3_third])
        );
    }

    #[test]
    fn test_sphere_translated_normal_at() {
        assert!(
            Sphere::plastic(transform::translate(0., 1., 0.))
                .normal_at(&point![0., 1. + FRAC_1_SQRT_2, -FRAC_1_SQRT_2])
                .is_close(&vector![0., FRAC_1_SQRT_2, -FRAC_1_SQRT_2])
        );
    }

    #[test]
    fn test_sphere_transformed_normal_at() {
        assert!(
            Sphere::plastic(transform::scale(1., 0.5, 1.) * transform::rotate_z(TAU / 10.),)
                .normal_at(&point![0., FRAC_1_SQRT_2, -FRAC_1_SQRT_2]) // is it even on the sphere?
                .is_close(&vector![0., 0.97014254, -0.24253564])
        );
    }
}
