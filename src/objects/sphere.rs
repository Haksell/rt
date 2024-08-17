use super::{Intersection, Object};
use crate::{material::Material, Matrix, Ray, Tuple};

#[derive(Debug)]
pub struct Sphere {
    // TODO: store inverse transform instead?
    pub transform: Matrix<4>, // TODO: &'a Matrix<4> ?
    pub material: Material,   // TODO: &Material ?
}

impl Sphere {
    pub fn new(transform: Matrix<4>, material: Material) -> Self {
        Self {
            transform,
            material,
        }
    }

    pub fn default() -> Self {
        Self {
            transform: Matrix::identity(),
            material: Material::default(),
        }
    }

    pub fn unit(material: Material) -> Self {
        Self {
            transform: Matrix::identity(),
            material,
        }
    }

    // Is it really plastic though?
    pub fn plastic(transform: Matrix<4>) -> Self {
        Self {
            transform,
            material: Material::default(),
        }
    }
}

impl Object for Sphere {
    fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let ray = ray.transform(&self.transform.inverse());
        let sphere_to_ray = ray.origin.clone();
        let a = ray.direction.dot(&ray.direction);
        let b = 2. * ray.direction.dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.;
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

    fn normal_at(&self, world_point: &Tuple) -> Tuple {
        let object_point = self.transform.inverse() * world_point.clone();
        let object_normal = Tuple::new_vector(object_point.x, object_point.y, object_point.z);
        // TODO: understand why the transpose
        let mut world_normal = self.transform.inverse().transpose() * object_normal;
        world_normal.w = 0.;
        world_normal.normalize()
    }

    fn set_transform(&mut self, transform: Matrix<4>) {
        self.transform = transform;
    }

    fn get_transform(&self) -> &Matrix<4> {
        &self.transform
    }

    fn set_material(&mut self, material: Material) {
        self.material = material;
    }

    fn get_material(&self) -> &Material {
        &self.material
    }
}

#[cfg(test)]
mod tests {
    use super::super::{Intersection, Object};
    use super::Sphere;
    use crate::material::Material;
    use crate::{transform, Color, Float, FloatExt, Matrix, Ray, Tuple};

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
        assert_eq!(default.transform, red.transform);
    }

    #[test]
    fn test_sphere_transform() {
        let mut s = Sphere::default();
        assert_eq!(s.transform, Matrix::identity());
        s.set_transform(transform::translate(2., 3., 4.));
        assert_eq!(
            s.transform,
            Matrix::new(&[
                [1., 0., 0., 2.],
                [0., 1., 0., 3.],
                [0., 0., 1., 4.],
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
        let mut sphere = Sphere::plastic(transform::scale_constant(2.));
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
        sphere.set_transform(transform::translate(5., 0., 0.));
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
        assert_eq!(
            s.normal_at(&Tuple::new_point(0., 1., 0.)),
            Tuple::new_vector(0., 1., 0.)
        );
        assert_eq!(
            s.normal_at(&Tuple::new_point(0., 0., 1.)),
            Tuple::new_vector(0., 0., 1.)
        );
        let sqrt3_third = (3. as Float).sqrt() / 3.;
        assert!(s
            .normal_at(&Tuple::new_point(sqrt3_third, sqrt3_third, sqrt3_third))
            .is_close(&Tuple::new_vector(sqrt3_third, sqrt3_third, sqrt3_third)));
    }

    #[test]
    fn test_sphere_translated_normal_at() {
        let sqrt_half = (0.5 as Float).sqrt();
        assert!(Sphere::plastic(transform::translate(0., 1., 0.))
            .normal_at(&Tuple::new_point(0., 1. + sqrt_half, -sqrt_half))
            .is_close(&Tuple::new_vector(0., sqrt_half, -sqrt_half)));
    }

    #[test]
    fn test_sphere_transformed_normal_at() {
        let sqrt_half = (0.5 as Float).sqrt();
        assert!(Sphere::plastic(
            transform::scale(1., 0.5, 1.) * transform::rotate_z(Float::TAU / 10.),
        )
        .normal_at(&Tuple::new_point(0., sqrt_half, -sqrt_half)) // is it even on the sphere?
        .is_close(&Tuple::new_vector(0., 0.97014254, -0.24253564)));
    }
}
