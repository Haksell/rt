use crate::matrix::Matrix;

pub struct Camera {
    hsize: usize,
    vsize: usize,
    fov: f32, // TODO: hfov or vfov?
    transform: Matrix<4>,
    aspect_ratio: f32, // TODO: remove?
    pixel_size: f32,
}

impl Camera {
    fn new(hsize: usize, vsize: usize, fov: f32) -> Self {
        let aspect_ratio = hsize as f32 / vsize as f32;
        let half_view = (fov / 2.0).tan();
        let (half_width, half_height) = if hsize > vsize {
            (half_view, half_view / aspect_ratio)
        } else {
            (half_view * aspect_ratio, half_view)
        };
        Self {
            hsize,
            vsize,
            fov,
            transform: Matrix::identity(),
            aspect_ratio,
            pixel_size: half_width * 2.0 / hsize as f32, // TODO
        }
    }

    // fn ray_for_pixel(&self, x: usize, y: usize) {}
}

#[cfg(test)]
mod tests {
    use super::Camera;
    use crate::matrix::Matrix;

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
}
