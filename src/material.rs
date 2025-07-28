use crate::{
    color::Color,
    patterns::{Pattern, Solid},
};

#[derive(Debug)]
pub struct Material {
    pub pattern: Box<dyn Pattern>,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
    pub reflectivity: f64,
    pub transparency: f64,
    pub refractive_index: f64,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            pattern: Box::new(Solid::new(Color::white())),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.,
            reflectivity: 0.0,
            transparency: 0.0,
            refractive_index: 1.0,
        }
    }
}
