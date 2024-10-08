use crate::{is_close, Tuple};
use std::ops::{Div, Index, Mul};

#[derive(Debug, PartialEq, Clone)]
pub struct Matrix<const N: usize> {
    values: [[f32; N]; N], // TODO: [f32; N * N]?
}

impl<const N: usize> Matrix<N> {
    pub fn new(values: [[f32; N]; N]) -> Self {
        Self { values }
    }

    pub fn zero() -> Self {
        Self {
            values: [[0.; N]; N],
        }
    }

    pub fn identity() -> Self {
        let mut values = [[0.; N]; N];
        for i in 0..N {
            values[i][i] = 1.;
        }
        Self { values }
    }

    pub fn is_close(&self, rhs: &Self) -> bool {
        (0..N).all(|y| (0..N).all(|x| is_close(self.values[y][x], rhs.values[y][x])))
    }

    pub fn transpose(&self) -> Self {
        let mut values = [[0.; N]; N];
        for y in 0..N {
            for x in 0..N {
                values[y][x] = self[x][y];
            }
        }
        Self { values }
    }

    pub fn inverse(&self) -> Self {
        // TODO: implement only for Matrix<4> so we can use arrays instead of vectors
        let mut augmented_matrix = vec![vec![0.; 2 * N]; N];
        for y in 0..N {
            for x in 0..N {
                augmented_matrix[y][x] = self.values[y][x];
            }
            augmented_matrix[y][N + y] = 1.;
        }

        for y in 0..N {
            if augmented_matrix[y][y] == 0. {
                let y_swap = (y + 1..N)
                    .find(|&y2| augmented_matrix[y2][y] != 0.)
                    .expect("matrix is singular and cannot be inverted");
                if y != y_swap {
                    augmented_matrix.swap(y, y_swap);
                }
            }

            let scalar = 1. / augmented_matrix[y][y];
            for x in y..2 * N {
                augmented_matrix[y][x] *= scalar;
            }

            for y_other in 0..N {
                if y_other != y {
                    let factor = augmented_matrix[y_other][y];
                    for x in y..2 * N {
                        augmented_matrix[y_other][x] -= factor * augmented_matrix[y][x];
                    }
                }
            }
        }

        let mut inverse_values = [[0.; N]; N];
        for y in 0..N {
            for x in 0..N {
                inverse_values[y][x] = augmented_matrix[y][N + x];
            }
        }

        Self {
            values: inverse_values,
        }
    }
}

impl<const N: usize> Index<usize> for Matrix<N> {
    type Output = [f32; N];

    fn index(&self, row: usize) -> &[f32; N] {
        &self.values[row]
    }
}

impl Mul<Matrix<4>> for Matrix<4> {
    type Output = Self;

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

impl Mul<Tuple> for Matrix<4> {
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

impl<const N: usize> Mul<f32> for Matrix<N> {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self {
        let mut values = self.values.clone();
        for y in 0..N {
            for x in 0..N {
                values[y][x] *= scalar
            }
        }
        Self { values }
    }
}

impl<const N: usize> Div<f32> for Matrix<N> {
    type Output = Self;

    fn div(self, divisor: f32) -> Self {
        let scalar = 1. / divisor;
        let mut values = self.values.clone();
        for y in 0..N {
            for x in 0..N {
                values[y][x] *= scalar
            }
        }
        Self { values }
    }
}

#[cfg(test)]
mod tests {
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
        assert_eq!(Matrix::<2>::zero(), Matrix::new([[0., 0.], [0., 0.]]));
    }

    #[test]
    fn test_identity() {
        assert_eq!(
            Matrix::<3>::identity(),
            Matrix::new([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]])
        );
    }

    #[test]
    fn test_scaling() {
        assert_eq!(
            Matrix::<3>::identity() * 4.2,
            Matrix::new([[4.2, 0., 0.], [0., 4.2, 0.], [0., 0., 4.2]])
        );
        assert_eq!(
            Matrix::<3>::identity() / 2.,
            Matrix::new([[0.5, 0., 0.], [0., 0.5, 0.], [0., 0., 0.5]])
        );
    }

    #[test]
    fn test_mat_mul() {
        assert_eq!(
            Matrix::<4>::zero() * Matrix::<4>::zero(),
            Matrix::<4>::zero()
        );
        assert_eq!(
            Matrix::<4>::zero() * Matrix::<4>::identity(),
            Matrix::<4>::zero()
        );
        assert_eq!(
            Matrix::<4>::identity() * Matrix::<4>::zero(),
            Matrix::<4>::zero()
        );
        assert_eq!(
            Matrix::<4>::identity() * Matrix::<4>::identity(),
            Matrix::<4>::identity()
        );
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
            Matrix::<4>::identity() * Tuple::new(1., 2., 3., 4.),
            Tuple::new(1., 2., 3., 4.)
        );
        assert_eq!(
            Matrix::<4>::identity() * 2. * Tuple::new(1., 2., 3., 4.),
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
        assert_eq!(Matrix::<6>::zero().transpose(), Matrix::<6>::zero());
        assert_eq!(Matrix::<7>::identity().transpose(), Matrix::<7>::identity());
        assert_eq!(
            Matrix::new([[1., 2.], [3., 4.]]).transpose(),
            Matrix::new([[1., 3.], [2., 4.]])
        );
    }

    #[test]
    fn test_inverse2() {
        assert_eq!(Matrix::<2>::identity().inverse(), Matrix::<2>::identity());
        assert!(Matrix::new([[1., 2.], [3., 4.]])
            .inverse()
            .is_close(&Matrix::new([[-2., 1.], [1.5, -0.5]])));
    }

    #[test]
    fn test_inverse3() {
        assert_eq!(Matrix::<3>::identity().inverse(), Matrix::<3>::identity());
        assert!(Matrix::new([[1., 2., 3.], [4., 0., 6.], [7., 8., 9.]])
            .inverse()
            .is_close(&(Matrix::new([[-24., 3., 6.], [3., -6., 3.], [16., 3., -4.]]) / 30.)));
        assert_eq!(
            Matrix::new([[1., 2., 3.], [3., -2., 1.], [2., 1., 3.5]]).inverse(),
            Matrix::new([[2., 1., -2.], [2.125, 0.625, -2.], [-1.75, -0.75, 2.]])
        );
    }

    #[test]
    fn test_inverse4() {
        assert_eq!(Matrix::<4>::identity().inverse(), Matrix::<4>::identity());
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
        assert!((mat.clone() * inv.clone()).is_close(&Matrix::<4>::identity()));
        assert!((inv.clone() * mat.clone()).is_close(&Matrix::<4>::identity()));
        assert!(inv.transpose().is_close(&mat.transpose().inverse()));
    }

    #[test]
    #[should_panic]
    fn test_inverse_zero() {
        Matrix::<4>::zero().inverse();
    }

    #[test]
    #[should_panic]
    fn test_inverse_singular() {
        Matrix::new([[1., 2., 3.], [4., 5., 6.], [7., 8., 9.]]).inverse();
    }
}
