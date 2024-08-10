use crate::{Color, Float};

#[derive(Debug, PartialEq)]
pub struct Material {
    pub color: Color,
    pub ambient: Float,
    pub diffuse: Float,
    pub specular: Float,
    pub shininess: Float,
}

impl Material {
    pub fn default() -> Self {
        Self {
            color: Color::white(),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0, // too much?
        }
    }

    pub fn from_color(color: Color) -> Self {
        Self {
            color,
            ..Self::default()
        }
    }
}
