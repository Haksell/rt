use crate::{Float, Tuple};
use std::ops::{Index, Mul};

#[derive(Debug, PartialEq, Clone)]
pub struct Matrix<const N: usize> {
    values: [[Float; N]; N], // TODO: [Float; N * N]?
}

impl<const N: usize> Matrix<N> {
    pub fn new(values: &[[Float; N]; N]) -> Self {
        // This dereference seems very sus
        Self { values: *values }
    }

    pub fn zero() -> Self {
        Self {
            values: [[0.0; N]; N],
        }
    }

    pub fn identity() -> Self {
        let mut values = [[0.0; N]; N];
        for i in 0..N {
            values[i][i] = 1.0;
        }
        Self { values }
    }

    pub fn transpose(&self) -> Self {
        let mut values = [[0.0; N]; N];
        for y in 0..N {
            for x in 0..N {
                values[y][x] = self[x][y];
            }
        }
        Self { values }
    }
}

impl<const N: usize> Index<usize> for Matrix<N> {
    type Output = [Float; N];

    fn index(&self, row: usize) -> &[Float; N] {
        &self.values[row]
    }
}

impl Mul<Matrix<4>> for Matrix<4> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let mut values = [[0.0; 4]; 4];
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

impl<const N: usize> Mul<Float> for Matrix<N> {
    type Output = Self;

    fn mul(self, scalar: Float) -> Self {
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
        let matrix = Matrix::new(&[
            [1.0, 2.0, 3.0, 4.0],
            [5.5, 6.5, 7.5, 8.5],
            [9.0, 10.0, 11.0, 12.0],
            [13.5, 14.5, 15.5, 16.5],
        ]);
        assert_eq!(matrix[0][0], 1.0);
        assert_eq!(matrix[0][1], 2.0);
        assert_eq!(matrix[0][2], 3.0);
        assert_eq!(matrix[0][3], 4.0);
        assert_eq!(matrix[1][0], 5.5);
        assert_eq!(matrix[1][1], 6.5);
        assert_eq!(matrix[1][2], 7.5);
        assert_eq!(matrix[1][3], 8.5);
        assert_eq!(matrix[2][0], 9.0);
        assert_eq!(matrix[2][1], 10.0);
        assert_eq!(matrix[2][2], 11.0);
        assert_eq!(matrix[2][3], 12.0);
        assert_eq!(matrix[3][0], 13.5);
        assert_eq!(matrix[3][1], 14.5);
        assert_eq!(matrix[3][2], 15.5);
        assert_eq!(matrix[3][3], 16.5);
    }

    #[test]
    fn test_zero() {
        assert_eq!(Matrix::<2>::zero(), Matrix::new(&[[0.0, 0.0], [0.0, 0.0]]));
    }

    #[test]
    fn test_identity() {
        assert_eq!(
            Matrix::<3>::identity(),
            Matrix::new(&[[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]])
        );
    }

    #[test]
    fn test_scaling() {
        assert_eq!(
            Matrix::<3>::identity() * 4.2,
            Matrix::new(&[[4.2, 0.0, 0.0], [0.0, 4.2, 0.0], [0.0, 0.0, 4.2]])
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
            Matrix::new(&[
                [2.0, 0.0, 0.0, 0.0],
                [0.0, 2.0, 0.0, 0.0],
                [0.0, 0.0, 2.0, 0.0],
                [0.0, 0.0, 0.0, 2.0],
            ]) * Matrix::new(&[
                [0.0, 1.0, 2.0, 3.0],
                [4.0, 5.0, 6.0, 7.0],
                [8.0, 9.0, 10.0, 11.0],
                [12.0, 13.0, 14.0, 15.0],
            ]),
            Matrix::new(&[
                [0.0, 2.0, 4.0, 6.0],
                [8.0, 10.0, 12.0, 14.0],
                [16.0, 18.0, 20.0, 22.0],
                [24.0, 26.0, 28.0, 30.0],
            ])
        );
        assert_eq!(
            Matrix::new(&[
                [0.0, 1.0, 0.0, 0.0],
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ]) * Matrix::new(&[
                [0.0, 1.0, 2.0, 3.0],
                [4.0, 5.0, 6.0, 7.0],
                [8.0, 9.0, 10.0, 11.0],
                [12.0, 13.0, 14.0, 15.0],
            ]),
            Matrix::new(&[
                [4.0, 5.0, 6.0, 7.0],
                [0.0, 1.0, 2.0, 3.0],
                [8.0, 9.0, 10.0, 11.0],
                [12.0, 13.0, 14.0, 15.0],
            ])
        );
    }

    #[test]
    fn test_tup_mul() {
        assert_eq!(
            Matrix::<4>::identity() * Tuple::new(1.0, 2.0, 3.0, 4.0),
            Tuple::new(1.0, 2.0, 3.0, 4.0)
        );
        assert_eq!(
            Matrix::<4>::identity() * 2.0 * Tuple::new(1.0, 2.0, 3.0, 4.0),
            Tuple::new(2.0, 4.0, 6.0, 8.0)
        );
        assert_eq!(
            Matrix::new(&[
                [0.0, 1.0, 0.0, 0.0],
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ]) * Tuple::new(1.0, 2.0, 3.0, 4.0),
            Tuple::new(2.0, 1.0, 3.0, 4.0),
        );
        assert_eq!(
            Matrix::new(&[
                [1.0, 2.0, 3.0, 4.0],
                [2.0, 4.0, 4.0, 2.0],
                [8.0, 6.0, 4.0, 1.0],
                [0.0, 0.0, 0.0, 1.0],
            ]) * Tuple::new(1.0, 2.0, 3.0, 1.0),
            Tuple::new(18.0, 24.0, 33.0, 1.0),
        );
    }

    #[test]
    fn test_transpose() {
        assert_eq!(Matrix::<6>::zero().transpose(), Matrix::<6>::zero());
        assert_eq!(Matrix::<7>::identity().transpose(), Matrix::<7>::identity());
        assert_eq!(
            Matrix::new(&[[1.0, 2.0], [3.0, 4.0]]).transpose(),
            Matrix::new(&[[1.0, 3.0], [2.0, 4.0]])
        );
    }
}
