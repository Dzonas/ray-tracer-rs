use std::ops::{Add, Mul, Sub};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Color { r, g, b }
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.r + rhs.r, self.g + rhs.g, self.b + rhs.b)
    }
}

impl Sub for Color {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.r - rhs.r, self.g - rhs.g, self.b - rhs.b)
    }
}

impl Mul<f64> for Color {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::new(self.r * rhs, self.g * rhs, self.b * rhs)
    }
}

impl Mul for Color {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.r * rhs.r, self.g * rhs.g, self.b * rhs.b)
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
    fn test_adding_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);

        let c3 = c1 + c2;

        assert_eq!(c3, Color::new(1.6, 0.7, 1.0));
    }

    #[test]
    fn test_subtracting_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);

        let c3 = c1 - c2;

        assert!(equal(c3.r, 0.2));
        assert!(equal(c3.g, 0.5));
        assert!(equal(c3.b, 0.5));
    }

    #[test]
    fn test_multiplying_a_color_by_a_scalar() {
        let c1 = Color::new(0.2, 0.3, 0.4);

        let c2 = c1 * 2.0;

        assert_eq!(c2, Color::new(0.4, 0.6, 0.8));
    }

    #[test]
    fn test_multiplying_colors() {
        let c1 = Color::new(1.0, 0.2, 0.4);
        let c2 = Color::new(0.9, 1.0, 0.1);

        let c3 = c1 * c2;

        assert!(equal(c3.r, 0.9));
        assert!(equal(c3.g, 0.2));
        assert!(equal(c3.b, 0.04));
    }
}
