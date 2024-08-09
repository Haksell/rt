use crate::Float;
use std::ops::Index;

pub struct Matrix<const N: usize> {
    values: [[Float; N]; N], // TODO: [Float; N * N]?
}

impl<const N: usize> Matrix<N> {
    pub fn new(values: &[[Float; N]; N]) -> Self {
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
}

impl<const N: usize> Index<usize> for Matrix<N> {
    type Output = [Float; N];

    fn index(&self, row: usize) -> &[Float; N] {
        &self.values[row]
    }
}

#[cfg(test)]
mod tests {
    use super::Matrix;

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
        let matrix = Matrix::<2>::zero();
        assert_eq!(matrix[0][0], 0.0);
        assert_eq!(matrix[0][1], 0.0);
        assert_eq!(matrix[1][0], 0.0);
        assert_eq!(matrix[1][1], 0.0);
    }

    #[test]
    fn test_identity() {
        let matrix = Matrix::<3>::identity();
        assert_eq!(matrix[0][0], 1.0);
        assert_eq!(matrix[0][1], 0.0);
        assert_eq!(matrix[0][2], 0.0);
        assert_eq!(matrix[1][0], 0.0);
        assert_eq!(matrix[1][1], 1.0);
        assert_eq!(matrix[1][2], 0.0);
        assert_eq!(matrix[2][0], 0.0);
        assert_eq!(matrix[2][1], 0.0);
        assert_eq!(matrix[2][2], 1.0);
    }
}
