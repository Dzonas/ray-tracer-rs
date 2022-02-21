use std::ops::Mul;

use crate::tuple::Tuple4;

#[derive(Debug, PartialEq, Clone)]
pub struct Matrix {
    width: usize,
    height: usize,
    data: Vec<f64>,
}

impl Matrix {
    const PRECISION: f64 = 1e-12;

    pub fn new<T: Copy + Into<f64>>(width: usize, height: usize, data: &[T]) -> Self {
        if width * height == 0 {
            panic!("Can't construct empty matrix");
        }
        if width * height != data.len() {
            panic!("Matrix dimensions don't match given element count");
        }

        let data = data.iter().map(|&n| n.into()).collect();

        Matrix {
            width,
            height,
            data,
        }
    }

    pub fn as_1x1<T: Copy + Into<f64>>(data: &[T]) -> Self {
        Matrix::new(1, 1, data)
    }

    pub fn as_2x2<T: Copy + Into<f64>>(data: &[T]) -> Self {
        Matrix::new(2, 2, data)
    }

    pub fn as_3x3<T: Copy + Into<f64>>(data: &[T]) -> Self {
        Matrix::new(3, 3, data)
    }

    pub fn as_4x4<T: Copy + Into<f64>>(data: &[T]) -> Self {
        Matrix::new(4, 4, data)
    }

    pub fn zero(width: usize, height: usize) -> Self {
        Matrix::new(width, height, &vec![0.0; width * height])
    }

    pub fn identity(n: usize) -> Self {
        let mut matrix = Matrix::zero(n, n);
        for i in 0..n {
            matrix.data[i * (n + 1)] = 1.0;
        }

        matrix
    }

    pub fn get(&self, y: usize, x: usize) -> f64 {
        let i = self.to_index(y, x);
        self.data[i]
    }

    pub fn transpose(self) -> Self {
        let mut new_data = vec![0.0; self.width * self.height];

        for (old_i, n) in self.data.iter().enumerate() {
            let (y, x) = self.to_yx(old_i);
            let new_i = self.to_index(x, y);
            new_data[new_i] = *n;
        }

        Matrix {
            width: self.width,
            height: self.height,
            data: new_data,
        }
    }

    pub fn det(&self) -> f64 {
        if !self.is_square() {
            panic!("Determinant is only defined for square matrices.")
        }

        let size = self.width;
        let mut det = 0.0;

        if size == 1 {
            det = self.data[0];
        } else if size == 2 {
            det = self.data[0] * self.data[3] - self.data[1] * self.data[2];
        } else {
            for i in 0..size {
                det += self.data[i] * self.cofactor(0, i);
            }
        }

        det
    }

    pub fn is_invertible(&self) -> bool {
        self.is_invertible_with_det().0
    }

    pub fn inverse(self) -> Self {
        if !self.is_square() {
            panic!("Inverse is only defined for square matrices.");
        }

        let (is_invertible, det) = self.is_invertible_with_det();

        if !is_invertible {
            panic!("Matrix is not invertible");
        }

        let size = self.width;
        let mut matrix = Matrix::zero(size, size);
        for y in 0..size {
            for x in 0..size {
                let c = self.cofactor(y, x);
                let i = self.to_index(x, y);
                matrix.data[i] = c / det;
            }
        }

        matrix
    }

    fn is_invertible_with_det(&self) -> (bool, f64) {
        let det = self.det();
        (det.abs() >= Self::PRECISION, det)
    }

    fn submatrix(&self, row: usize, col: usize) -> Self {
        let width = self.width - 1;
        let height = self.height - 1;
        let data = self
            .data
            .iter()
            .enumerate()
            .map(|(i, n)| (self.to_yx(i), n))
            .filter(|&((y, x), _)| y != row && x != col)
            .map(|(_, &n)| n)
            .collect();

        Matrix {
            width,
            height,
            data,
        }
    }

    fn minor(&self, row: usize, col: usize) -> f64 {
        self.submatrix(row, col).det()
    }

    fn cofactor(&self, row: usize, col: usize) -> f64 {
        let n = if (row + col) % 2 == 1 { -1.0 } else { 1.0 };
        n * self.minor(row, col)
    }

    fn is_square(&self) -> bool {
        self.width == self.height
    }

    fn to_index(&self, y: usize, x: usize) -> usize {
        y * self.width + x
    }

    fn to_yx(&self, i: usize) -> (usize, usize) {
        let y = i / self.width;
        let x = i % self.width;

        (y, x)
    }
}

impl Mul<Matrix> for Matrix {
    type Output = Self;

    fn mul(self, rhs: Matrix) -> Self::Output {
        if self.width != rhs.height {
            panic!("Dimensions of matrices don't match");
        }

        let new_width = rhs.width;
        let new_height = self.height;

        let mut new_data = Vec::with_capacity(new_width * new_height);
        for y in 0..self.width {
            for x in 0..self.width {
                let n: f64 = (0..self.width)
                    .map(|n| self.get(y, n) * rhs.get(n, x))
                    .sum();
                new_data.push(n);
            }
        }

        Matrix {
            width: new_width,
            height: new_height,
            data: new_data,
        }
    }
}

