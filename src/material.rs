use crate::{Color, Float};

#[derive(Debug)]
pub struct Material {
    color: Color,
    ambient: Float,
    diffuse: Float,
    specular: Float,
    shininess: Float,
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
}
