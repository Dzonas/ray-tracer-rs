use crate::{matrix::Matrix4x4, tuple::Tuple4};

pub struct Ray {
    pub origin: Tuple4,
    pub direction: Tuple4,
}

impl Ray {
    pub fn new(origin: Tuple4, direction: Tuple4) -> Ray {
        Ray { origin, direction }
    }

    pub fn position(&self, t: f64) -> Tuple4 {
        self.origin + self.direction * t
    }

    pub fn transform(&self, m: Matrix4x4) -> Ray {
        let new_origin = m * self.origin;
        let new_direction = m * self.direction;

        Ray {
            origin: new_origin,
            direction: new_direction,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_computing_point_from_a_distance() {
        let r = Ray::new(Tuple4::point(2.0, 3.0, 4.0), Tuple4::vector(1.0, 0.0, 0.0));

        assert_eq!(r.position(0.0), Tuple4::point(2.0, 3.0, 4.0));
        assert_eq!(r.position(1.0), Tuple4::point(3.0, 3.0, 4.0));
        assert_eq!(r.position(-1.0), Tuple4::point(1.0, 3.0, 4.0));
        assert_eq!(r.position(2.5), Tuple4::point(4.5, 3.0, 4.0));
    }

    #[test]
    fn test_translating_a_ray() {
        let r = Ray::new(Tuple4::point(1.0, 2.0, 3.0), Tuple4::vector(0.0, 1.0, 0.0));
        let m = Matrix4x4::translation(3.0, 4.0, 5.0);

        let r2 = r.transform(m);

        assert_eq!(r2.origin, Tuple4::point(4.0, 6.0, 8.0));
        assert_eq!(r2.direction, Tuple4::vector(0.0, 1.0, 0.0));
    }

    #[test]
    fn test_scaling_a_ray() {
        let r = Ray::new(Tuple4::point(1.0, 2.0, 3.0), Tuple4::vector(0.0, 1.0, 0.0));
        let m = Matrix4x4::scaling(2.0, 3.0, 4.0);

        let r2 = r.transform(m);

        assert_eq!(r2.origin, Tuple4::point(2.0, 6.0, 12.0));
        assert_eq!(r2.direction, Tuple4::vector(0.0, 3.0, 0.0));
    }
}
