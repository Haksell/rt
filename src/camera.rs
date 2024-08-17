use crate::{matrix::Matrix, Ray, Tuple};

pub struct Camera {
    pub hsize: usize,
    pub vsize: usize,
    pub fov: f32,
    pub transform: Matrix<4>,
    pub aspect_ratio: f32,
    pub half_width: f32,
    pub half_height: f32,
    pub pixel_size: f32,
}

impl Camera {
    // TODO: identity
    pub fn new(hsize: usize, vsize: usize, fov: f32) -> Self {
        Self::with_transform(hsize, vsize, fov, Matrix::identity())
    }

    // TODO: new
    pub fn with_transform(hsize: usize, vsize: usize, fov: f32, transform: Matrix<4>) -> Self {
        let aspect_ratio = hsize as f32 / vsize as f32;
        let half_view = (fov / 2.0).tan();
        let (half_width, half_height) = if hsize > vsize {
            (half_view, half_view / aspect_ratio)
        } else {
            (half_view * aspect_ratio, half_view)
        };
        let pixel_size = half_width * 2.0 / hsize as f32;
        Self {
            hsize,
            vsize,
            fov,
            transform,
            aspect_ratio,
            half_width,
            half_height,
            pixel_size,
        }
    }

    pub fn ray_for_pixel(&self, px: usize, py: usize) -> Ray {
        let xoffset = (px as f32 + 0.5) * self.pixel_size;
        let yoffset = (py as f32 + 0.5) * self.pixel_size;
        let world_x = self.half_width - xoffset;
        let world_y = self.half_height - yoffset;
        let inverse_transform = self.transform.inverse(); // TODO: in Camera directly?
        let pixel = inverse_transform.clone() * Tuple::new_point(world_x, world_y, -1.);
        let origin = inverse_transform * Tuple::zero_point(); // TODO: in Camera directly
        let direction = (pixel - origin.clone()).normalize();
        Ray { origin, direction }
    }
}

#[cfg(test)]
mod tests {
    use super::Camera;
    use crate::{
        matrix::Matrix,
        transform::{rotate_y, translate},
        Tuple,
    };

    #[test]
    fn test_camera_new() {
        let camera = Camera::new(160, 120, std::f32::consts::FRAC_PI_2);
        assert_eq!(camera.hsize, 160);
        assert_eq!(camera.vsize, 120);
        assert_eq!(camera.fov, std::f32::consts::FRAC_PI_2);
        assert_eq!(camera.transform, Matrix::identity());
    }

    #[test]
    fn test_camera_pixel_size() {
        assert_eq!(
            Camera::new(200, 125, std::f32::consts::FRAC_PI_2).pixel_size,
            0.01
        );
        assert_eq!(
            Camera::new(125, 200, std::f32::consts::FRAC_PI_2).pixel_size,
            0.01
        );
    }

    #[test]
    fn test_ray_for_pixel_center() {
        let camera = Camera::new(201, 101, std::f32::consts::FRAC_PI_2);
        let ray = camera.ray_for_pixel(100, 50);
        assert!(ray.origin.is_close(&Tuple::zero_point()));
        assert!(ray.direction.is_close(&Tuple::new_vector(0., 0., -1.)));
    }

    #[test]
    fn test_ray_for_pixel_corner() {
        let camera = Camera::new(201, 101, std::f32::consts::FRAC_PI_2);
        let ray = camera.ray_for_pixel(0, 0);
        assert!(ray.origin.is_close(&Tuple::zero_point()));
        assert!(ray
            .direction
            .is_close(&Tuple::new_vector(0.6651864, 0.33259323, -0.66851234)));
    }

    #[test]
    fn test_ray_for_pixel_center_transformed() {
        let camera = Camera::with_transform(
            201,
            101,
            std::f32::consts::FRAC_PI_2,
            rotate_y(std::f32::consts::FRAC_PI_4) * translate(0., -2., 5.),
        );
        let ray = camera.ray_for_pixel(100, 50);
        assert!(ray.origin.is_close(&Tuple::new_point(0., 2., -5.)));
        assert!(ray.direction.is_close(&Tuple::new_vector(
            std::f32::consts::FRAC_1_SQRT_2,
            0.,
            -std::f32::consts::FRAC_1_SQRT_2
        )));
    }
}
