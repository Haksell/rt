use nalgebra::SMatrix;

use crate::{is_close, Tuple};
use std::ops::{Div, Index, Mul};

#[derive(Debug, PartialEq, Clone)]
pub struct Matrix {
    values: [[f64; 4]; 4],
}

impl Matrix {
    pub fn new(values: [[f64; 4]; 4]) -> Self {
        Self { values }
    }

    pub fn zero() -> Self {
        Self {
            values: [[0.; 4]; 4],
        }
    }

    pub fn identity() -> Self {
        Self {
            values: [
                [1., 0., 0., 0.],
                [0., 1., 0., 0.],
                [0., 0., 1., 0.],
                [0., 0., 0., 1.],
            ],
        }
    }

    pub fn is_close(&self, rhs: &Self) -> bool {
        (0..4).all(|y| (0..4).all(|x| is_close(self.values[y][x], rhs.values[y][x])))
    }

    pub fn transpose(&self) -> Self {
        let mut values = [[0.; 4]; 4];
        for y in 0..4 {
            for x in 0..4 {
                values[y][x] = self[x][y];
            }
        }
        Self { values }
    }

    fn to_nalgebra(&self) -> SMatrix<f64, 4, 4> {
        SMatrix::<f64, 4, 4>::from_row_slice(&self.values.concat())
    }

    // Convert from nalgebra's SMatrix back to your Matrix struct
    fn from_nalgebra(matrix: SMatrix<f64, 4, 4>) -> Self {
        let mut values = [[0.0; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                values[i][j] = matrix[(i, j)];
            }
        }
        Matrix::new(values)
    }

    pub fn inverse(&self) -> Self {
        let nalgebra_matrix = self.to_nalgebra();
        // Compute the inverse using nalgebra's inversion method
        Matrix::from_nalgebra(nalgebra_matrix.try_inverse().unwrap())
    }
}

impl Index<usize> for Matrix {
    type Output = [f64; 4];

    fn index(&self, row: usize) -> &[f64; 4] {
        &self.values[row]
    }
}

impl Mul<Matrix> for Matrix {
    type Output = Self;

    // TODO: no loop
    fn mul(self, rhs: Self) -> Self {
        let mut values = [[0.; 4]; 4];
        for y in 0..4 {
            for x in 0..4 {
                for i in 0..4 {
                    values[y][x] += self.values[y][i] * rhs.values[i][x];
                }
            }
        }
        Self { values }
    }
}

impl Mul<Tuple> for Matrix {
    type Output = Tuple;

    fn mul(self, rhs: Tuple) -> Tuple {
        Tuple::new(
            self.values[0][0] * rhs.x
                + self.values[0][1] * rhs.y
                + self.values[0][2] * rhs.z
                + self.values[0][3] * rhs.w,
            self.values[1][0] * rhs.x
                + self.values[1][1] * rhs.y
                + self.values[1][2] * rhs.z
                + self.values[1][3] * rhs.w,
            self.values[2][0] * rhs.x
                + self.values[2][1] * rhs.y
                + self.values[2][2] * rhs.z
                + self.values[2][3] * rhs.w,
            self.values[3][0] * rhs.x
                + self.values[3][1] * rhs.y
                + self.values[3][2] * rhs.z
                + self.values[3][3] * rhs.w,
        )
    }
}

impl Mul<f64> for Matrix {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        let mut values = self.values.clone();
        for y in 0..4 {
            for x in 0..4 {
                values[y][x] *= scalar
            }
        }
        Self { values }
    }
}

impl Div<f64> for Matrix {
    type Output = Self;

    fn div(self, divisor: f64) -> Self {
        let scalar = 1. / divisor;
        let mut values = self.values.clone();
        for y in 0..4 {
            for x in 0..4 {
                values[y][x] *= scalar
            }
        }
        Self { values }
    }
}

#[cfg(test)]
mod tests {
    use rand::Rng as _;

    use super::Matrix;
    use crate::Tuple;

    #[test]
    fn test_new() {
        let matrix = Matrix::new([
            [1., 2., 3., 4.],
            [5.5, 6.5, 7.5, 8.5],
            [9., 10., 11., 12.],
            [13.5, 14.5, 15.5, 16.5],
        ]);
        assert_eq!(matrix[0][0], 1.);
        assert_eq!(matrix[0][1], 2.);
        assert_eq!(matrix[0][2], 3.);
        assert_eq!(matrix[0][3], 4.);
        assert_eq!(matrix[1][0], 5.5);
        assert_eq!(matrix[1][1], 6.5);
        assert_eq!(matrix[1][2], 7.5);
        assert_eq!(matrix[1][3], 8.5);
        assert_eq!(matrix[2][0], 9.);
        assert_eq!(matrix[2][1], 10.);
        assert_eq!(matrix[2][2], 11.);
        assert_eq!(matrix[2][3], 12.);
        assert_eq!(matrix[3][0], 13.5);
        assert_eq!(matrix[3][1], 14.5);
        assert_eq!(matrix[3][2], 15.5);
        assert_eq!(matrix[3][3], 16.5);
    }

