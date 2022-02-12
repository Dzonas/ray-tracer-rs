use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Tuple {
    fn new(x: f64, y: f64, z: f64, w: f64) -> Tuple {
        Tuple { x, y, z, w }
    }

    pub fn point(x: f64, y: f64, z: f64) -> Tuple {
        Tuple::new(x, y, z, 1.0)
    }

    pub fn vector(x: f64, y: f64, z: f64) -> Tuple {
        Tuple::new(x, y, z, 0.0)
    }

    pub fn is_point(&self) -> bool {
        self.w == 1.0
    }

    pub fn is_vector(&self) -> bool {
        self.w == 0.0
    }

    pub fn negate(self) -> Tuple {
        Tuple::new(-self.x, -self.y, -self.z, -self.w)
    }

    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(self) -> Tuple {
        let mag = self.magnitude();
        self / mag
    }

    pub fn dot(self, other: Tuple) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    pub fn cross(self, other: Tuple) -> Tuple {
        Tuple::vector(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }
}

impl Add for Tuple {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Tuple::new(
            self.x + other.x,
            self.y + other.y,
            self.z + other.z,
            self.w + other.w,
        )
    }
}

impl Sub for Tuple {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Tuple::new(
            self.x - other.x,
            self.y - other.y,
            self.z - other.z,
            self.w - other.w,
        )
    }
}

impl Mul<f64> for Tuple {
    type Output = Self;

    fn mul(self, other: f64) -> Self::Output {
        Tuple::new(
            self.x * other,
            self.y * other,
            self.z * other,
            self.w * other,
        )
    }
}

impl Mul<Tuple> for f64 {
    type Output = Tuple;

    fn mul(self, other: Tuple) -> Self::Output {
        other * self
    }
}

impl Div<f64> for Tuple {
    type Output = Self;

    fn div(self, other: f64) -> Self::Output {
        self * (1.0 / other)
    }
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
        let tuple = Tuple::new(4.3, -4.2, 3.1, 1.0);

        assert_eq!(tuple.x, 4.3);
        assert_eq!(tuple.y, -4.2);
        assert_eq!(tuple.z, 3.1);
        assert_eq!(tuple.w, 1.0);
        assert_eq!(tuple.is_point(), true);
        assert_eq!(tuple.is_vector(), false);
    }

    #[test]
    fn test_tuple_with_w_as_zero_should_be_a_vector() {
        let tuple = Tuple::new(4.3, -4.2, 3.1, 0.0);

        assert_eq!(tuple.x, 4.3);
        assert_eq!(tuple.y, -4.2);
        assert_eq!(tuple.z, 3.1);
        assert_eq!(tuple.w, 0.0);
        assert_eq!(tuple.is_point(), false);
        assert_eq!(tuple.is_vector(), true);
    }

    #[test]
    fn test_point_function_should_return_tuple_with_w_as_one() {
        let point = Tuple::point(4.3, -4.2, 3.1);

        assert_eq!(point, Tuple::new(4.3, -4.2, 3.1, 1.0));
    }

    #[test]
    fn test_vector_function_should_return_tuple_with_w_as_zero() {
        let vector = Tuple::vector(4.3, -4.2, 3.1);

        assert_eq!(vector, Tuple::new(4.3, -4.2, 3.1, 0.0));
    }

    #[test]
    fn test_adding_two_tuples() {
        let t1 = Tuple::new(3.0, -2.0, 5.0, 1.0);
        let t2 = Tuple::new(-2.0, 3.0, 1.0, 0.0);

        let result = t1 + t2;

        assert_eq!(result, Tuple::new(1.0, 1.0, 6.0, 1.0));
    }

    #[test]
    fn test_subtracting_two_tuples() {
        let t1 = Tuple::new(3.0, -2.0, 5.0, 1.0);
        let t2 = Tuple::new(-2.0, 3.0, 1.0, 0.0);

        let result = t1 - t2;

        assert_eq!(result, Tuple::new(5.0, -5.0, 4.0, 1.0));
    }

    #[test]
    fn test_negating_a_tuple() {
        let t = Tuple::new(1.0, -2.0, 3.0, -4.0);

        let result = t.negate();

        assert_eq!(result, Tuple::new(-1.0, 2.0, -3.0, 4.0));
    }

    #[test]
    fn test_multiply_scalar_by_a_tuple() {
        let t = Tuple::new(1.0, -2.0, 3.0, -4.0);
        let scalar = 0.5;

        let result = t * scalar;

        assert_eq!(result, Tuple::new(0.5, -1.0, 1.5, -2.0));
    }

    #[test]
    fn test_multiply_tuple_by_a_scalar() {
        let t = Tuple::new(1.0, -2.0, 3.0, -4.0);
        let scalar = 0.5;

        let result = scalar * t;

        assert_eq!(result, Tuple::new(0.5, -1.0, 1.5, -2.0));
    }

    #[test]
    fn test_divide_tuple_by_a_scalar() {
        let t = Tuple::new(1.0, -2.0, 3.0, -4.0);
        let scalar = 2.0;

        let result = t / scalar;

        assert_eq!(result, Tuple::new(0.5, -1.0, 1.5, -2.0));
    }

    #[test]
    fn test_vector_magnitude() {
        let v = Tuple::vector(1.0, 2.0, 3.0);

        let mag = v.magnitude();

        assert_eq!(mag * mag, 14.0);
    }

    #[test]
    fn test_vector_normalize() {
        let v = Tuple::vector(1.0, 2.0, 3.0);

        let normalized_v = v.normalize();

        assert_eq!(equal(normalized_v.x, 0.267261), true);
        assert_eq!(equal(normalized_v.y, 0.534522), true);
        assert_eq!(equal(normalized_v.z, 0.801783), true);
    }

    #[test]
    fn test_vector_dot_product() {
        let v1 = Tuple::vector(1.0, 2.0, 3.0);
        let v2 = Tuple::vector(2.0, 3.0, 4.0);

        let result = v1.dot(v2);

        assert_eq!(result, 20.0);
    }

    #[test]
    fn test_vector_cross_product() {
        let v1 = Tuple::vector(1.0, 2.0, 3.0);
        let v2 = Tuple::vector(2.0, 3.0, 4.0);

        let result = v1.cross(v2);

        assert_eq!(result, Tuple::vector(-1.0, 2.0, -1.0));
    }
}
