use crate::{color::Color, objects::Object, ray::Ray};

#[derive(Debug)]
pub struct World {
    pub objects: Vec<Box<dyn Object>>,
    // pub lights: Vec<PointLight>,
    // TODO: ambient_color: Color
}

impl World {
    // pub fn new(objects: Vec<Box<dyn Object>>, lights: Vec<PointLight>) -> Self {
    //     Self { objects, lights }
    // }

    pub fn new(objects: Vec<Box<dyn Object>>) -> Self {
        Self { objects }
    }

    pub fn color_at(&self, ray: &Ray) -> Color {
        const MIN_HIT_DISTANCE: f64 = 1e-6; // TODO: somewhere else
        let mut hit_object = None;
        let mut hit_distance = std::f64::INFINITY;
        for object in &self.objects {
            for t in object.intersect(ray) {
                if t >= MIN_HIT_DISTANCE && t < hit_distance {
                    hit_distance = t;
                    hit_object = Some(object);
                }
            }
        }
        match hit_object {
            None => Color::black(), // TODO: ambient color instead
            // TODO: shade_hit(self, &Computations::prepare(intersection, ray)),
            Some(object) => object.get_material().color, // WIP
        }
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::{objects::Sphere, transform::scale_constant, tuple::Tuple},
    };

    impl World {
        pub fn default() -> Self {
            Self {
                objects: vec![
                    Box::new(Sphere::default()),
                    Box::new(Sphere::plastic(scale_constant(0.5))),
                ],
                // lights: vec![PointLight::new(Color::white(), point![-10., 10., -10.])],
            }
        }
    }

    #[test]
    fn test_color_at_void() {
        let world = World::default();
        let ray = Ray::new(point![0., 0., -5.], Tuple::up());
        assert_eq!(world.color_at(&ray), Color::black());
    }

    #[test]
    fn test_color_at_sphere() {
        let world = World::default();
        let ray = Ray::new(point![0., 0., -5.], vector![0., 0., 1.]);
        assert!(
            // .is_close(&color![0.3806612, 0.47582647, 0.2854959])
            world.color_at(&ray).is_close(&Color::white())
        );
    }

    // #[test]
    // fn test_color_at_between() {
    //     let world = World::default();
    //     let ray = Ray::new(point![0., 0., 0.75], vector![0., 0., -1.]);
    //     assert!(
    //         world
    //             .color_at(&ray)
    //             .is_close(&(Color::white() * world.objects[1].get_material().ambient))
    //     );
    // }
}
