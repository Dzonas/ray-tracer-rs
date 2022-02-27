use crate::tuple::Tuple4;

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
}
