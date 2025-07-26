use crate::{canvas::Canvas, matrix::Matrix, point, ray::Ray, tuple::Tuple, world::World};

pub struct Camera {
    pub hsize: usize,
    pub vsize: usize,
    pub transform: Matrix,
    pub inverse_transform: Matrix,
    pub origin: Tuple,
    pub fov: f64,
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
        let half_view = (fov * 0.5).tan();
        let (half_width, half_height) = if hsize > vsize {
            (half_view, half_view / aspect_ratio)
        } else {
            (half_view * aspect_ratio, half_view)
        };
        let pixel_size = half_width * 2.0 / hsize as f64;
        let inverse_transform = transform.inverse();
        let origin = point![
            inverse_transform[(0, 3)],
            inverse_transform[(1, 3)],
            inverse_transform[(2, 3)],
        ];

        Self {
            hsize,
            vsize,
            transform,
            inverse_transform,
            origin,
            fov,
            aspect_ratio,
            half_width,
            half_height,
            pixel_size,
        }
    }

    pub fn identity(hsize: usize, vsize: usize, fov: f64) -> Self {
        Self::new(hsize, vsize, fov, Matrix::identity())
    }

    fn ray_for_pixel(&self, px: usize, py: usize) -> Ray {
        let xoffset = (px as f64 + 0.5) * self.pixel_size;
        let yoffset = (py as f64 + 0.5) * self.pixel_size;
        let world_x = self.half_width - xoffset;
        let world_y = self.half_height - yoffset;
        let pixel = &self.inverse_transform * point![world_x, world_y, -1.];

        Ray {
            origin: self.origin,
            direction: (pixel - self.origin).normalize(),
        }
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
            color::Color,
            floats::is_close,
            transform::{rotate_y, translate, view_transform},
            vector,
            world::TESTING_WORLD,
        },
        std::f64::consts::{FRAC_1_SQRT_2, FRAC_PI_2, FRAC_PI_4},
    };

    #[test]
    fn test_camera_identity() {
        let camera = Camera::identity(160, 120, FRAC_PI_2);
        assert_eq!(camera.hsize, 160);
        assert_eq!(camera.vsize, 120);
        assert_eq!(camera.fov, FRAC_PI_2);
        assert_eq!(camera.transform, Matrix::identity());
    }

    #[test]
    fn test_camera_pixel_size() {
        assert!(is_close(
            Camera::identity(200, 125, FRAC_PI_2).pixel_size,
            0.01
        ));
        assert!(is_close(
            Camera::identity(125, 200, FRAC_PI_2).pixel_size,
            0.01
        ));
    }

    #[test]
    fn test_ray_for_pixel_center() {
        let camera = Camera::identity(201, 101, FRAC_PI_2);
        let ray = camera.ray_for_pixel(100, 50);
        assert!(ray.origin.is_close(&Tuple::zero_point()));
        assert!(ray.direction.is_close(&vector![0., 0., -1.]));
    }

    #[test]
    fn test_ray_for_pixel_corner() {
        let camera = Camera::identity(201, 101, FRAC_PI_2);
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
            FRAC_PI_2,
            rotate_y(FRAC_PI_4) * translate(0., -2., 5.),
        );
        let ray = camera.ray_for_pixel(100, 50);
        assert_eq!(ray.origin, point![0., 2., -5.]);
        assert!(
            ray.direction
                .is_close(&vector![FRAC_1_SQRT_2, 0., -FRAC_1_SQRT_2])
        );
    }

    #[test]
    fn test_render_center() {
        let camera = Camera::new(
            11,
            11,
            FRAC_PI_2,
            view_transform(&point![0., 0., -5.], &Tuple::zero_point(), &Tuple::up()),
        );
        let canvas = camera.render(&TESTING_WORLD);
        assert_eq!(canvas.height, 11);
        assert_eq!(canvas.width, 11);
        let target = Color::new(0.3806612, 0.47582647, 0.2854959);
        assert!(
            canvas[(5, 5)].is_close(&target),
            "actual={:?}\nexpected={:?}",
            canvas[(5, 5)],
            target
        );
    }
}
