use {
    crate::{
        color::Color,
        lighting::{PointLight, lighting},
        material::Material,
        objects::{Object, Sphere},
        ray::Ray,
        transform::scale_constant,
    },
    std::sync::LazyLock,
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
        const MIN_HIT_DISTANCE: f64 = 1e-6; // TODO: put somewhere else

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
                // TODO: calculations directly in lighting function?
                // TODO: handle inside (no ambient)
                let point = ray.at(hit_distance);
                let normal = object.normal_at(&point);
                let eye = -ray.direction;
                lighting(&**object, &self.lights[0], &point, &eye, &normal, false)
            } // WIP
        }
    }
}

#[cfg(test)]
pub const TESTING_WORLD: LazyLock<World> = LazyLock::new(|| World {
    objects: vec![
        Box::new(Sphere::unit(Material {
            color: Color::new(0.8, 1., 0.6),
            diffuse: 0.7,
            specular: 0.2,
            ..Material::default()
        })),
        Box::new(Sphere::plastic(scale_constant(0.5))),
    ],
    lights: vec![PointLight::new(Color::white(), point![-10., 10., -10.])],
});

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::{material::Material, objects::Sphere, transform::scale_constant, tuple::Tuple},
        std::sync::LazyLock,
    };

    #[test]
    fn test_color_at_void() {
        let ray = Ray::new(point![0., 0., -5.], Tuple::up());
        assert_eq!(TESTING_WORLD.color_at(&ray), Color::black());
    }

    #[test]
    fn test_color_at_sphere() {
        let ray = Ray::new(point![0., 0., -5.], vector![0., 0., 1.]);
        assert!(
            TESTING_WORLD
                .color_at(&ray)
                .is_close(&Color::new(0.3806612, 0.47582647, 0.2854959))
        );
    }

    #[test]
    fn test_color_at_between() {
        let ray = Ray::new(point![0., 0., 0.75], vector![0., 0., -1.]);
        assert!(
            TESTING_WORLD
                .color_at(&ray)
                .is_close(&(Color::white() * TESTING_WORLD.objects[1].get_material().ambient))
        );
    }
}
