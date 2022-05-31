use std::ops::Index;

use crate::ray::Ray;
use crate::tuple::Tuple4;

#[allow(dead_code)]
pub struct Sphere {
    origin: Tuple4,
    radius: f64,
}

impl Sphere {
    pub fn new() -> Sphere {
        let origin = Tuple4::point(0.0, 0.0, 0.0);
        let radius = 1.0;

        Sphere { origin, radius }
    }

    pub fn intersect(&self, ray: &Ray) -> SphereIntersections {
        let sphere_to_ray = ray.origin - self.origin;
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * ray.direction.dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;
        let discriminant = b * b - 4.0 * a * c;

        let intersections = if discriminant < 0.0 {
            Vec::new()
        } else {
            let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
            let i1 = SphereIntersection::new(t1, self);
            let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
            let i2 = SphereIntersection::new(t2, self);
            vec![i1, i2]
        };

        SphereIntersections::new(intersections)
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(dead_code)]
pub struct SphereIntersection<'a> {
    t: f64,
    sphere: &'a Sphere,
}

impl SphereIntersection<'_> {
    pub fn new(t: f64, sphere: &Sphere) -> SphereIntersection {
        SphereIntersection { t, sphere }
    }
}

pub struct SphereIntersections<'a> {
    intersections: Vec<SphereIntersection<'a>>,
}

impl SphereIntersections<'_> {
    pub fn new(intersections: Vec<SphereIntersection<'_>>) -> SphereIntersections {
        SphereIntersections { intersections }
    }

    pub fn len(&self) -> usize {
        self.intersections.len()
    }

    pub fn is_empty(&self) -> bool {
        self.intersections.len() == 0
    }

    pub fn hit(&self) -> Option<&SphereIntersection> {
        self.intersections
            .iter()
            .filter(|x| x.t >= 0.0)
            .min_by(|a, b| a.t.partial_cmp(&b.t).expect("Tried to compare to NaN"))
    }
}

impl<'a> Index<usize> for SphereIntersections<'a> {
    type Output = SphereIntersection<'a>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.intersections[index]
    }
}

#[cfg(test)]
mod tests {
    use std::ptr;

    use super::*;

    #[test]
    fn test_a_ray_intersects_sphere_at_two_points() {
        let r = Ray::new(Tuple4::point(0.0, 0.0, -5.0), Tuple4::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();

        let xs = s.intersect(&r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 6.0);
    }

    #[test]
    fn test_a_ray_intersects_sphere_at_a_tangent() {
        let r = Ray::new(Tuple4::point(0.0, 1.0, -5.0), Tuple4::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();

        let xs = s.intersect(&r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 5.0);
        assert_eq!(xs[1].t, 5.0);
    }

    #[test]
    fn test_a_ray_misses_a_sphere() {
        let r = Ray::new(Tuple4::point(0.0, 2.0, -5.0), Tuple4::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();

        let xs = s.intersect(&r);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn test_a_ray_originates_inside_a_sphere() {
        let r = Ray::new(Tuple4::point(0.0, 0.0, 0.0), Tuple4::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();

        let xs = s.intersect(&r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -1.0);
        assert_eq!(xs[1].t, 1.0);
    }

    #[test]
    fn test_a_sphere_is_behind_a_ray() {
        let r = Ray::new(Tuple4::point(0.0, 0.0, 5.0), Tuple4::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();

        let xs = s.intersect(&r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -6.0);
        assert_eq!(xs[1].t, -4.0);
    }

    #[test]
    fn test_intersects_sets_object_on_the_intersection() {
        let r = Ray::new(Tuple4::point(0.0, 0.0, -5.0), Tuple4::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();

        let xs = s.intersect(&r);

        assert_eq!(xs.len(), 2);
        assert!(ptr::eq(xs[0].sphere, &s));
    }

    #[test]
    fn test_the_hit_when_all_intersections_have_positive_t() {
        let s = Sphere::new();
        let i1 = SphereIntersection::new(1.0, &s);
        let i2 = SphereIntersection::new(2.0, &s);
        let xs = SphereIntersections::new(vec![i1, i2]);

        let i = xs.hit().unwrap();

        assert!(ptr::eq(i, &xs.intersections[0]))
    }

    #[test]
    fn test_the_hit_when_some_intersections_have_negative_t() {
        let s = Sphere::new();
        let i1 = SphereIntersection::new(-1.0, &s);
        let i2 = SphereIntersection::new(1.0, &s);
        let xs = SphereIntersections::new(vec![i1, i2]);

        let i = xs.hit().unwrap();

        assert!(ptr::eq(i, &xs.intersections[1]))
    }

    #[test]
    fn test_the_hit_when_all_intersections_have_negative_t() {
        let s = Sphere::new();
        let i1 = SphereIntersection::new(-2.0, &s);
        let i2 = SphereIntersection::new(-1.0, &s);
        let xs = SphereIntersections::new(vec![i1, i2]);

        let i = xs.hit();

        assert!(i.is_none());
    }

    #[test]
    fn test_the_hit_is_always_the_lowest_nonnegative_intersection() {
        let s = Sphere::new();
        let i1 = SphereIntersection::new(5.0, &s);
        let i2 = SphereIntersection::new(7.0, &s);
        let i3 = SphereIntersection::new(-3.0, &s);
        let i4 = SphereIntersection::new(2.0, &s);
        let xs = SphereIntersections::new(vec![i1, i2, i3, i4]);

        let i = xs.hit().unwrap();

        assert!(ptr::eq(i, &xs.intersections[3]));
    }
}
