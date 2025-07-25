use {
    crate::{floats::is_close, tuple::Tuple},
    core::ops::{Div, Index, Mul},
};

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
        (0..4).all(|y| (0..4).all(|x| is_close(self[y][x], rhs[y][x])))
    }

    pub fn transpose(&self) -> Self {
        Self {
            values: [
                [self[(0, 0)], self[(1, 0)], self[(2, 0)], self[(3, 0)]],
                [self[(0, 1)], self[(1, 1)], self[(2, 1)], self[(3, 1)]],
                [self[(0, 2)], self[(1, 2)], self[(2, 2)], self[(3, 2)]],
                [self[(0, 3)], self[(1, 3)], self[(2, 3)], self[(3, 3)]],
            ],
        }
    }

    // https://docs.rs/nalgebra/latest/src/nalgebra/linalg/inverse.rs.html
    pub fn inverse(&self) -> Self {
        let [[a, b, c, d], [e, f, g, h], [i, j, k, l], [m, n, o, p]] = self.values;

        let mut out = [
            [
                f * k * p - f * o * l - g * j * p + g * n * l + h * j * o - h * n * k,
                -b * k * p + b * o * l + c * j * p - c * n * l - d * j * o + d * n * k,
                b * g * p - b * o * h - c * f * p + c * n * h + d * f * o - d * n * g,
                -b * g * l + b * k * h + c * f * l - c * j * h - d * f * k + d * j * g,
            ],
            [
                -e * k * p + e * o * l + g * i * p - g * m * l - h * i * o + h * m * k,
                a * k * p - a * o * l - c * i * p + c * m * l + d * i * o - d * m * k,
                -a * g * p + a * o * h + c * e * p - c * m * h - d * e * o + d * m * g,
                a * g * l - a * k * h - c * e * l + c * i * h + d * e * k - d * i * g,
            ],
            [
                e * j * p - e * n * l - f * i * p + f * m * l + h * i * n - h * m * j,
                -a * j * p + a * n * l + b * i * p - b * m * l - d * i * n + d * m * j,
                a * f * p - a * n * h - b * e * p + b * m * h + d * e * n - d * m * f,
                -a * f * l + a * j * h + b * e * l - b * i * h - d * e * j + d * i * f,
            ],
            [
                -e * j * o + e * n * k + f * i * o - f * m * k - g * i * n + g * m * j,
                a * j * o - a * n * k - b * i * o + b * m * k + c * i * n - c * m * j,
                -a * f * o + a * n * g + b * e * o - b * m * g - c * e * n + c * m * f,
                a * f * k - a * j * g - b * e * k + b * i * g + c * e * j - c * i * f,
            ],
        ];

        let det = a * out[0][0] + e * out[0][1] + i * out[0][2] + m * out[0][3];
        debug_assert_ne!(det, 0.0);
        let inv_det = 1.0 / det;
        for y in 0..4 {
            for x in 0..4 {
                out[y][x] *= inv_det;
            }
        }

        Matrix::new(out)
    }
}

impl Index<usize> for Matrix {
    type Output = [f64; 4];

    fn index(&self, row: usize) -> &[f64; 4] {
        &self.values[row]
    }
}

impl Index<(usize, usize)> for Matrix {
    type Output = f64;

    fn index(&self, (y, x): (usize, usize)) -> &f64 {
        &self.values[y][x]
    }
}