impl Mul<Tuple4> for Matrix {
    type Output = Tuple4;

    fn mul(self, rhs: Tuple4) -> Self::Output {
        let k = 4;
        if self.width != k || self.height != k {
            panic!("Only 4x4 matrices are supported for tuple multiplication");
        }
        let mut data = Vec::with_capacity(k);

        for y in 0..k {
            let start = self.to_index(y, 0);
            let end = self.to_index(y, k - 1);
            let row = &self.data[start..=end];
            let n = row[0] * rhs.x + row[1] * rhs.y + row[2] * rhs.z + row[3] * rhs.w;
            data.push(n);
        }

        Tuple4::new(data[0], data[1], data[2], data[3])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constructing_and_inspecting_2x2_matrix() {
        let matrix = Matrix::as_2x2(&[-3, 5, 1, -2]);

        assert_eq!(matrix.get(0, 0), -3.0);
        assert_eq!(matrix.get(0, 1), 5.0);
        assert_eq!(matrix.get(1, 0), 1.0);
        assert_eq!(matrix.get(1, 1), -2.0);
    }

    #[test]
    fn test_constructing_and_inspecting_3x3_matrix() {
        let matrix = Matrix::as_3x3(&[-3, 5, 0, 1, -2, -7, 0, 1, 1]);

        assert_eq!(matrix.get(0, 0), -3.0);
        assert_eq!(matrix.get(1, 1), -2.0);
        assert_eq!(matrix.get(2, 2), 1.0);
    }

    #[test]
    fn test_constructing_and_inspecting_4x4_matrix() {
        let matrix = Matrix::as_4x4(&[
            1.0, 2.0, 3.0, 4.0, 5.5, 6.5, 7.5, 8.5, 9.0, 10.0, 11.0, 12.0, 13.5, 14.5, 15.5, 16.5,
        ]);

        assert_eq!(matrix.get(0, 0), 1.0);
        assert_eq!(matrix.get(0, 3), 4.0);
        assert_eq!(matrix.get(1, 0), 5.5);
        assert_eq!(matrix.get(1, 2), 7.5);
        assert_eq!(matrix.get(2, 2), 11.0);
        assert_eq!(matrix.get(3, 0), 13.5);
        assert_eq!(matrix.get(3, 2), 15.5);
    }

    #[test]
    fn test_multiplying_two_matrices() {
        let a = Matrix::as_4x4(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 8, 7, 6, 5, 4, 3, 2]);
        let b = Matrix::as_4x4(&[-2, 1, 2, 3, 3, 2, 1, -1, 4, 3, 6, 5, 1, 2, 7, 8]);

        let result = a * b;

        assert_eq!(
            result,
            Matrix::as_4x4(&[20, 22, 50, 48, 44, 54, 114, 108, 40, 58, 110, 102, 16, 26, 46, 42])
        );
    }

    #[test]
    fn test_multiplying_matrix_with_tuple() {
        let matrix = Matrix::as_4x4(&[1, 2, 3, 4, 2, 4, 4, 2, 8, 6, 4, 1, 0, 0, 0, 1]);
        let tuple = Tuple4::new(1.0, 2.0, 3.0, 1.0);

        let result = matrix * tuple;

        assert_eq!(result, Tuple4::new(18.0, 24.0, 33.0, 1.0));
    }

    #[test]
    fn test_multiplying_matrix_by_identity_matrix() {
        let matrix = Matrix::as_4x4(&[0, 1, 2, 4, 1, 2, 4, 8, 2, 4, 8, 16, 4, 8, 16, 32]);
        let identity = Matrix::identity(4);

        let result = matrix.clone() * identity;

        assert_eq!(result, matrix);
    }

    #[test]
    fn test_multiplying_identity_matrix_by_tuple() {
        let identity = Matrix::identity(4);
        let tuple = Tuple4::new(1.0, 2.0, 3.0, 4.0);

        let result = identity * tuple;

        assert_eq!(result, tuple);
    }

    #[test]
    fn test_matrix_transpose() {
        let matrix = Matrix::as_4x4(&[0, 9, 3, 0, 9, 8, 0, 8, 1, 8, 5, 3, 0, 0, 5, 8]);

        let transposed_matrix = matrix.transpose();

        assert_eq!(
            transposed_matrix,
            Matrix::as_4x4(&[0, 9, 1, 0, 9, 8, 8, 0, 3, 0, 5, 5, 0, 8, 3, 8])
        );
    }

    #[test]
    fn test_matrix_transpose_twice() {
        let matrix = Matrix::as_4x4(&[0, 9, 3, 0, 9, 8, 0, 8, 1, 8, 5, 3, 0, 0, 5, 8]);

        let transposed_twice_matrix = matrix.clone().transpose().transpose();

        assert_eq!(transposed_twice_matrix, matrix);
    }

