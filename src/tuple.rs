use std::ops::{Add, Div, Mul, Sub};

use crate::ppm::RGB;

type Elem = f64;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Tuple4 {
    pub x: Elem,
    pub y: Elem,
    pub z: Elem,
    pub w: Elem,
}

impl Tuple4 {
    pub fn new<T: Copy + Into<Elem>>(x: T, y: T, z: T, w: T) -> Self {
        Tuple4 {
            x: x.into(),
            y: y.into(),
            z: z.into(),
            w: w.into(),
        }
    }

    pub fn point<T: Copy + Into<Elem>>(x: T, y: T, z: T) -> Self {
        Tuple4::new(x.into(), y.into(), z.into(), 1.0)
    }

    pub fn vector<T: Copy + Into<Elem>>(x: T, y: T, z: T) -> Self {
        Tuple4::new(x.into(), y.into(), z.into(), 0.0)
    }

    pub fn is_point(&self) -> bool {
        self.w == 1.0
    }

    pub fn is_vector(&self) -> bool {
        self.w == 0.0
    }

    pub fn negate(self) -> Self {
        Tuple4::new(-self.x, -self.y, -self.z, -self.w)
    }

    pub fn magnitude(&self) -> Elem {
        self.dot(self).sqrt()
    }

    pub fn normalize(self) -> Self {
        let mag = self.magnitude();
        self / mag
    }

    pub fn dot(self, other: &Tuple4) -> Elem {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    pub fn cross(self, other: Tuple4) -> Self {
        Self::vector(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }
}

impl Add for Tuple4 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Tuple4::new(
            self.x + other.x,
            self.y + other.y,
            self.z + other.z,
            self.w + other.w,
        )
    }
}

impl Sub for Tuple4 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Tuple4::new(
            self.x - other.x,
            self.y - other.y,
            self.z - other.z,
            self.w - other.w,
        )
    }
}

impl Mul<Elem> for Tuple4 {
    type Output = Self;

    fn mul(self, other: Elem) -> Self::Output {
        Tuple4::new(
            self.x * other,
            self.y * other,
            self.z * other,
            self.w * other,
        )
    }
}

impl Mul<Tuple4> for Elem {
    type Output = Tuple4;

    fn mul(self, other: Tuple4) -> Self::Output {
        other * self
    }
}

impl Div<Elem> for Tuple4 {
    type Output = Self;

    fn div(self, other: Elem) -> Self::Output {
        self * (1.0 / other)
    }
}

impl RGB for Tuple4 {
    fn r(&self) -> u8 {
        clamp_to_u8(self.x)
    }

    fn g(&self) -> u8 {
        clamp_to_u8(self.y)
    }

    fn b(&self) -> u8 {
        clamp_to_u8(self.z)
    }
}