macro_rules! impl_matrix_matrix {
    ($lhs:ty, $rhs:ty) => {
        impl Mul<$rhs> for $lhs {
            type Output = Matrix;

            // TODO: optimize with Strassen
            fn mul(self, rhs: $rhs) -> Matrix {
                Matrix::new([
                    [
                        self[0][0] * rhs[0][0]
                            + self[0][1] * rhs[1][0]
                            + self[0][2] * rhs[2][0]
                            + self[0][3] * rhs[3][0],
                        self[0][0] * rhs[0][1]
                            + self[0][1] * rhs[1][1]
                            + self[0][2] * rhs[2][1]
                            + self[0][3] * rhs[3][1],
                        self[0][0] * rhs[0][2]
                            + self[0][1] * rhs[1][2]
                            + self[0][2] * rhs[2][2]
                            + self[0][3] * rhs[3][2],
                        self[0][0] * rhs[0][3]
                            + self[0][1] * rhs[1][3]
                            + self[0][2] * rhs[2][3]
                            + self[0][3] * rhs[3][3],
                    ],
                    [
                        self[1][0] * rhs[0][0]
                            + self[1][1] * rhs[1][0]
                            + self[1][2] * rhs[2][0]
                            + self[1][3] * rhs[3][0],
                        self[1][0] * rhs[0][1]
                            + self[1][1] * rhs[1][1]
                            + self[1][2] * rhs[2][1]
                            + self[1][3] * rhs[3][1],
                        self[1][0] * rhs[0][2]
                            + self[1][1] * rhs[1][2]
                            + self[1][2] * rhs[2][2]
                            + self[1][3] * rhs[3][2],
                        self[1][0] * rhs[0][3]
                            + self[1][1] * rhs[1][3]
                            + self[1][2] * rhs[2][3]
                            + self[1][3] * rhs[3][3],
                    ],
                    [
                        self[2][0] * rhs[0][0]
                            + self[2][1] * rhs[1][0]
                            + self[2][2] * rhs[2][0]
                            + self[2][3] * rhs[3][0],
                        self[2][0] * rhs[0][1]
                            + self[2][1] * rhs[1][1]
                            + self[2][2] * rhs[2][1]
                            + self[2][3] * rhs[3][1],
                        self[2][0] * rhs[0][2]
                            + self[2][1] * rhs[1][2]
                            + self[2][2] * rhs[2][2]
                            + self[2][3] * rhs[3][2],
                        self[2][0] * rhs[0][3]
                            + self[2][1] * rhs[1][3]
                            + self[2][2] * rhs[2][3]
                            + self[2][3] * rhs[3][3],
                    ],
                    [
                        self[3][0] * rhs[0][0]
                            + self[3][1] * rhs[1][0]
                            + self[3][2] * rhs[2][0]
                            + self[3][3] * rhs[3][0],
                        self[3][0] * rhs[0][1]
                            + self[3][1] * rhs[1][1]
                            + self[3][2] * rhs[2][1]
                            + self[3][3] * rhs[3][1],
                        self[3][0] * rhs[0][2]
                            + self[3][1] * rhs[1][2]
                            + self[3][2] * rhs[2][2]
                            + self[3][3] * rhs[3][2],
                        self[3][0] * rhs[0][3]
                            + self[3][1] * rhs[1][3]
                            + self[3][2] * rhs[2][3]
                            + self[3][3] * rhs[3][3],
                    ],
                ])
            }
        }
    };
}
impl_matrix_matrix!(Matrix, Matrix);
impl_matrix_matrix!(Matrix, &Matrix);
impl_matrix_matrix!(&Matrix, Matrix);
impl_matrix_matrix!(&Matrix, &Matrix);

impl Mul<Tuple> for Matrix {
    type Output = Tuple;

    fn mul(self, rhs: Tuple) -> Tuple {
        let [[a, b, c, d], [e, f, g, h], [i, j, k, l], [m, n, o, p]] = self.values;

        Tuple::new(
            a * rhs.x + b * rhs.y + c * rhs.z + d * rhs.w,
            e * rhs.x + f * rhs.y + g * rhs.z + h * rhs.w,
            i * rhs.x + j * rhs.y + k * rhs.z + l * rhs.w,
            m * rhs.x + n * rhs.y + o * rhs.z + p * rhs.w,
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
        self * (1. / divisor)
    }
}

#[cfg(test)]
mod tests {
    use {super::*, rand::Rng as _};

