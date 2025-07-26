mod point_light;

pub use point_light::PointLight;

use crate::{
    color::Color, computations::Computations, math::Tuple, objects::Object, ray::Ray, world::World,
};

pub fn lighting(
    object: &dyn Object,
    light: &PointLight, // TODO: &[PointLight]
    point: &Tuple,
    eyev: &Tuple,
    normalv: &Tuple,
    in_shadow: bool, // TODO: not an argument
) -> Color {
    let material = object.get_material();
    let effective_color = object.color_at(&point) * light.intensity;
    let ambient = effective_color * material.ambient; // TODO: shouldn't depend on light.intensity

    if in_shadow {
        return ambient;
    }

    let lightv = (light.position - point).normalize();
    let light_dot_normal = lightv.dot(normalv);
    if light_dot_normal <= 0. {
        return ambient;
    }
    let diffuse = effective_color * material.diffuse * light_dot_normal;

    let reflectv = -lightv.reflect(normalv);
    let reflect_dot_eye = reflectv.dot(eyev);
    if reflect_dot_eye <= 0. {
        return ambient + diffuse;
    }
    let factor = reflect_dot_eye.powf(material.shininess);
    let specular = light.intensity * material.specular * factor;

    ambient + diffuse + specular
}

pub fn is_shadowed(world: &World, point: &Tuple) -> bool {
    // TODO: for all lights in world
    let v = world.lights[0].position - point;
    let distance = v.magnitude();
    let direction = v.normalize();
    let ray = Ray::new(*point, direction);
    world.intersect(&ray).is_some_and(|(_, t)| t < distance)
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::{
            material::Material,
            math::transform::{scale_constant, translate},
            objects::Sphere,
            point, vector,
            world::{TESTING_WORLD, World},
        },
        std::f64::consts::FRAC_1_SQRT_2,
    };

    #[test]
    fn test_lighting_light_behind_eye() {
        let sphere = Sphere::default();
        let position = Tuple::zero_point();
        let eyev = vector![0., 0., -1.];
        let normalv = vector![0., 0., -1.];
        let light = PointLight::new(Color::white(), point![0., 0., -10.]);
        assert!(
            lighting(&sphere, &light, &position, &eyev, &normalv, false)
                .is_close(&Color::new(1.9, 1.9, 1.9))
        )
    }

    #[test]
    fn test_lighting_eye_diagonal() {
        let sphere = Sphere::default();
        let position = Tuple::zero_point();
        let eyev = vector![0., FRAC_1_SQRT_2, -FRAC_1_SQRT_2];
        let normalv = vector![0., 0., -1.];
        let light = PointLight::new(Color::white(), point![0., 0., -10.]);
        // 0.9 + 0.1 = 1., specular has disappeared
        assert!(
            lighting(&sphere, &light, &position, &eyev, &normalv, false).is_close(&Color::white())
        )
    }

    #[test]
    fn test_lighting_light_diagonal() {
        let sphere = Sphere::default();
        let position = Tuple::zero_point();
        let eyev = vector![0., 0., -1.];
        let normalv = vector![0., 0., -1.];
        let light = PointLight::new(Color::white(), point![0., 10., -10.]);
        assert!(
            lighting(&sphere, &light, &position, &eyev, &normalv, false)
                .is_close(&Color::new(0.73639613, 0.73639613, 0.73639613))
        )
    }

    #[test]
    fn test_lighting_both_diagonal_full_specular() {
        let sphere = Sphere::default();
        let position = Tuple::zero_point();
        let eyev = vector![0., -FRAC_1_SQRT_2, -FRAC_1_SQRT_2];
        let normalv = vector![0., 0., -1.];
        let light = PointLight::new(Color::white(), point![0., 10., -10.]);
        assert!(
            lighting(&sphere, &light, &position, &eyev, &normalv, false)
                .is_close(&Color::new(1.6363962, 1.6363962, 1.6363962))
        )
    }

    #[test]
    fn test_lighting_light_behind_surface() {
        let sphere = Sphere::default();
        let position = Tuple::zero_point();
        let eyev = vector![0., 0., -1.];
        let normalv = vector![0., 0., -1.];
        let light = PointLight::new(Color::white(), point![0., 0., 10.]);
        assert!(
            lighting(&sphere, &light, &position, &eyev, &normalv, false)
                .is_close(&Color::new(0.1, 0.1, 0.1))
        )
    }

    #[test]
    fn test_lighting_surface_in_shadow() {
        let sphere = Sphere::default();
        let position = Tuple::zero_point();
        let eyev = vector![0., 0., -1.];
        let normalv = vector![0., 0., -1.];
        let light = PointLight::new(Color::white(), point![0., 0., -10.]);
        assert!(
            lighting(&sphere, &light, &position, &eyev, &normalv, true)
                .is_close(&Color::new(0.1, 0.1, 0.1))
        )
    }

    // #[test]
    // fn test_lighting_stripe() {
    //     let material = Material::new(Box::new(Stripe::default()), 1.0, 0.0, 0.0, 0.0);
    //     let sphere = Sphere::unit(material);
    //     let eyev = vector![0., 0., -1.];
    //     let normalv = vector![0., 0., -1.];
    //     let light = PointLight::new(Color::white(), point![0., 0., -10.]);

    //     assert_eq!(
    //         lighting(
    //             &sphere,
    //             &light,
    //             &point![0.9, 0.0, 0.0],
    //             &eyev,
    //             &normalv,
    //             true
    //         ),
    //         Color::white()
    //     );

    //     assert_eq!(
    //         lighting(
    //             &sphere,
    //             &light,
    //             &point![1.1, 0.0, 0.0],
    //             &eyev,
    //             &normalv,
    //             true
    //         ),
    //         Color::black()
    //     );
    // }

    #[test]
    fn test_is_shadowed_diagonal() {
        assert!(!is_shadowed(&TESTING_WORLD, &point![0., 10., 0.]));
    }

    #[test]
    fn test_is_shadowed_sphere_middle() {
        assert!(is_shadowed(&TESTING_WORLD, &point![10., -10., 10.]));
    }

    #[test]
    fn test_is_shadowed_sphere_inside() {
        assert!(is_shadowed(&TESTING_WORLD, &point![0., 0., 0.]));
    }

    #[test]
    fn test_is_shadowed_light_middle() {
        assert!(!is_shadowed(&TESTING_WORLD, &point![-20., 20., -20.]));
    }

    #[test]
    fn test_is_shadowed_point_middle() {
        assert!(!is_shadowed(&TESTING_WORLD, &point![-2., 2., -2.]));
    }
}
