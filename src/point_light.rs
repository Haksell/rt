use crate::{Color, Tuple};

pub struct PointLight {
    pub intensity: Color,
    pub position: Tuple,
}

impl PointLight {
    pub fn new(intensity: Color, position: Tuple) -> Self {
        Self {
            intensity,
            position,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PointLight;
    use crate::{Color, Tuple};

    #[test]
    fn test_point_light_new() {
        let point_light = PointLight::new(Color::white(), Tuple::zero_point());
        assert_eq!(point_light.intensity, Color::white());
        assert_eq!(point_light.position, Tuple::zero_point());
    }
}