    #[test]
    fn test_submatrix_of_3x3_matrix() {
        let matrix = Matrix::as_3x3(&[1, 5, 0, -3, 2, 7, 0, 6, -3]);

        let submatrix = matrix.submatrix(0, 2);

        assert_eq!(submatrix, Matrix::as_2x2(&[-3, 2, 0, 6]));
    }

    #[test]
    fn test_submatrix_of_4x4_matrix() {
        let matrix = Matrix::as_4x4(&[-6, 1, 1, 6, -8, 5, 8, 6, -1, 0, 8, 2, -7, 1, -1, 1]);

        let submatrix = matrix.submatrix(2, 1);

        assert_eq!(submatrix, Matrix::as_3x3(&[-6, 1, 6, -8, 8, 6, -7, -1, 1]));
    }

    #[test]
    fn test_minor_of_3x3_matrix() {
        let matrix = Matrix::as_3x3(&[3, 5, 0, 2, -1, -7, 6, -1, 5]);

        let minor = matrix.minor(1, 0);

        assert_eq!(minor, 25.0);
    }

    #[test]
    fn test_cofactor_of_3x3_matrix() {
        let matrix = Matrix::as_3x3(&[3, 5, 0, 2, -1, -7, 6, -1, 5]);

        let minor_without_sign_change = matrix.cofactor(0, 0);
        let minor_with_sign_change = matrix.cofactor(1, 0);

        assert_eq!(minor_without_sign_change, -12.0);
        assert_eq!(minor_with_sign_change, -25.0);
    }

    #[test]
    fn test_determinant_of_1x1_matrix() {
        let matrix = Matrix::as_1x1(&[5]);

        let determinant = matrix.det();

        assert_eq!(determinant, 5.0);
    }

    #[test]
    fn test_2x2_matrix_determinant() {
        let matrix = Matrix::as_2x2(&[1, 5, -3, 2]);

        let determinant = matrix.det();

        assert_eq!(determinant, 17.0);
    }

    #[test]
    fn test_determinant_of_3x3_matrix() {
        let matrix = Matrix::as_3x3(&[1, 2, 6, -5, 8, -4, 2, 6, 4]);

        let det = matrix.det();

        assert_eq!(det, -196.0);
    }

    #[test]
    fn test_determinant_of_4x4_matrix() {
        let matrix = Matrix::as_4x4(&[-2, -8, 3, 5, -3, 1, 7, 3, 1, 2, -9, 6, -6, 7, 7, -9]);

        let det = matrix.det();

        assert_eq!(det, -4071.0);
    }

    #[test]
    fn test_if_matrix_is_invertible() {
        let matrix = Matrix::as_4x4(&[6, 4, 4, 4, 5, 5, 7, 6, 4, -9, 3, -7, 9, 1, 7, -6]);

        let is_invertible = matrix.is_invertible();

        assert_eq!(is_invertible, true);
    }

    #[test]
    fn test_if_matrix_is_not_invertible() {
        let matrix = Matrix::as_4x4(&[-4, 2, -2, -3, 9, 6, 2, 6, 0, -5, 1, -5, 0, 0, 0, 0]);

        let is_invertible = matrix.is_invertible();

        assert_eq!(is_invertible, false);
    }

    #[test]
    fn test_matrix_inverse() {
        let matrix = Matrix::as_4x4(&[-5, 2, 6, -8, 1, -5, 1, 8, 7, 7, -6, -7, 1, -3, 7, 4]);

        let inverse = matrix.inverse();

        let expected = Matrix::as_4x4(&[
            0.21805, 0.45113, 0.24060, -0.04511, -0.80827, -1.45677, -0.44361, 0.52068, -0.07895,
            -0.22368, -0.05263, 0.19737, -0.52256, -0.81391, -0.30075, 0.30639,
        ]);
        for y in 0..4 {
            for x in 0..4 {
                let a = expected.get(y, x);
                let b = inverse.get(y, x);
                assert!((a - b).abs() < 1e-5)
            }
        }
    }

    #[test]
    fn test_inverting_matrix_twice() {
        let matrix = Matrix::as_4x4(&[-5, 2, 6, -8, 1, -5, 1, 8, 7, 7, -6, -7, 1, -3, 7, 4]);

        let double_inversed = matrix.clone().inverse().inverse();

        for y in 0..4 {
            for x in 0..4 {
                let a = matrix.get(y, x);
                let b = double_inversed.get(y, x);
                assert!((a - b).abs() < Matrix::PRECISION)
            }
        }
    }

    #[test]
    #[should_panic]
    fn test_inverse_should_panic_on_non_square_matrix() {
        let matrix = Matrix::new(1, 2, &[1, 2]);

        matrix.inverse();
    }

    #[test]
    #[should_panic]
    fn test_inverse_should_panic_on_non_invertible_matrix() {
        let matrix = Matrix::as_4x4(&[-4, 2, -2, -3, 9, 6, 2, 6, 0, -5, 1, -5, 0, 0, 0, 0]);

        matrix.inverse();
    }
}
