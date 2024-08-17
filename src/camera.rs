use crate::matrix::Matrix;

pub struct Camera {
    hsize: usize,
    vsize: usize,
    fov: f32, // TODO: hfov or vfov?
    transform: Matrix<4>,
}

impl Camera {
    fn new(hsize: usize, vsize: usize, fov: f32) -> Self {
        Self {
            hsize,
            vsize,
            fov,
            transform: Matrix::identity(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Camera;

    #[test]
    fn test_camera_new() {
        let camera = Camera::new(160, 120, std::f32::consts::FRAC_PI_2);
    }
}
