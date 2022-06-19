use std::ops::Index;

use crate::materials::Material;
use crate::matrix::Matrix4x4;
use crate::ray::Ray;
use crate::tuple::Tuple4;

#[allow(dead_code)]
pub struct Sphere {
    origin: Tuple4,
    radius: f64,
    transform: Matrix4x4,
    material: Material,
}

impl Sphere {
    pub fn new() -> Sphere {
        let origin = Tuple4::point(0.0, 0.0, 0.0);
        let radius = 1.0;
        let transform = Matrix4x4::identity();
        let material = Material::default();

        Sphere {
            origin,
            radius,
            transform,
            material,
        }
    }

    pub fn intersect(&self, ray: &Ray) -> SphereIntersections {
        let ray_transformation_matrix = self
            .transform
            .inverse()
            .expect("Can't inverse singular matrix");
        let transformed_ray = ray.transform(ray_transformation_matrix);

        let sphere_to_ray = transformed_ray.origin - self.origin;
        let a = transformed_ray.direction.dot(&transformed_ray.direction);
        let b = 2.0 * transformed_ray.direction.dot(&sphere_to_ray);
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

    pub fn set_transform(&mut self, m: Matrix4x4) {
        self.transform = m;
    }

    pub fn normal_at(&self, p: Tuple4) -> Tuple4 {
        let object_point = self.transform.inverse().unwrap() * p;
        let object_normal = object_point - Tuple4::point(0.0, 0.0, 0.0);
        let mut world_normal = self.transform.inverse().unwrap().transpose() * object_normal;
        world_normal.w = 0.0;
        world_normal.normalize()
    }

    pub fn set_material(&mut self, m: Material) {
        self.material = m;
    }

    pub fn get_material(&self) -> &Material {
        &self.material
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Self::new()
    }
}

pub struct SphereIntersection<'a> {
    pub t: f64,
    pub sphere: &'a Sphere,
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
    use std::f64::consts::PI;
    use std::ptr;

    use super::*;

    const EPSILON: f64 = 1e-6;

    fn equal(a: f64, b: f64) -> bool {
        (a - b).abs() < EPSILON
    }

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

    #[test]
    fn test_spheres_default_transformation_is_identity() {
        let s = Sphere::new();

        assert_eq!(s.transform, Matrix4x4::identity());
    }

    #[test]
    fn test_changing_spheres_transformation() {
        let mut s = Sphere::new();
        let t = Matrix4x4::translation(2.0, 3.0, 4.0);

        s.set_transform(t);

        assert_eq!(s.transform, t);
    }

    #[test]
    fn test_intersecting_a_scaled_sphere_with_a_ray() {
        let r = Ray::new(Tuple4::point(0.0, 0.0, -5.0), Tuple4::vector(0.0, 0.0, 1.0));
        let mut s = Sphere::new();
        s.set_transform(Matrix4x4::scaling(2.0, 2.0, 2.0));

        let xs = s.intersect(&r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 3.0);
        assert_eq!(xs[1].t, 7.0);
    }

    #[test]
    fn test_intersecting_a_translated_sphere_with_a_ray() {
        let r = Ray::new(Tuple4::point(0.0, 0.0, -5.0), Tuple4::vector(0.0, 0.0, 1.0));
        let mut s = Sphere::new();
        s.set_transform(Matrix4x4::translation(5.0, 0.0, 0.0));

        let xs = s.intersect(&r);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn test_normal_on_a_sphere_at_a_point_on_the_x_axis() {
        let s = Sphere::new();

        let n = s.normal_at(Tuple4::point(1.0, 0.0, 0.0));

        assert_eq!(n, Tuple4::vector(1.0, 0.0, 0.0));
    }

    #[test]
    fn test_normal_on_a_sphere_at_a_point_on_the_y_axis() {
        let s = Sphere::new();

        let n = s.normal_at(Tuple4::point(0.0, 1.0, 0.0));

        assert_eq!(n, Tuple4::vector(0.0, 1.0, 0.0));
    }
    #[test]
    fn test_normal_on_a_sphere_at_a_point_on_the_z_axis() {
        let s = Sphere::new();

        let n = s.normal_at(Tuple4::point(0.0, 0.0, 1.0));

        assert_eq!(n, Tuple4::vector(0.0, 0.0, 1.0));
    }

    #[test]
    fn test_normal_on_a_sphere_at_a_nonaxial_point() {
        let s = Sphere::new();

        let n = s.normal_at(Tuple4::point(
            3.0_f64.sqrt() / 3.0,
            3.0_f64.sqrt() / 3.0,
            3.0_f64.sqrt() / 3.0,
        ));

        assert_eq!(
            n,
            Tuple4::vector(
                3.0_f64.sqrt() / 3.0,
                3.0_f64.sqrt() / 3.0,
                3.0_f64.sqrt() / 3.0,
            )
        );
    }

    #[test]
    fn test_the_normal_is_a_normalized_vector() {
        let s = Sphere::new();

        let n = s.normal_at(Tuple4::point(
            3.0_f64.sqrt() / 3.0,
            3.0_f64.sqrt() / 3.0,
            3.0_f64.sqrt() / 3.0,
        ));

        assert_eq!(
            n,
            Tuple4::vector(
                3.0_f64.sqrt() / 3.0,
                3.0_f64.sqrt() / 3.0,
                3.0_f64.sqrt() / 3.0,
            )
            .normalize()
        );
    }

    #[test]
    fn test_computing_the_normal_on_a_translated_sphere() {
        let mut s = Sphere::new();
        s.set_transform(Matrix4x4::translation(0.0, 1.0, 0.0));

        let n = s.normal_at(Tuple4::point(0.0, 1.70711, -0.70711));

        assert_eq!(n.x, 0.0);
        assert!(equal(n.y, 0.707106));
        assert!(equal(n.z, -0.707106));
        assert!(n.is_vector());
    }

    #[test]
    fn test_computing_the_normal_on_a_transformed_sphere() {
        let mut s = Sphere::new();
        let m = Matrix4x4::scaling(1.0, 0.5, 1.0) * Matrix4x4::rotation_z(PI / 5.0);
        s.set_transform(m);

        let n = s.normal_at(Tuple4::point(
            0.0,
            2.0_f64.sqrt() / 2.0,
            -(2.0_f64.sqrt()) / 2.0,
        ));

        assert_eq!(n.x, 0.0);
        assert!(equal(n.y, 0.970142));
        assert!(equal(n.z, -0.242535));
        assert!(n.is_vector());
    }

    #[test]
    fn test_sphere_has_default_material() {
        let s = Sphere::new();

        assert_eq!(s.material, Material::default());
    }

    #[test]
    fn test_sphere_may_be_assigned_a_material() {
        let mut s = Sphere::new();
        let mut m = Material::default();
        m.ambient = 1.0;

        s.set_material(m.clone());

        assert_eq!(s.material, m);
    }
}
