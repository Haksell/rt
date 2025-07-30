use crate::{math::Tuple, objects::Object, ray::Ray};

const ACNE_EPSILON: f64 = 1e-6;

pub struct Computations<'a> {
    pub t: f64,
    pub object: &'a dyn Object,
    pub point: Tuple,
    pub over_point: Tuple,
    pub eyev: Tuple,
    pub normalv: Tuple,
    pub reflectv: Tuple,
    pub inside: bool,
    pub n1: f64,
    pub n2: f64,
}

impl<'a> Computations<'a> {
    pub fn prepare(object: &'a dyn Object, t: f64, ray: &Ray) -> Self {
        let point = ray.at(t);
        let eyev = -ray.direction;
        let mut normalv = object.normal_at(&point);
        let inside = normalv.dot(&eyev) < 0.;
        if inside {
            normalv = -normalv;
        }
        let reflectv = ray.direction.reflect(&normalv);
        let over_point = point + normalv * ACNE_EPSILON;
        let n1 = 0.0;
        let n2 = 0.0;

        Self {
            t,
            object,
            point,
            over_point,
            eyev,
            normalv,
            reflectv,
            inside,
            n1,
            n2,
        }
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::{
            material::Material,
            math::{
                Matrix, is_close,
                transform::{scale, translate, translate_z},
            },
            objects::{Plane, Sphere},
            point, vector,
        },
        std::f64::consts::SQRT_2,
    };

    #[test]
    fn test_prepare_computations_outside() {
        let sphere = Sphere::default();
        let ray = Ray::new(point![0., 0., -5.], vector![0., 0., 1.]);
        let comps = Computations::prepare(&sphere, 4., &ray);
        assert_eq!(comps.t, 4.);
        assert!(std::ptr::eq(comps.object, &sphere));
        assert_eq!(comps.point, point![0., 0., -1.]);
        assert_eq!(comps.eyev, vector![0., 0., -1.]);
        assert_eq!(comps.normalv, vector![0., 0., -1.]);
        assert!(!comps.inside);
    }

    #[test]
    fn test_prepare_computations_inside() {
        let sphere = Sphere::default();
        let ray = Ray::new(point![0., 0., 0.], vector![0., 0., 1.]);
        let comps = Computations::prepare(&sphere, 1., &ray);
        assert_eq!(comps.t, 1.);
        assert!(std::ptr::eq(comps.object, &sphere));
        assert_eq!(comps.point, point![0., 0., 1.]);
        assert_eq!(comps.eyev, vector![0., 0., -1.]);
        assert_eq!(comps.normalv, vector![0., 0., -1.]); // inverted
        assert!(comps.inside);
    }

    #[test]
    fn test_prepare_computations_over_point() {
        let ray = Ray::new(point![0., 0., -5.], vector![0., 0., 1.]);
        let sphere = Sphere::plastic(translate_z(1.));
        let comps = Computations::prepare(&sphere, 5., &ray);
        assert!(comps.over_point.z < -ACNE_EPSILON / 2.);
        assert!(comps.over_point.z < comps.point.z);
    }

    #[test]
    fn test_prepare_computations_reflection() {
        let plane = Plane::default();
        let ray = Ray::new(point![0., 1., -1.], vector![0., -SQRT_2 / 2., SQRT_2 / 2.]);
        let comps = Computations::prepare(&plane, SQRT_2, &ray);
        assert!(
            comps
                .reflectv
                .is_close(&vector![0., SQRT_2 / 2., SQRT_2 / 2.])
        );
    }

    #[test]
    fn test_prepare_computations_refraction() {
        fn glass_sphere(transform: Matrix, refractive_index: f64) -> Sphere {
            Sphere::new(
                transform,
                Material {
                    refractive_index,
                    ..Material::glass()
                },
            )
        }

        // a contains b and c, which overlap each other
        let a = glass_sphere(scale(2.0), 1.5);
        let b = glass_sphere(translate_z(-0.25), 2.0);
        let c = glass_sphere(translate_z(0.25), 2.5);

        let ray = Ray::new(point![0., 0., -4.], vector![0., 0., 1.]);
        for (sphere, t, expected_n1, expected_n2) in [
            (&a, 2.0, 1.0, 1.5),  // enters a
            (&b, 2.75, 1.5, 2.0), // enters b
            (&c, 3.25, 2.0, 2.5), // enters c
            (&b, 4.75, 2.5, 2.5), // exits b
            (&c, 5.25, 2.5, 1.5), // exits c
            (&a, 6.0, 1.5, 1.0),  // exits a
        ] {
            let comps = Computations::prepare(sphere, t, &ray);
            assert_eq!(comps.n1, expected_n1);
            assert_eq!(comps.n2, expected_n2);
        }
    }
}