fn clamp_to_u8(n: f64) -> u8 {
    (n * 255.0).clamp(0.0, 255.0).round() as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 1e-6;

    fn equal(a: f64, b: f64) -> bool {
        (a - b).abs() < EPSILON
    }

    #[test]
    fn test_tuple_with_w_as_one_should_be_a_point() {
        let tuple = Tuple4::new(4.3, -4.2, 3.1, 1.0);

        assert_eq!(tuple.x, 4.3);
        assert_eq!(tuple.y, -4.2);
        assert_eq!(tuple.z, 3.1);
        assert_eq!(tuple.w, 1.0);
        assert_eq!(tuple.is_point(), true);
        assert_eq!(tuple.is_vector(), false);
    }

    #[test]
    fn test_tuple_with_w_as_zero_should_be_a_vector() {
        let tuple = Tuple4::new(4.3, -4.2, 3.1, 0.0);

        assert_eq!(tuple.x, 4.3);
        assert_eq!(tuple.y, -4.2);
        assert_eq!(tuple.z, 3.1);
        assert_eq!(tuple.w, 0.0);
        assert_eq!(tuple.is_point(), false);
        assert_eq!(tuple.is_vector(), true);
    }

    #[test]
    fn test_point_function_should_return_tuple_with_w_as_one() {
        let point = Tuple4::point(4.3, -4.2, 3.1);

        assert_eq!(point, Tuple4::new(4.3, -4.2, 3.1, 1.0));
    }

    #[test]
    fn test_vector_function_should_return_tuple_with_w_as_zero() {
        let vector = Tuple4::vector(4.3, -4.2, 3.1);

        assert_eq!(vector, Tuple4::new(4.3, -4.2, 3.1, 0.0));
    }

    #[test]
    fn test_adding_two_tuples() {
        let t1 = Tuple4::new(3.0, -2.0, 5.0, 1.0);
        let t2 = Tuple4::new(-2.0, 3.0, 1.0, 0.0);

        let result = t1 + t2;

        assert_eq!(result, Tuple4::new(1.0, 1.0, 6.0, 1.0));
    }

    #[test]
    fn test_subtracting_two_tuples() {
        let t1 = Tuple4::new(3.0, -2.0, 5.0, 1.0);
        let t2 = Tuple4::new(-2.0, 3.0, 1.0, 0.0);

        let result = t1 - t2;

        assert_eq!(result, Tuple4::new(5.0, -5.0, 4.0, 1.0));
    }

    #[test]
    fn test_negating_a_tuple() {
        let t = Tuple4::new(1.0, -2.0, 3.0, -4.0);

        let result = t.negate();

        assert_eq!(result, Tuple4::new(-1.0, 2.0, -3.0, 4.0));
    }

    #[test]
    fn test_multiply_scalar_by_a_tuple() {
        let t = Tuple4::new(1.0, -2.0, 3.0, -4.0);
        let scalar = 0.5;

        let result = t * scalar;

        assert_eq!(result, Tuple4::new(0.5, -1.0, 1.5, -2.0));
    }

    #[test]
    fn test_multiply_tuple_by_a_scalar() {
        let t = Tuple4::new(1.0, -2.0, 3.0, -4.0);
        let scalar = 0.5;

        let result = scalar * t;

        assert_eq!(result, Tuple4::new(0.5, -1.0, 1.5, -2.0));
    }

    #[test]
    fn test_divide_tuple_by_a_scalar() {
        let t = Tuple4::new(1.0, -2.0, 3.0, -4.0);
        let scalar = 2.0;

        let result = t / scalar;

        assert_eq!(result, Tuple4::new(0.5, -1.0, 1.5, -2.0));
    }

    #[test]
    fn test_vector_magnitude() {
        let v = Tuple4::vector(1.0, 2.0, 3.0);

        let mag = v.magnitude();

        assert_eq!(mag * mag, 14.0);
    }

    #[test]
    fn test_vector_normalize() {
        let v = Tuple4::vector(1.0, 2.0, 3.0);

        let normalized_v = v.normalize();

        assert_eq!(equal(normalized_v.x, 0.267261), true);
        assert_eq!(equal(normalized_v.y, 0.534522), true);
        assert_eq!(equal(normalized_v.z, 0.801783), true);
    }

    #[test]
    fn test_vector_dot_product() {
        let v1 = Tuple4::vector(1.0, 2.0, 3.0);
        let v2 = Tuple4::vector(2.0, 3.0, 4.0);

        let result = v1.dot(&v2);

        assert_eq!(result, 20.0);
    }

    #[test]
    fn test_vector_cross_product() {
        let v1 = Tuple4::vector(1.0, 2.0, 3.0);
        let v2 = Tuple4::vector(2.0, 3.0, 4.0);

        let result = v1.cross(v2);

        assert_eq!(result, Tuple4::vector(-1.0, 2.0, -1.0));
    }

    #[test]
    fn test_tuple_color_mapping() {
        let p = Tuple4::point(1.0, 0.5, 1.5);

        assert_eq!(p.r(), 255);
        assert_eq!(p.g(), 128);
        assert_eq!(p.b(), 255);
    }
}
