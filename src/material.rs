use crate::Color;

#[derive(Debug, PartialEq)]
pub struct Material {
    pub color: Color,
    pub ambient: f32,
    pub diffuse: f32,
    pub specular: f32,
    pub shininess: f32,
}

impl Material {
    pub fn default() -> Self {
        Self {
            color: Color::white(),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200., // too much?
        }
    }

    pub fn from_color(color: Color) -> Self {
        Self {
            color,
            ..Self::default()
        }
    }
}