    #[test]
    fn test_zero() {
        assert_eq!(
            Matrix::zero(),
            Matrix::new([
                [0., 0., 0., 0.],
                [0., 0., 0., 0.],
                [0., 0., 0., 0.],
                [0., 0., 0., 0.]
            ])
        );
    }

    #[test]
    fn test_identity() {
        assert_eq!(
            Matrix::identity(),
            Matrix::new([
                [1., 0., 0., 0.],
                [0., 1., 0., 0.],
                [0., 0., 1., 0.],
                [0., 0., 0., 1.]
            ])
        );
    }

    #[test]
    fn test_scaling() {
        assert_eq!(
            Matrix::identity() * 4.2,
            Matrix::new([
                [4.2, 0., 0., 0.],
                [0., 4.2, 0., 0.],
                [0., 0., 4.2, 0.],
                [0., 0., 0., 4.2]
            ])
        );
        assert_eq!(
            Matrix::identity() / 2.,
            Matrix::new([
                [0.5, 0., 0., 0.],
                [0., 0.5, 0., 0.],
                [0., 0., 0.5, 0.],
                [0., 0., 0., 0.5]
            ])
        );
    }

    #[test]
    fn test_mat_mul() {
        assert_eq!(Matrix::zero() * Matrix::zero(), Matrix::zero());
        assert_eq!(Matrix::zero() * Matrix::identity(), Matrix::zero());
        assert_eq!(Matrix::identity() * Matrix::zero(), Matrix::zero());
        assert_eq!(Matrix::identity() * Matrix::identity(), Matrix::identity());
        assert_eq!(
            Matrix::new([
                [2., 0., 0., 0.],
                [0., 2., 0., 0.],
                [0., 0., 2., 0.],
                [0., 0., 0., 2.],
            ]) * Matrix::new([
                [0., 1., 2., 3.],
                [4., 5., 6., 7.],
                [8., 9., 10., 11.],
                [12., 13., 14., 15.],
            ]),
            Matrix::new([
                [0., 2., 4., 6.],
                [8., 10., 12., 14.],
                [16., 18., 20., 22.],
                [24., 26., 28., 30.],
            ])
        );
        assert_eq!(
            Matrix::new([
                [0., 1., 0., 0.],
                [1., 0., 0., 0.],
                [0., 0., 1., 0.],
                [0., 0., 0., 1.],
            ]) * Matrix::new([
                [0., 1., 2., 3.],
                [4., 5., 6., 7.],
                [8., 9., 10., 11.],
                [12., 13., 14., 15.],
            ]),
            Matrix::new([
                [4., 5., 6., 7.],
                [0., 1., 2., 3.],
                [8., 9., 10., 11.],
                [12., 13., 14., 15.],
            ])
        );
    }

    #[test]
    fn test_tup_mul() {
        assert_eq!(
            Matrix::identity() * Tuple::new(1., 2., 3., 4.),
            Tuple::new(1., 2., 3., 4.)
        );
        assert_eq!(
            Matrix::identity() * 2. * Tuple::new(1., 2., 3., 4.),
            Tuple::new(2., 4., 6., 8.)
        );
        assert_eq!(
            Matrix::new([
                [0., 1., 0., 0.],
                [1., 0., 0., 0.],
                [0., 0., 1., 0.],
                [0., 0., 0., 1.],
            ]) * Tuple::new(1., 2., 3., 4.),
            Tuple::new(2., 1., 3., 4.),
        );
        assert_eq!(
            Matrix::new([
                [1., 2., 3., 4.],
                [2., 4., 4., 2.],
                [8., 6., 4., 1.],
                [0., 0., 0., 1.],
            ]) * Tuple::new(1., 2., 3., 1.),
            Tuple::new(18., 24., 33., 1.),
        );
    }

