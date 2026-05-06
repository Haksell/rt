use rand::{RngExt as _, rng};

use crate::vec3::Vec3;

pub struct Camera {
    origin: Vec3,
    aspect_ratio: f32,
    window_width: f32,
    window_height: f32,
    viewport_width: f32,
    viewport_height: f32,
    focal_length: f32,
}

impl Camera {
    pub fn new(window_width: usize, window_height: usize) -> Self {
        let aspect_ratio = window_width as f32 / window_height as f32;
        let viewport_height = 2.0;
        Self {
            origin: Vec3::zero(),
            aspect_ratio,
            window_width: window_width as f32,
            window_height: window_height as f32,
            viewport_height,
            viewport_width: viewport_height * aspect_ratio,
            focal_length: 1.0,
        }
    }

    pub fn get_ray_direction(&self, x: usize, y: usize) -> Vec3 {
        let mut rng = rng();
        let x_sample = x as f32 + rng.random::<f32>();
        let y_sample = y as f32 + rng.random::<f32>();
        let viewport_x = self.viewport_width * (x_sample / self.window_width - 0.5);
        let viewport_y = self.viewport_height * (0.5 - y_sample / self.window_height);
        let viewport_z = -self.focal_length;
        Vec3::new(viewport_x, viewport_y, viewport_z)
    }
}
