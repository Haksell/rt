use super::{Intersection, Object};
use crate::{material::Material, Matrix, Ray, Tuple};

#[derive(Debug)]
pub struct Sphere {
    pub inverse_transform: Matrix, // TODO: &'a Matrix ?
    pub material: Material,        // TODO: &Material ?
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

    pub fn default() -> Self {
        Self {
            inverse_transform: Matrix::identity(),
            material: Material::default(),
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

impl Object for Sphere {
    fn object_space_intersect(&self, object_ray: &Ray) -> Vec<Intersection> {
        let a = object_ray.direction.magnitude_squared();
        let b = 2. * object_ray.direction.dot(&object_ray.origin);
        let c = object_ray.origin.magnitude_squared() - 1.;
        let discriminant = b * b - 4. * a * c;
        if discriminant < 0. {
            return vec![];
        }
        let neg_b = -b;
        let sqrt_discriminant = discriminant.sqrt();
        let factor = 0.5 / a;
        vec![
            Intersection::new(self, (neg_b - sqrt_discriminant) * factor),
            Intersection::new(self, (neg_b + sqrt_discriminant) * factor),
        ]
    }

    fn object_space_normal_at(&self, object_point: &Tuple) -> Tuple {
        Tuple::new_vector(object_point.x, object_point.y, object_point.z)
    }

    fn get_inverse_transform(&self) -> &Matrix {
        &self.inverse_transform
    }

    fn get_material(&self) -> &Material {
        &self.material
    }
}

#[cfg(test)]
mod tests {
    use super::super::{Intersection, Object};
    use super::Sphere;
    use crate::{material::Material, transform, Color, Matrix, Ray, Tuple};

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
        assert_eq!(default.material, squashed.material);
        assert_eq!(default.inverse_transform, red.inverse_transform);
    }

    #[test]
    fn test_sphere_transform() {
        assert_eq!(Sphere::default().inverse_transform, Matrix::identity());
        assert_eq!(
            Sphere::plastic(transform::translate(2., 3., 4.)).inverse_transform,
            Matrix::new([
                [1., 0., 0., -2.],
                [0., 1., 0., -3.],
                [0., 0., 1., -4.],
                [0., 0., 0., 1.],
            ])
        );
    }

    #[test]
    fn test_sphere_unit_intersect() {
        let sphere = Sphere::default();
        let direction = Tuple::new_vector(0., 0., 1.);
        let ray = Ray::new(Tuple::new_point(0., 0., -5.), direction.clone());
        assert_eq!(
            sphere.intersect(&ray),
            vec![
                Intersection::new(&sphere, 4.),
                Intersection::new(&sphere, 6.)
            ]
        );
        let ray = Ray::new(Tuple::new_point(0., 1., -5.), direction.clone());
        assert_eq!(
            sphere.intersect(&ray),
            vec![
                Intersection::new(&sphere, 5.),
                Intersection::new(&sphere, 5.)
            ]
        ); // at least for now
        let ray = Ray::new(Tuple::new_point(0., 2., -5.), direction.clone());
        assert_eq!(sphere.intersect(&ray), vec![]);
        let ray = Ray::new(Tuple::zero_point(), direction.clone());
        assert_eq!(
            sphere.intersect(&ray),
            vec![
                Intersection::new(&sphere, -1.),
                Intersection::new(&sphere, 1.)
            ]
        );
        let ray = Ray::new(Tuple::new_point(0., 0., 5.), direction.clone());
        assert_eq!(
            sphere.intersect(&ray),
            vec![
                Intersection::new(&sphere, -6.),
                Intersection::new(&sphere, -4.)
            ]
        );
    }

    #[test]
    fn test_sphere_transformed_intersect() {
        let sphere = Sphere::plastic(transform::scale_constant(2.));
        assert_eq!(
            sphere.intersect(&Ray::new(
                Tuple::new_point(0., 0., -5.),
                Tuple::new_vector(0., 0., 1.),
            )),
            vec![
                Intersection::new(&sphere, 3.),
                Intersection::new(&sphere, 7.)
            ]
        );
        let sphere = Sphere::plastic(transform::translate(5., 0., 0.));
        assert_eq!(
            sphere.intersect(&Ray::new(
                Tuple::new_point(0., 0., -5.),
                Tuple::new_vector(0., 0., 1.),
            )),
            vec![]
        );
    }

    #[test]
    fn test_sphere_unit_normal_at() {
        let s = Sphere::default();
        assert_eq!(
            s.normal_at(&Tuple::new_point(1., 0., 0.)),
            Tuple::new_vector(1., 0., 0.)
        );
        assert_eq!(s.normal_at(&Tuple::new_point(0., 1., 0.)), Tuple::up());
        assert_eq!(
            s.normal_at(&Tuple::new_point(0., 0., 1.)),
            Tuple::new_vector(0., 0., 1.)
        );
        let sqrt3_third = (3. as f64).sqrt() / 3.;
        assert!(s
            .normal_at(&Tuple::new_point(sqrt3_third, sqrt3_third, sqrt3_third))
            .is_close(&Tuple::new_vector(sqrt3_third, sqrt3_third, sqrt3_third)));
    }

    #[test]
    fn test_sphere_translated_normal_at() {
        let sqrt_half = std::f64::consts::FRAC_1_SQRT_2;
        assert!(Sphere::plastic(transform::translate(0., 1., 0.))
            .normal_at(&Tuple::new_point(0., 1. + sqrt_half, -sqrt_half))
            .is_close(&Tuple::new_vector(0., sqrt_half, -sqrt_half)));
    }

    #[test]
    fn test_sphere_transformed_normal_at() {
        let sqrt_half = std::f64::consts::FRAC_1_SQRT_2;
        assert!(Sphere::plastic(
            transform::scale(1., 0.5, 1.) * transform::rotate_z(std::f64::consts::TAU / 10.),
        )
        .normal_at(&Tuple::new_point(0., sqrt_half, -sqrt_half)) // is it even on the sphere?
        .is_close(&Tuple::new_vector(0., 0.97014254, -0.24253564)));
    }
}
