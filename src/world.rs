use crate::{
    color::Color,
    lighting::{PointLight, lighting},
    objects::Object,
    ray::Ray,
};

#[derive(Debug)]
pub struct World {
    // TODO: ambient_color: Color
    pub objects: Vec<Box<dyn Object>>,
    pub lights: Vec<PointLight>,
}

impl World {
    pub fn new(objects: Vec<Box<dyn Object>>, lights: Vec<PointLight>) -> Self {
        Self { objects, lights }
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
            Some(object) => {
                let point = ray.at(hit_distance);
                let normal = object.normal_at(&point);
                let eye = -ray.direction;
                lighting(&**object, &self.lights[0], &point, &eye, &normal, false)
            } // WIP
        }
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::{material::Material, objects::Sphere, transform::scale_constant, tuple::Tuple},
    };

    impl World {
        pub fn default() -> Self {
            Self {
                objects: vec![
                    Box::new(Sphere::unit(Material {
                        color: color![0.8, 1., 0.6],
                        diffuse: 0.7,
                        specular: 0.2,
                        ..Material::default()
                    })),
                    Box::new(Sphere::plastic(scale_constant(0.5))),
                ],
                lights: vec![PointLight::new(Color::white(), point![-10., 10., -10.])],
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
        let target = color![0.3806612, 0.47582647, 0.2854959];
        assert!(
            world.color_at(&ray).is_close(&target),
            "actual={:?}\nexpected={:?}",
            world.color_at(&ray),
            target
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
