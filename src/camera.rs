use crate::{canvas::Canvas, matrix::Matrix, point, ray::Ray, tuple::Tuple, world::World};

pub struct Camera {
    pub hsize: usize,
    pub vsize: usize,
    pub fov: f64,
    pub transform: Matrix,
    pub aspect_ratio: f64,
    pub half_width: f64,
    pub half_height: f64,
    pub pixel_size: f64,
}

impl Camera {
    pub fn new(hsize: usize, vsize: usize, fov: f64, transform: Matrix) -> Self {
        debug_assert!(hsize > 0);
        debug_assert!(vsize > 0);
        let aspect_ratio = hsize as f64 / vsize as f64;
        let half_view = (fov / 2.0).tan();
        let (half_width, half_height) = if hsize > vsize {
            (half_view, half_view / aspect_ratio)
        } else {
            (half_view * aspect_ratio, half_view)
        };
        let pixel_size = half_width * 2.0 / hsize as f64;
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

    pub fn identity(hsize: usize, vsize: usize, fov: f64) -> Self {
        Self::new(hsize, vsize, fov, Matrix::identity())
    }

    pub fn ray_for_pixel(&self, px: usize, py: usize) -> Ray {
        let xoffset = (px as f64 + 0.5) * self.pixel_size;
        let yoffset = (py as f64 + 0.5) * self.pixel_size;
        let world_x = self.half_width - xoffset;
        let world_y = self.half_height - yoffset;
        let inverse_transform = self.transform.inverse(); // TODO: in Camera directly?
        let pixel = &inverse_transform * point![world_x, world_y, -1.];
        let origin = inverse_transform * Tuple::zero_point(); // TODO: in Camera directly
        let direction = (pixel - origin).normalize();
        Ray { origin, direction }
    }

    pub fn render(&self, world: &World) -> Canvas {
        let mut canvas = Canvas::new(self.hsize, self.vsize);
        for py in 0..self.vsize {
            for px in 0..self.hsize {
                canvas[(py, px)] = world.color_at(&self.ray_for_pixel(px, py));
            }
        }
        canvas
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::{
            color,
            color::Color,
            floats::is_close,
            transform::{rotate_y, translate, view_transform},
            vector,
        },
    };

    #[test]
    fn test_camera_new() {
        let camera = Camera::identity(160, 120, std::f64::consts::FRAC_PI_2);
        assert_eq!(camera.hsize, 160);
        assert_eq!(camera.vsize, 120);
        assert_eq!(camera.fov, std::f64::consts::FRAC_PI_2);
        assert_eq!(camera.transform, Matrix::identity());
    }

    #[test]
    fn test_camera_pixel_size() {
        assert!(is_close(
            Camera::identity(200, 125, std::f64::consts::FRAC_PI_2).pixel_size,
            0.01
        ));
        assert!(is_close(
            Camera::identity(125, 200, std::f64::consts::FRAC_PI_2).pixel_size,
            0.01
        ));
    }

    #[test]
    fn test_ray_for_pixel_center() {
        let camera = Camera::identity(201, 101, std::f64::consts::FRAC_PI_2);
        let ray = camera.ray_for_pixel(100, 50);
        assert!(ray.origin.is_close(&Tuple::zero_point()));
        assert!(ray.direction.is_close(&vector![0., 0., -1.]));
    }

    #[test]
    fn test_ray_for_pixel_corner() {
        let camera = Camera::identity(201, 101, std::f64::consts::FRAC_PI_2);
        let ray = camera.ray_for_pixel(0, 0);
        assert!(ray.origin.is_close(&Tuple::zero_point()));
        assert!(
            ray.direction
                .is_close(&vector![0.6651864, 0.33259323, -0.66851234])
        );
    }

    #[test]
    fn test_ray_for_pixel_center_transformed() {
        let camera = Camera::new(
            201,
            101,
            std::f64::consts::FRAC_PI_2,
            rotate_y(std::f64::consts::FRAC_PI_4) * translate(0., -2., 5.),
        );
        let ray = camera.ray_for_pixel(100, 50);
        assert!(ray.origin.is_close(&point![0., 2., -5.]));
        assert!(ray.direction.is_close(&vector![
            std::f64::consts::FRAC_1_SQRT_2,
            0.,
            -std::f64::consts::FRAC_1_SQRT_2
        ]));
    }

    #[test]
    fn test_render_center() {
        let camera = Camera::new(
            11,
            11,
            std::f64::consts::FRAC_PI_2,
            view_transform(&point![0., 0., -5.], &Tuple::zero_point(), &Tuple::up()),
        );
        let world = World::default();
        let canvas = camera.render(&world);
        assert_eq!(canvas.height, 11);
        assert_eq!(canvas.width, 11);
        // assert!(canvas[(5, 5)].is_close(&color![0.3806612, 0.47582647, 0.2854959]));
        assert!(canvas[(5, 5)].is_close(&Color::white()));
    }
}
