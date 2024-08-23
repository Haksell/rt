use crate::{
    objects::{Intersection, Object},
    Ray, Tuple,
};

const ACNE_EPSILON: f32 = 1e-5; // TODO: test best value

#[allow(dead_code)] // TODO: remove
pub struct Computations<'a> {
    pub t: f32,
    pub object: &'a dyn Object,
    pub point: Tuple,
    pub over_point: Tuple,
    pub eyev: Tuple,
    pub normalv: Tuple,
    pub inside: bool,
}

impl<'a> Computations<'a> {
    pub fn prepare(intersection: &'a Intersection, ray: &Ray) -> Self {
        let point = ray.position(intersection.t);
        let eyev = -ray.direction.clone();
        let mut normalv = intersection.object.normal_at(&point);
        let mut inside = false;
        if normalv.dot(&eyev) < 0. {
            inside = true;
            normalv = -normalv;
        }
        let over_point = point.clone() + normalv.clone() * ACNE_EPSILON;
        Self {
            t: intersection.t,
            object: intersection.object,
            point,
            over_point,
            eyev,
            normalv,
            inside,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Computations, ACNE_EPSILON};
    use crate::{
        objects::{Intersection, Sphere},
        transform::translate,
        Ray, Tuple,
    };

    #[test]
    fn test_prepare_computations_outside() {
        let sphere = Sphere::default();
        let intersection = Intersection::new(&sphere, 4.);
        let ray = Ray::new(Tuple::new_point(0., 0., -5.), Tuple::new_vector(0., 0., 1.));
        let comps = Computations::prepare(&intersection, &ray);
        assert!(!comps.inside);
        assert_eq!(comps.t, 4.);
        assert_eq!(comps.point, Tuple::new_point(0., 0., -1.));
        assert_eq!(comps.eyev, Tuple::new_vector(0., 0., -1.));
        assert_eq!(comps.normalv, Tuple::new_vector(0., 0., -1.));
        // same reference
        assert!(std::ptr::eq(comps.object, intersection.object));
        assert!(std::ptr::eq(comps.object, &sphere));
        assert!(std::ptr::eq(intersection.object, &sphere));
    }

    #[test]
    fn test_prepare_computations_inside() {
        let sphere = Sphere::default();
        let intersection = Intersection::new(&sphere, 1.);
        let ray = Ray::new(Tuple::new_point(0., 0., 0.), Tuple::new_vector(0., 0., 1.));
        let comps = Computations::prepare(&intersection, &ray);
        assert!(comps.inside);
        assert_eq!(comps.t, 1.);
        assert_eq!(comps.point, Tuple::new_point(0., 0., 1.));
        assert_eq!(comps.eyev, Tuple::new_vector(0., 0., -1.));
        assert_eq!(comps.normalv, Tuple::new_vector(0., 0., -1.)); // inverted
    }

    #[test]
    fn test_prepare_computations_over_point() {
        let ray = Ray::new(Tuple::new_point(0., 0., -5.), Tuple::new_vector(0., 0., 1.));
        let sphere = Sphere::plastic(translate(0., 0., 1.));
        let intersection = Intersection::new(&sphere, 5.);
        let comps = Computations::prepare(&intersection, &ray);
        assert!(comps.over_point.z < -ACNE_EPSILON / 2.);
        assert!(comps.over_point.z < comps.point.z);
    }
}
