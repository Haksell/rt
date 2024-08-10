mod point_light;

use crate::{Color, Material, Tuple};
pub use point_light::PointLight;

pub fn lighting(
    material: &Material,
    light: &PointLight,
    point: &Tuple,
    eyev: &Tuple,
    normalv: &Tuple,
) -> Color {
    let effective_color = material.color * light.intensity;
    let lightv = (light.position.clone() - point.clone()).normalize();
    let ambient = effective_color * material.ambient;
    let light_dot_normal = lightv.dot(normalv);
    let mut diffuse = Color::black();
    let mut specular = Color::black();
    if light_dot_normal > 0.0 {
        diffuse = effective_color * material.diffuse * light_dot_normal;
        let reflectv = (-lightv).reflect(normalv);
        let reflect_dot_eye = reflectv.dot(eyev);
        if reflect_dot_eye > 0.0 {
            let factor = reflect_dot_eye.powf(material.shininess);
            specular = light.intensity * material.specular * factor;
        }
    }
    ambient + diffuse + specular
}

#[cfg(test)]
mod tests {
    use super::lighting;
    use super::point_light::PointLight;
    use crate::{Color, Float, Material, Tuple};

    #[test]
    fn test_lighting_light_behind_eye() {
        let material = Material::default();
        let position = Tuple::zero_point();
        let eyev = Tuple::new_vector(0.0, 0.0, -1.0);
        let normalv = Tuple::new_vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Color::white(), Tuple::new_point(0.0, 0.0, -10.0));
        assert!(lighting(&material, &light, &position, &eyev, &normalv)
            .is_close(&Color::new(1.9, 1.9, 1.9)))
    }

    #[test]
    fn test_lighting_eye_diagonal() {
        let material = Material::default();
        let position = Tuple::zero_point();
        let sqrt_half = (0.5 as Float).sqrt();
        let eyev = Tuple::new_vector(0.0, sqrt_half, -sqrt_half);
        let normalv = Tuple::new_vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Color::white(), Tuple::new_point(0.0, 0.0, -10.0));
        // 0.9 + 0.1 = 1.0, specular has disappeared
        assert!(lighting(&material, &light, &position, &eyev, &normalv).is_close(&Color::white()))
    }

    #[test]
    fn test_lighting_light_diagonal() {
        let material = Material::default();
        let position = Tuple::zero_point();
        let eyev = Tuple::new_vector(0.0, 0.0, -1.0);
        let normalv = Tuple::new_vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Color::white(), Tuple::new_point(0.0, 10.0, -10.0));
        assert!(lighting(&material, &light, &position, &eyev, &normalv)
            .is_close(&Color::new(0.73639613, 0.73639613, 0.73639613)))
    }

    #[test]
    fn test_lighting_both_diagonal_full_specular() {
        let material = Material::default();
        let position = Tuple::zero_point();
        let sqrt_half = (0.5 as Float).sqrt();
        let eyev = Tuple::new_vector(0.0, -sqrt_half, -sqrt_half);
        let normalv = Tuple::new_vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Color::white(), Tuple::new_point(0.0, 10.0, -10.0));
        assert!(lighting(&material, &light, &position, &eyev, &normalv)
            .is_close(&Color::new(1.6363962, 1.6363962, 1.6363962)))
    }

    #[test]
    fn test_lighting_light_behind_surface() {
        let material = Material::default();
        let position = Tuple::zero_point();
        let eyev = Tuple::new_vector(0.0, 0.0, -1.0);
        let normalv = Tuple::new_vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Color::white(), Tuple::new_point(0.0, 0.0, 10.0));
        assert!(lighting(&material, &light, &position, &eyev, &normalv)
            .is_close(&Color::new(0.1, 0.1, 0.1)))
    }
}
