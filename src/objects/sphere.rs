use super::{Intersection, Object};
use crate::{Matrix, Ray, Tuple};

#[derive(Debug)]
pub struct Sphere {
    // TODO: store inverse transform instead?
    pub transform: Matrix<4>, // TODO: &'a Matrix<4> ?
}

impl Sphere {
    pub fn unit() -> Self {
        Self {
            transform: Matrix::identity(),
        }
    }

    // TODO: pub fn new(transform: &Matrix<4>) so we don't need mut everywhere
}

impl Object for Sphere {
    fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let ray = ray.transform(&self.transform.inverse());
        let sphere_to_ray = ray.origin.clone();
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * ray.direction.dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;
        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
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
        world_normal.w = 0.0;
        world_normal.normalize()
    }

    fn set_transform(&mut self, transform: &Matrix<4>) {
        self.transform = transform.clone();
    }

    fn get_transform(&self) -> &Matrix<4> {
        &self.transform
    }
}

#[cfg(test)]
mod tests {
    use super::super::{Intersection, Object};
    use super::Sphere;
    use crate::{transform, Float, FloatExt, Matrix, Ray, Tuple};

    #[test]
    fn test_sphere_transform() {
        let mut s = Sphere::unit();
        assert_eq!(s.transform, Matrix::identity());
        s.set_transform(&transform::translate(2.0, 3.0, 4.0));
        assert_eq!(
            s.transform,
            Matrix::new(&[
                [1.0, 0.0, 0.0, 2.0],
                [0.0, 1.0, 0.0, 3.0],
                [0.0, 0.0, 1.0, 4.0],
                [0.0, 0.0, 0.0, 1.0],
            ])
        );
    }

    #[test]
    fn test_sphere_unit_intersect() {
        let sphere = Sphere::unit();
        let direction = Tuple::new_vector(0.0, 0.0, 1.0);
        let ray = Ray::new(&Tuple::new_point(0.0, 0.0, -5.0), &direction.clone());
        assert_eq!(
            sphere.intersect(&ray),
            vec![
                Intersection::new(&sphere, 4.0),
                Intersection::new(&sphere, 6.0)
            ]
        );
        let ray = Ray::new(&Tuple::new_point(0.0, 1.0, -5.0), &direction.clone());
        assert_eq!(
            sphere.intersect(&ray),
            vec![
                Intersection::new(&sphere, 5.0),
                Intersection::new(&sphere, 5.0)
            ]
        ); // at least for now
        let ray = Ray::new(&Tuple::new_point(0.0, 2.0, -5.0), &direction.clone());
        assert_eq!(sphere.intersect(&ray), vec![]);
        let ray = Ray::new(&Tuple::new_point(0.0, 0.0, 0.0), &direction.clone());
        assert_eq!(
            sphere.intersect(&ray),
            vec![
                Intersection::new(&sphere, -1.0),
                Intersection::new(&sphere, 1.0)
            ]
        );
        let ray = Ray::new(&Tuple::new_point(0.0, 0.0, 5.0), &direction.clone());
        assert_eq!(
            sphere.intersect(&ray),
            vec![
                Intersection::new(&sphere, -6.0),
                Intersection::new(&sphere, -4.0)
            ]
        );
    }

    #[test]
    fn test_sphere_transformed_intersect() {
        let mut sphere = Sphere::unit();
        sphere.set_transform(&transform::scale_constant(2.0));
        assert_eq!(
            sphere.intersect(&Ray::new(
                &Tuple::new_point(0.0, 0.0, -5.0),
                &Tuple::new_vector(0.0, 0.0, 1.0),
            )),
            vec![
                Intersection::new(&sphere, 3.0),
                Intersection::new(&sphere, 7.0)
            ]
        );
        sphere.set_transform(&transform::translate(5.0, 0.0, 0.0));
        assert_eq!(
            sphere.intersect(&Ray::new(
                &Tuple::new_point(0.0, 0.0, -5.0),
                &Tuple::new_vector(0.0, 0.0, 1.0),
            )),
            vec![]
        );
    }

    #[test]
    fn test_sphere_unit_normal_at() {
        let s = Sphere::unit();
        assert_eq!(
            s.normal_at(&Tuple::new_point(1.0, 0.0, 0.0)),
            Tuple::new_vector(1.0, 0.0, 0.0)
        );
        assert_eq!(
            s.normal_at(&Tuple::new_point(0.0, 1.0, 0.0)),
            Tuple::new_vector(0.0, 1.0, 0.0)
        );
        assert_eq!(
            s.normal_at(&Tuple::new_point(0.0, 0.0, 1.0)),
            Tuple::new_vector(0.0, 0.0, 1.0)
        );
        let sqrt3_third = (3.0 as Float).sqrt() / 3.0;
        assert!(s
            .normal_at(&Tuple::new_point(sqrt3_third, sqrt3_third, sqrt3_third))
            .is_close(&Tuple::new_vector(sqrt3_third, sqrt3_third, sqrt3_third)));
    }

    #[test]
    fn test_sphere_translated_normal_at() {
        let mut s = Sphere::unit();
        s.set_transform(&transform::translate(0.0, 1.0, 0.0));
        let sqrt_half = (0.5 as Float).sqrt();
        assert!(s
            .normal_at(&Tuple::new_point(0.0, 1.0 + sqrt_half, -sqrt_half))
            .is_close(&Tuple::new_vector(0.0, sqrt_half, -sqrt_half)));
    }

    #[test]
    fn test_sphere_transformed_normal_at() {
        let mut s = Sphere::unit();
        s.set_transform(
            &(transform::scale(1.0, 0.5, 1.0) * transform::rotate_z(Float::TAU / 10.0)),
        );
        let sqrt_half = (0.5 as Float).sqrt();
        assert!(s
            .normal_at(&Tuple::new_point(0.0, sqrt_half, -sqrt_half)) // is it even on the sphere?
            .is_close(&Tuple::new_vector(0.0, 0.97014254, -0.24253564)));
    }
}
