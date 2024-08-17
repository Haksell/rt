use crate::{
    objects::{Intersection, Object},
    Float, Ray, Tuple,
};

#[allow(dead_code)] // TODO: remove
pub struct Computations<'a> {
    pub t: Float,
    pub object: &'a dyn Object,
    pub point: Tuple,
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
        if normalv.dot(&eyev) < 0.0 {
            inside = true;
            normalv = -normalv;
        }
        Self {
            t: intersection.t,
            object: intersection.object,
            point,
            eyev,
            normalv,
            inside,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Computations;
    use crate::{
        objects::{Intersection, Sphere},
        Ray, Tuple,
    };

    #[test]
    fn test_prepare_computations_outside() {
        let sphere = Sphere::default();
        let intersection = Intersection::new(&sphere, 4.0);
        let ray = Ray::new(
            Tuple::new_point(0.0, 0.0, -5.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let comps = Computations::prepare(&intersection, &ray);
        assert!(!comps.inside);
        assert_eq!(comps.t, 4.0);
        assert_eq!(comps.point, Tuple::new_point(0.0, 0.0, -1.0));
        assert_eq!(comps.eyev, Tuple::new_vector(0.0, 0.0, -1.0));
        assert_eq!(comps.normalv, Tuple::new_vector(0.0, 0.0, -1.0));
        // same reference
        assert!(std::ptr::eq(comps.object, intersection.object));
        assert!(std::ptr::eq(comps.object, &sphere));
        assert!(std::ptr::eq(intersection.object, &sphere));
    }

    #[test]
    fn test_prepare_computations_inside() {
        let sphere = Sphere::default();
        let intersection = Intersection::new(&sphere, 1.0);
        let ray = Ray::new(
            Tuple::new_point(0.0, 0.0, 0.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let comps = Computations::prepare(&intersection, &ray);
        assert!(comps.inside);
        assert_eq!(comps.t, 1.0);
        assert_eq!(comps.point, Tuple::new_point(0.0, 0.0, 1.0));
        assert_eq!(comps.eyev, Tuple::new_vector(0.0, 0.0, -1.0));
        assert_eq!(comps.normalv, Tuple::new_vector(0.0, 0.0, -1.0)); // inverted
    }
}
