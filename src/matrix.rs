use std::ops::Mul;

use crate::tuple::Tuple4;

fn to_index(size: usize, y: usize, x: usize) -> usize {
    y * size + x
}

fn to_yx(size: usize, i: usize) -> (usize, usize) {
    let y = i / size;
    let x = i % size;

    (y, x)
}

type Elem = f64;

#[derive(Debug, PartialEq, Clone, Copy)]
struct Matrix2x2 {
    data: [Elem; Matrix2x2::size()],
}

impl Matrix2x2 {
    const N: usize = 2;

    const fn size() -> usize {
        Matrix2x2::N * Matrix2x2::N
    }

    #[allow(dead_code)]
    fn new(data: [Elem; Matrix2x2::size()]) -> Matrix2x2 {
        Matrix2x2 { data }
    }

    #[allow(dead_code)]
    fn get(&self, y: usize, x: usize) -> Elem {
        let i = to_index(Matrix2x2::N, y, x);
        self.data[i]
    }

    fn det(&self) -> Elem {
        self.data[0] * self.data[3] - self.data[1] * self.data[2]
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Matrix3x3 {
    data: [Elem; Matrix3x3::size()],
}

impl Matrix3x3 {
    const N: usize = 3;

    const fn size() -> usize {
        Matrix3x3::N * Matrix3x3::N
    }

    #[allow(dead_code)]
    fn new(data: [Elem; Matrix3x3::size()]) -> Matrix3x3 {
        Matrix3x3 { data }
    }

    #[allow(dead_code)]
    fn get(&self, y: usize, x: usize) -> Elem {
        let i = to_index(Matrix3x3::N, y, x);
        self.data[i]
    }

    fn submatrix(&self, row: usize, col: usize) -> Matrix2x2 {
        let data = self
            .data
            .iter()
            .enumerate()
            .map(|(i, n)| (to_yx(Matrix3x3::N, i), n))
            .filter(|&((y, x), _)| y != row && x != col)
            .map(|(_, &n)| n)
            .collect::<Vec<Elem>>()
            .try_into()
            .unwrap();

        Matrix2x2 { data }
    }

    fn minor(&self, row: usize, col: usize) -> Elem {
        self.submatrix(row, col).det()
    }

    fn cofactor(&self, row: usize, col: usize) -> Elem {
        let n = if (row + col) % 2 == 1 { -1.0 } else { 1.0 };
        n * self.minor(row, col)
    }

    fn det(&self) -> Elem {
        self.data[..3]
            .iter()
            .enumerate()
            .map(|(i, &n)| n * self.cofactor(0, i))
            .sum()
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Matrix4x4 {
    data: [Elem; Matrix4x4::size()],
}

impl Matrix4x4 {
    const N: usize = 4;
    const PRECISION: f64 = 1e-12;

    const fn size() -> usize {
        Matrix4x4::N * Matrix4x4::N
    }

    pub fn new(data: [Elem; Matrix4x4::size()]) -> Self {
        Matrix4x4 { data }
    }

    pub fn zero() -> Self {
        Matrix4x4::new([0.0; Matrix4x4::size()])
    }

    pub fn identity() -> Self {
        let mut matrix = Matrix4x4::zero();
        for i in 0..Matrix4x4::N {
            matrix.data[i * (Matrix4x4::N + 1)] = 1.0;
        }

        matrix
    }

    pub fn translation(x: Elem, y: Elem, z: Elem) -> Self {
        let mut m = Self::identity();
        m.data[3] = x;
        m.data[7] = y;
        m.data[11] = z;

        m
    }

    pub fn scaling(x: Elem, y: Elem, z: Elem) -> Self {
        let mut m = Self::identity();
        m.data[0] = x;
        m.data[5] = y;
        m.data[10] = z;

        m
    }

    pub fn rotation_x(x: Elem) -> Self {
        let mut m = Self::identity();
        m.data[5] = x.cos();
        m.data[6] = -x.sin();
        m.data[9] = x.sin();
        m.data[10] = x.cos();

        m
    }

    pub fn rotation_y(y: Elem) -> Self {
        let mut m = Self::identity();
        m.data[0] = y.cos();
        m.data[2] = y.sin();
        m.data[8] = -y.sin();
        m.data[10] = y.cos();

        m
    }

    pub fn rotation_z(z: Elem) -> Self {
        let mut m = Self::identity();
        m.data[0] = z.cos();
        m.data[1] = -z.sin();
        m.data[4] = z.sin();
        m.data[5] = z.cos();

        m
    }

    pub fn shearing(xy: Elem, xz: Elem, yx: Elem, yz: Elem, zx: Elem, zy: Elem) -> Self {
        let mut m = Self::identity();
        m.data[1] = xy;
        m.data[2] = xz;
        m.data[4] = yx;
        m.data[6] = yz;
        m.data[8] = zx;
        m.data[9] = zy;

        m
    }

    pub fn get(&self, y: usize, x: usize) -> Elem {
        let i = self.to_index(y, x);
        self.data[i]
    }

    pub fn transpose(self) -> Self {
        let mut data = self.data;
        for y in 0..Matrix4x4::N {
            for x in y..Matrix4x4::N {
                let old_i = self.to_index(y, x);
                let new_i = self.to_index(x, y);
                data.swap(new_i, old_i);
            }
        }

        Matrix4x4 { data }
    }

    pub fn det(&self) -> Elem {
        self.data[..Matrix4x4::N]
            .iter()
            .enumerate()
            .map(|(i, &n)| n * self.cofactor(0, i))
            .sum()
    }

    pub fn is_invertible(&self) -> bool {
        self.is_invertible_with_det().0
    }

    pub fn inverse(self) -> Option<Self> {
        let (is_invertible, det) = self.is_invertible_with_det();
        if !is_invertible {
            return None;
        }
        let mut matrix = Matrix4x4::zero();
        for y in 0..Matrix4x4::N {
            for x in 0..Matrix4x4::N {
                let c = self.cofactor(y, x);
                let i = self.to_index(x, y);
                matrix.data[i] = c / det;
            }
        }

        Some(matrix)
    }

    fn is_invertible_with_det(&self) -> (bool, Elem) {
        let det = self.det();
        (det.abs() >= Self::PRECISION, det)
    }

    fn submatrix(&self, row: usize, col: usize) -> Matrix3x3 {
        let data = self
            .data
            .iter()
            .enumerate()
            .map(|(i, n)| (self.to_yx(i), n))
            .filter(|&((y, x), _)| y != row && x != col)
            .map(|(_, &n)| n)
            .collect::<Vec<Elem>>()
            .try_into()
            .unwrap();

        Matrix3x3 { data }
    }

    fn minor(&self, row: usize, col: usize) -> Elem {
        self.submatrix(row, col).det()
    }

    fn cofactor(&self, row: usize, col: usize) -> Elem {
        let n = if (row + col) % 2 == 1 { -1.0 } else { 1.0 };
        n * self.minor(row, col)
    }

    fn to_index(&self, y: usize, x: usize) -> usize {
        to_index(Matrix4x4::N, y, x)
    }

    fn to_yx(&self, i: usize) -> (usize, usize) {
        to_yx(Matrix4x4::N, i)
    }
}

impl Mul<Matrix4x4> for Matrix4x4 {
    type Output = Self;

    fn mul(self, rhs: Matrix4x4) -> Self::Output {
        let mut data = [0.0; Matrix4x4::size()];

        for y in 0..Matrix4x4::N {
            for x in 0..Matrix4x4::N {
                let n: Elem = (0..Matrix4x4::N)
                    .map(|n| self.get(y, n) * rhs.get(n, x))
                    .sum();
                let i = to_index(Matrix4x4::N, y, x);
                data[i] = n;
            }
        }

        Matrix4x4 { data }
    }
}

impl Mul<Tuple4> for Matrix4x4 {
    type Output = Tuple4;

    fn mul(self, rhs: Tuple4) -> Self::Output {
        let mut data = [0.0; Matrix4x4::N];

        for (i, row) in self.data.chunks(Matrix4x4::N).enumerate() {
            let n = row[0] * rhs.x + row[1] * rhs.y + row[2] * rhs.z + row[3] * rhs.w;
            data[i] = n;
        }

        Tuple4::new(data[0], data[1], data[2], data[3])
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use super::*;

    const EPSILON: f64 = 1e-6;

    fn equal(a: f64, b: f64) -> bool {
        (a - b).abs() < EPSILON
    }

    #[test]
    fn test_constructing_and_inspecting_2x2_matrix() {
        let matrix = Matrix2x2::new([-3.0, 5.0, 1.0, -2.0]);

        assert_eq!(matrix.get(0, 0), -3.0);
        assert_eq!(matrix.get(0, 1), 5.0);
        assert_eq!(matrix.get(1, 0), 1.0);
        assert_eq!(matrix.get(1, 1), -2.0);
    }

    #[test]
    fn test_det_of_2x2_matrix() {
        let matrix = Matrix2x2::new([1.0, 5.0, -3.0, 2.0]);

        let det = matrix.det();

        assert_eq!(det, 17.0);
    }

    #[test]
    fn test_constructing_and_inspecting_3x3_matrix() {
        let matrix = Matrix3x3::new([-3.0, 5.0, 0.0, 1.0, -2.0, -7.0, 0.0, 1.0, 1.0]);

        assert_eq!(matrix.get(0, 0), -3.0);
        assert_eq!(matrix.get(1, 1), -2.0);
        assert_eq!(matrix.get(2, 2), 1.0);
    }

    #[test]
    fn test_submatrix_of_3x3_matrix() {
        let matrix = Matrix3x3::new([1.0, 5.0, 0.0, -3.0, 2.0, 7.0, 0.0, 6.0, -3.0]);

        let submatrix = matrix.submatrix(0, 2);

        assert_eq!(submatrix, Matrix2x2::new([-3.0, 2.0, 0.0, 6.0]));
    }

    #[test]
    fn test_minor_of_3x3_matrix() {
        let matrix = Matrix3x3::new([3.0, 5.0, 0.0, 2.0, -1.0, -7.0, 6.0, -1.0, 5.0]);

        let minor = matrix.minor(1, 0);

        assert_eq!(minor, 25.0);
    }

    #[test]
    fn test_cofactor_of_3x3_matrix() {
        let matrix = Matrix3x3::new([3.0, 5.0, 0.0, 2.0, -1.0, -7.0, 6.0, -1.0, 5.0]);

        let minor_without_sign_change = matrix.cofactor(0, 0);
        let minor_with_sign_change = matrix.cofactor(1, 0);

        assert_eq!(minor_without_sign_change, -12.0);
        assert_eq!(minor_with_sign_change, -25.0);
    }

    #[test]
    fn test_determinant_of_3x3_matrix() {
        let matrix = Matrix3x3::new([1.0, 2.0, 6.0, -5.0, 8.0, -4.0, 2.0, 6.0, 4.0]);

        let det = matrix.det();

        assert_eq!(det, -196.0);
    }

    #[test]
    fn test_constructing_and_inspecting_4x4_matrix() {
        let matrix = Matrix4x4::new([
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
        let a = Matrix4x4::new([
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
        ]);
        let b = Matrix4x4::new([
            -2.0, 1.0, 2.0, 3.0, 3.0, 2.0, 1.0, -1.0, 4.0, 3.0, 6.0, 5.0, 1.0, 2.0, 7.0, 8.0,
        ]);

        let result = a * b;

        assert_eq!(
            result,
            Matrix4x4::new([
                20.0, 22.0, 50.0, 48.0, 44.0, 54.0, 114.0, 108.0, 40.0, 58.0, 110.0, 102.0, 16.0,
                26.0, 46.0, 42.0
            ])
        );
    }

    #[test]
    fn test_multiplying_matrix_with_tuple() {
        let matrix = Matrix4x4::new([
            1.0, 2.0, 3.0, 4.0, 2.0, 4.0, 4.0, 2.0, 8.0, 6.0, 4.0, 1.0, 0.0, 0.0, 0.0, 1.0,
        ]);
        let tuple = Tuple4::new(1.0, 2.0, 3.0, 1.0);

        let result = matrix * tuple;

        assert_eq!(result, Tuple4::new(18.0, 24.0, 33.0, 1.0));
    }

    #[test]
    fn test_multiplying_matrix_by_identity_matrix() {
        let matrix = Matrix4x4::new([
            0.0, 1.0, 2.0, 4.0, 1.0, 2.0, 4.0, 8.0, 2.0, 4.0, 8.0, 16.0, 4.0, 8.0, 16.0, 32.0,
        ]);
        let identity = Matrix4x4::identity();

        let result = matrix.clone() * identity;

        assert_eq!(result, matrix);
    }

    #[test]
    fn test_multiplying_identity_matrix_by_tuple() {
        let identity = Matrix4x4::identity();
        let tuple = Tuple4::new(1.0, 2.0, 3.0, 4.0);

        let result = identity * tuple;

        assert_eq!(result, tuple);
    }

    #[test]
    fn test_matrix_transpose() {
        let matrix = Matrix4x4::new([
            0.0, 9.0, 3.0, 0.0, 9.0, 8.0, 0.0, 8.0, 1.0, 8.0, 5.0, 3.0, 0.0, 0.0, 5.0, 8.0,
        ]);

        let transposed_matrix = matrix.transpose();

        assert_eq!(
            transposed_matrix,
            Matrix4x4::new([
                0.0, 9.0, 1.0, 0.0, 9.0, 8.0, 8.0, 0.0, 3.0, 0.0, 5.0, 5.0, 0.0, 8.0, 3.0, 8.0
            ])
        );
    }

    #[test]
    fn test_matrix_transpose_twice() {
        let matrix = Matrix4x4::new([
            0.0, 9.0, 3.0, 0.0, 9.0, 8.0, 0.0, 8.0, 1.0, 8.0, 5.0, 3.0, 0.0, 0.0, 5.0, 8.0,
        ]);

        let transposed_twice_matrix = matrix.clone().transpose().transpose();

        assert_eq!(transposed_twice_matrix, matrix);
    }

    #[test]
    fn test_submatrix_of_4x4_matrix() {
        let matrix = Matrix4x4::new([
            -6.0, 1.0, 1.0, 6.0, -8.0, 5.0, 8.0, 6.0, -1.0, 0.0, 8.0, 2.0, -7.0, 1.0, -1.0, 1.0,
        ]);

        let submatrix = matrix.submatrix(2, 1);

        assert_eq!(
            submatrix,
            Matrix3x3::new([-6.0, 1.0, 6.0, -8.0, 8.0, 6.0, -7.0, -1.0, 1.0])
        );
    }

    #[test]
    fn test_determinant_of_4x4_matrix() {
        let matrix = Matrix4x4::new([
            -2.0, -8.0, 3.0, 5.0, -3.0, 1.0, 7.0, 3.0, 1.0, 2.0, -9.0, 6.0, -6.0, 7.0, 7.0, -9.0,
        ]);

        let det = matrix.det();

        assert_eq!(det, -4071.0);
    }

    #[test]
    fn test_if_matrix_is_invertible() {
        let matrix = Matrix4x4::new([
            6.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 6.0, 4.0, -9.0, 3.0, -7.0, 9.0, 1.0, 7.0, -6.0,
        ]);

        let is_invertible = matrix.is_invertible();

        assert_eq!(is_invertible, true);
    }

    #[test]
    fn test_if_matrix_is_not_invertible() {
        let matrix = Matrix4x4::new([
            -4.0, 2.0, -2.0, -3.0, 9.0, 6.0, 2.0, 6.0, 0.0, -5.0, 1.0, -5.0, 0.0, 0.0, 0.0, 0.0,
        ]);

        let is_invertible = matrix.is_invertible();

        assert_eq!(is_invertible, false);
    }

    #[test]
    fn test_matrix_inverse() {
        let matrix = Matrix4x4::new([
            -5.0, 2.0, 6.0, -8.0, 1.0, -5.0, 1.0, 8.0, 7.0, 7.0, -6.0, -7.0, 1.0, -3.0, 7.0, 4.0,
        ]);

        let inverse = matrix.inverse().unwrap();

        let expected = Matrix4x4::new([
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
        let matrix = Matrix4x4::new([
            -5.0, 2.0, 6.0, -8.0, 1.0, -5.0, 1.0, 8.0, 7.0, 7.0, -6.0, -7.0, 1.0, -3.0, 7.0, 4.0,
        ]);

        let double_inversed = matrix.clone().inverse().unwrap().inverse().unwrap();

        for y in 0..4 {
            for x in 0..4 {
                let a = matrix.get(y, x);
                let b = double_inversed.get(y, x);
                assert!((a - b).abs() < Matrix4x4::PRECISION)
            }
        }
    }

    #[test]
    fn test_inverse_of_non_invertible_matrix() {
        let matrix = Matrix4x4::new([
            -4.0, 2.0, -2.0, -3.0, 9.0, 6.0, 2.0, 6.0, 0.0, -5.0, 1.0, -5.0, 0.0, 0.0, 0.0, 0.0,
        ]);

        let inverse = matrix.inverse();

        assert_eq!(inverse, None);
    }

    #[test]
    fn test_multiplying_point_by_translation_matrix() {
        let t = Matrix4x4::translation(5.0, -3.0, 2.0);
        let p = Tuple4::point(-3.0, 4.0, 5.0);

        let result = t * p;

        assert_eq!(result, Tuple4::point(2.0, 1.0, 7.0));
    }

    #[test]
    fn test_multiplying_point_by_inverse_of_translation_matrix() {
        let t = Matrix4x4::translation(5.0, -3.0, 2.0).inverse().unwrap();
        let p = Tuple4::point(-3.0, 4.0, 5.0);

        let result = t * p;

        assert_eq!(result, Tuple4::point(-8.0, 7.0, 3.0));
    }

    #[test]
    fn test_multiplying_vector_by_inverse_matrix() {
        let t = Matrix4x4::translation(5.0, -3.0, 2.0);
        let p = Tuple4::vector(-3.0, 4.0, 5.0);

        let result = t * p;

        assert_eq!(result, Tuple4::vector(-3.0, 4.0, 5.0));
    }

    #[test]
    fn test_scaling_matrix_applied_to_a_point() {
        let s = Matrix4x4::scaling(2.0, 3.0, 4.0);
        let p = Tuple4::point(-4.0, 6.0, 8.0);

        let result = s * p;

        assert_eq!(result, Tuple4::point(-8.0, 18.0, 32.0));
    }

    #[test]
    fn test_scaling_matrix_applied_to_a_vector() {
        let s = Matrix4x4::scaling(2.0, 3.0, 4.0);
        let p = Tuple4::vector(-4.0, 6.0, 8.0);

        let result = s * p;

        assert_eq!(result, Tuple4::vector(-8.0, 18.0, 32.0));
    }

    #[test]
    fn test_inverse_of_scaling_matrix_applied_to_a_vector() {
        let s = Matrix4x4::scaling(2.0, 3.0, 4.0).inverse().unwrap();
        let p = Tuple4::vector(-4.0, 6.0, 8.0);

        let result = s * p;

        assert_eq!(result, Tuple4::vector(-2.0, 2.0, 2.0));
    }

    #[test]
    fn test_reflection_as_scaling_by_negative_value() {
        let s = Matrix4x4::scaling(-1.0, 1.0, 1.0);
        let p = Tuple4::point(2.0, 3.0, 4.0);

        let result = s * p;

        assert_eq!(result, Tuple4::point(-2.0, 3.0, 4.0));
    }

    #[test]
    fn test_rotating_point_around_x_axis() {
        let p = Tuple4::point(0.0, 1.0, 0.0);
        let half_quarter = Matrix4x4::rotation_x(PI / 4.0);
        let full_quarter = Matrix4x4::rotation_x(PI / 2.0);

        let r1 = half_quarter * p;
        let r2 = full_quarter * p;

        assert_eq!(r1.x, 0.0);
        assert!(equal(r1.y, 2.0_f64.sqrt() / 2.0));
        assert!(equal(r1.z, 2.0_f64.sqrt() / 2.0));

        assert_eq!(r2.x, 0.0);
        assert!(equal(r2.y, 0.0));
        assert!(equal(r2.z, 1.0));
    }

    #[test]
    fn test_inverse_of_x_rotation_rotates_in_opposite_direction() {
        let p = Tuple4::point(0.0, 1.0, 0.0);
        let half_quarter = Matrix4x4::rotation_x(PI / 4.0);

        let result = half_quarter.inverse().unwrap() * p;

        assert_eq!(result.x, 0.0);
        assert!(equal(result.y, 2.0_f64.sqrt() / 2.0));
        assert!(equal(result.z, -2.0_f64.sqrt() / 2.0));
    }

    #[test]
    fn test_rotating_point_around_y_axis() {
        let p = Tuple4::point(0.0, 0.0, 1.0);
        let half_quarter = Matrix4x4::rotation_y(PI / 4.0);
        let full_quarter = Matrix4x4::rotation_y(PI / 2.0);

        let r1 = half_quarter * p;
        let r2 = full_quarter * p;

        assert!(equal(r1.x, 2.0_f64.sqrt() / 2.0));
        assert_eq!(r1.y, 0.0);
        assert!(equal(r1.z, 2.0_f64.sqrt() / 2.0));

        assert!(equal(r2.x, 1.0));
        assert_eq!(r2.y, 0.0);
        assert!(equal(r2.z, 0.0));
    }

    #[test]
    fn test_rotating_point_around_z_axis() {
        let p = Tuple4::point(0.0, 1.0, 0.0);
        let half_quarter = Matrix4x4::rotation_z(PI / 4.0);
        let full_quarter = Matrix4x4::rotation_z(PI / 2.0);

        let r1 = half_quarter * p;
        let r2 = full_quarter * p;

        assert!(equal(r1.x, -2.0_f64.sqrt() / 2.0));
        assert!(equal(r1.y, 2.0_f64.sqrt() / 2.0));
        assert_eq!(r1.z, 0.0);

        assert!(equal(r2.x, -1.0));
        assert!(equal(r2.y, 0.0));
        assert_eq!(r1.z, 0.0);
    }

    #[test]
    fn test_shearing_x_in_proportion_to_y() {
        let transform = Matrix4x4::shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let point = Tuple4::point(2.0, 3.0, 4.0);

        let result = transform * point;

        assert_eq!(result, Tuple4::point(5.0, 3.0, 4.0));
    }

    #[test]
    fn test_shearing_x_in_proportion_to_z() {
        let transform = Matrix4x4::shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let point = Tuple4::point(2.0, 3.0, 4.0);

        let result = transform * point;

        assert_eq!(result, Tuple4::point(6.0, 3.0, 4.0));
    }

    #[test]
    fn test_shearing_y_in_proportion_to_x() {
        let transform = Matrix4x4::shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        let point = Tuple4::point(2.0, 3.0, 4.0);

        let result = transform * point;

        assert_eq!(result, Tuple4::point(2.0, 5.0, 4.0));
    }

    #[test]
    fn test_shearing_y_in_proportion_to_z() {
        let transform = Matrix4x4::shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let point = Tuple4::point(2.0, 3.0, 4.0);

        let result = transform * point;

        assert_eq!(result, Tuple4::point(2.0, 7.0, 4.0));
    }

    #[test]
    fn test_shearing_z_in_proportion_to_x() {
        let transform = Matrix4x4::shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let point = Tuple4::point(2.0, 3.0, 4.0);

        let result = transform * point;

        assert_eq!(result, Tuple4::point(2.0, 3.0, 6.0));
    }

    #[test]
    fn test_shearing_z_in_proportion_to_y() {
        let transform = Matrix4x4::shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let point = Tuple4::point(2.0, 3.0, 4.0);

        let result = transform * point;

        assert_eq!(result, Tuple4::point(2.0, 3.0, 7.0));
    }
}
