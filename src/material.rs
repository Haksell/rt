use crate::color::Color;

#[derive(Debug, Clone)]
pub struct Material {
    // pub pattern: Box<dyn Pattern>,
    pub color: Color, // TODO: replace with pattern
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
}

impl Material {
    pub fn new(
        // pattern: Box<dyn Pattern>,
        color: Color,
        ambient: f64,
        diffuse: f64,
        specular: f64,
        shininess: f64,
    ) -> Self {
        Self {
            // pattern,
            color,
            ambient,
            diffuse,
            specular,
            shininess,
        }
    }

    // pub fn from_color(color: Color) -> Self {
    //     Self {
    //         pattern: Box::new(Solid::new(Color::white())),
    //         ..Self::default()
    //     }
    // }
}

impl Default for Material {
    fn default() -> Self {
        Self {
            // pattern: Box::new(Solid::new(Color::white())),
            color: Color::white(),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.,
        }
    }
}
