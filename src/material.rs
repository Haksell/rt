use crate::{
    patterns::{Pattern, Solid},
    Color,
};

#[derive(Debug)]
pub struct Material {
    pub pattern: Box<dyn Pattern>,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
}

impl PartialEq for Material {
    fn eq(&self, other: &Self) -> bool {
        self.ambient == other.ambient
            && self.diffuse == other.diffuse
            && self.specular == other.specular
            && self.shininess == other.shininess
    }
}

impl Material {
    pub fn default() -> Self {
        Self {
            pattern: Box::new(Solid::new(Color::white())),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200., // too much?
        }
    }

    pub fn from_color(color: Color) -> Self {
        Self {
            pattern: Box::new(Solid::new(Color::white())),
            ..Self::default()
        }
    }
}