    #[test]
    fn test_transpose() {
        assert_eq!(Matrix::zero().transpose(), Matrix::zero());
        assert_eq!(Matrix::identity().transpose(), Matrix::identity());
        assert_eq!(
            Matrix::new([
                [1., 2., 3., 4.],
                [5., 6., 7., 8.],
                [9., 10., 11., 12.],
                [13., 14., 15., 16.],
            ])
            .transpose(),
            Matrix::new([
                [1., 5., 9., 13.],
                [2., 6., 10., 14.],
                [3., 7., 11., 15.],
                [4., 8., 12., 16.],
            ])
        );
    }

    #[test]
    fn test_inverse() {
        assert_eq!(Matrix::identity().inverse(), Matrix::identity());
        let swap = Matrix::new([
            [0., 0., 1., 0.],
            [0., 1., 0., 0.],
            [1., 0., 0., 0.],
            [0., 0., 0., 1.],
        ]);
        assert_eq!(swap.inverse(), swap);
        assert!(Matrix::new([
            [1., 2., 3., 4.],
            [12., 13., 14., 5.],
            [11., 0., 15., 6.],
            [10., 9., 8., 7.]
        ])
        .inverse()
        .is_close(
            &(Matrix::new([
                [-411., -132., 55., 282.],
                [68., 121., -110., -31.],
                [187., 154., 55., -264.],
                [286., -143., 0., 143.]
            ]) / 1430.)
        ));
        let mat = Matrix::new([
            [-0.5, -4., -0.5, -1.25],
            [-2.75, 0.5, -4.75, -4.25],
            [5., -0.75, -4., 0.25],
            [4.5, 3.75, 4.5, 3.75],
        ]);
        let inv = Matrix::new([
            [0.26050284, 0.16642336, 0.07412814, 0.27050554],
            [-0.15247364, 0.11678832, -0.02595296, 0.08326575],
            [0.32019465, 0.15474453, -0.14549878, 0.29180860],
            [-0.54436334, -0.50218978, 0.11159773, -0.49137605],
        ]);
        assert!(mat.inverse().is_close(&inv));
    }

    #[test]
    fn test_inverse_rules() {
        let mat = Matrix::new([
            [-0.5, -4., -0.5, -1.25],
            [-2.75, 0.5, -4.75, -4.25],
            [5., -0.75, -4., 0.25],
            [4.5, 3.75, 4.5, 3.75],
        ]);
        let inv = mat.inverse();
        assert!((mat.clone() * inv.clone()).is_close(&Matrix::identity()));
        assert!((inv.clone() * mat.clone()).is_close(&Matrix::identity()));
        assert!(inv.transpose().is_close(&mat.transpose().inverse()));
    }

    #[test]
    fn test_random_matrix_inverses() {
        let mut rng = rand::thread_rng();

        for _ in 0..1000 {
            let mat_data: [[f64; 4]; 4] = [
                [
                    rng.gen_range(-10.0..10.0),
                    rng.gen_range(-10.0..10.0),
                    rng.gen_range(-10.0..10.0),
                    rng.gen_range(-10.0..10.0),
                ],
                [
                    rng.gen_range(-10.0..10.0),
                    rng.gen_range(-10.0..10.0),
                    rng.gen_range(-10.0..10.0),
                    rng.gen_range(-10.0..10.0),
                ],
                [
                    rng.gen_range(-10.0..10.0),
                    rng.gen_range(-10.0..10.0),
                    rng.gen_range(-10.0..10.0),
                    rng.gen_range(-10.0..10.0),
                ],
                [
                    rng.gen_range(-10.0..10.0),
                    rng.gen_range(-10.0..10.0),
                    rng.gen_range(-10.0..10.0),
                    rng.gen_range(-10.0..10.0),
                ],
            ];

            let mat = Matrix::new(mat_data);

            let inv = mat.inverse();
            let identity = Matrix::identity();
            let mat_mul_inv = mat.clone() * inv.clone();
            assert!(mat_mul_inv.is_close(&identity),);

            let inv_mul_mat = inv.clone() * mat.clone();
            assert!(inv_mul_mat.is_close(&identity),);

            let inv_transpose = inv.transpose();
            let mat_transpose_inv = mat.transpose().inverse();
            assert!(inv_transpose.is_close(&mat_transpose_inv));
        }
    }

    #[test]
    #[should_panic]
    fn test_inverse_zero() {
        Matrix::zero().inverse();
    }

    #[test]
    #[should_panic]
    fn test_inverse_singular() {
        Matrix::new([
            [1., 2., 3., 4.],
            [5., 6., 7., 8.],
            [9., 10., 11., 12.],
            [15., 18., 21., 24.],
        ])
        .inverse();
    }
}