    #[test]
    fn test_new() {
        let matrix = Matrix::new([
            [1., 2., 3., 4.],
            [5.5, 6.5, 7.5, 8.5],
            [9., 10., 11., 12.],
            [13.5, 14.5, 15.5, 16.5],
        ]);
        assert_eq!(matrix[0][0], 1.);
        assert_eq!(matrix[(0, 0)], 1.);
        assert_eq!(matrix[0][1], 2.);
        assert_eq!(matrix[(0, 1)], 2.);
        assert_eq!(matrix[0][2], 3.);
        assert_eq!(matrix[(0, 2)], 3.);
        assert_eq!(matrix[0][3], 4.);
        assert_eq!(matrix[(0, 3)], 4.);
        assert_eq!(matrix[1][0], 5.5);
        assert_eq!(matrix[(1, 0)], 5.5);
        assert_eq!(matrix[1][1], 6.5);
        assert_eq!(matrix[(1, 1)], 6.5);
        assert_eq!(matrix[1][2], 7.5);
        assert_eq!(matrix[(1, 2)], 7.5);
        assert_eq!(matrix[1][3], 8.5);
        assert_eq!(matrix[(1, 3)], 8.5);
        assert_eq!(matrix[2][0], 9.);
        assert_eq!(matrix[(2, 0)], 9.);
        assert_eq!(matrix[2][1], 10.);
        assert_eq!(matrix[(2, 1)], 10.);
        assert_eq!(matrix[2][2], 11.);
        assert_eq!(matrix[(2, 2)], 11.);
        assert_eq!(matrix[2][3], 12.);
        assert_eq!(matrix[(2, 3)], 12.);
        assert_eq!(matrix[3][0], 13.5);
        assert_eq!(matrix[(3, 0)], 13.5);
        assert_eq!(matrix[3][1], 14.5);
        assert_eq!(matrix[(3, 1)], 14.5);
        assert_eq!(matrix[3][2], 15.5);
        assert_eq!(matrix[(3, 2)], 15.5);
        assert_eq!(matrix[3][3], 16.5);
        assert_eq!(matrix[(3, 3)], 16.5);
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
        assert_eq!(
            Matrix::new([
                [1., 2., 3., 4.],
                [5., 6., 7., 8.],
                [9., 8., 7., 6.],
                [5., 4., 3., 2.],
            ]) * Matrix::new([
                [-2., 1., 2., 3.],
                [3., 2., 1., -1.],
                [4., 3., 6., 5.],
                [1., 2., 7., 8.]
            ]),
            Matrix::new([
                [20., 22., 50., 48.],
                [44., 54., 114., 108.],
                [40., 58., 110., 102.],
                [16., 26., 46., 42.],
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
        assert!(
            Matrix::new([
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
            )
        );
        assert!(
            Matrix::new([
                [-0.5, -4., -0.5, -1.25],
                [-2.75, 0.5, -4.75, -4.25],
                [5., -0.75, -4., 0.25],
                [4.5, 3.75, 4.5, 3.75],
            ])
            .inverse()
            .is_close(&Matrix::new([
                [0.26050284, 0.16642336, 0.07412814, 0.27050554],
                [-0.15247364, 0.11678832, -0.02595296, 0.08326575],
                [0.32019465, 0.15474453, -0.14549878, 0.29180860],
                [-0.54436334, -0.50218978, 0.11159773, -0.49137605],
            ]))
        );
        let a = Matrix::new([
            [1., 2., 3., 4.],
            [5., 6., 7., 8.],
            [9., -8., 7.25, 6.],
            [0., 42., 3.5, -2.],
        ]);
        assert!((&a * a.inverse()).is_close(&Matrix::identity()));
        let b = Matrix::new([
            [-2., 1., 2., 3.],
            [3., 2., 1., -1.],
            [4., 3., 6., 5.],
            [1., 2., 7., 8.],
        ]);
        assert!((&a * &b * b.inverse()).is_close(&a));
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
        assert!((&mat * &inv).is_close(&Matrix::identity()));
        assert!((&inv * &mat).is_close(&Matrix::identity()));
        assert!(inv.transpose().is_close(&mat.transpose().inverse()));
    }

    #[test]
    fn test_random_matrix_inverses() {
        let mut rng = rand::rng();

        for _ in 0..1000 {
            let mat_data: [[f64; 4]; 4] = [
                [
                    rng.random_range(-10.0..10.0),
                    rng.random_range(-10.0..10.0),
                    rng.random_range(-10.0..10.0),
                    rng.random_range(-10.0..10.0),
                ],
                [
                    rng.random_range(-10.0..10.0),
                    rng.random_range(-10.0..10.0),
                    rng.random_range(-10.0..10.0),
                    rng.random_range(-10.0..10.0),
                ],
                [
                    rng.random_range(-10.0..10.0),
                    rng.random_range(-10.0..10.0),
                    rng.random_range(-10.0..10.0),
                    rng.random_range(-10.0..10.0),
                ],
                [
                    rng.random_range(-10.0..10.0),
                    rng.random_range(-10.0..10.0),
                    rng.random_range(-10.0..10.0),
                    rng.random_range(-10.0..10.0),
                ],
            ];

            let mat = Matrix::new(mat_data);

            let inv = mat.inverse();
            let identity = Matrix::identity();
            let mat_mul_inv = &mat * &inv;
            assert!(mat_mul_inv.is_close(&identity),);

            let inv_mul_mat = &inv * &mat;
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
