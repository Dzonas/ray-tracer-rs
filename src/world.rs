use crate::color::Color;
use crate::materials::Material;
use crate::matrix::Matrix4x4;
use crate::ray::Ray;
use crate::sphere::{SphereIntersections, SphereIntersection};
use crate::tuple::Tuple4;
use crate::{lights::PointLight, sphere::Sphere};

pub struct World {
    objects: Vec<Sphere>,
    light: Option<PointLight>,
}

impl World {
    pub fn new() -> World {
        let objects = Vec::new();
        let light = None;

        World { objects, light }
    }

    pub fn objects(&self) -> &Vec<Sphere> {
        &self.objects
    }

    pub fn light(&self) -> Option<&PointLight> {
        self.light.as_ref()
    }

    pub fn intersect(&self, r: &Ray) -> SphereIntersections {
        let mut all_intersections = SphereIntersections::new(Vec::new());

        for object in self.objects.iter() {
            let intersections = object.intersect(r);
            all_intersections.append(intersections);
        }

        all_intersections.sort_by_t_ascending();

        all_intersections
    }

    fn shade_hit(&self, comps: &PreparedComputations) -> Option<Color> {
        if let Some(point_light) = self.light {
            Some(comps.object.get_material().lighting(point_light, comps.point, comps.eyev, comps.normalv))
        } else {
            None
        }
    }

    fn color_at(&self, ray: &Ray) -> Color {
        if let Some(intersection) = self.intersect(ray).hit() {
            let comps = PreparedComputations::new(intersection, ray);
            self.shade_hit(&comps).unwrap_or(Color::new(0.0, 0.0, 0.0))
        } else {
            Color::new(0.0, 0.0, 0.0)
        }
    }
}

impl Default for World {
    fn default() -> Self {
        let light = Some(PointLight::new(
            Tuple4::point(-10.0, 10.0, -10.0),
            Color::new(1.0, 1.0, 1.0),
        ));

        let mut s1 = Sphere::new();
        let material = Material {
            color: Color::new(0.8, 1.0, 0.6),
            diffuse: 0.7,
            specular: 0.2,
            ..Default::default()
        };
        s1.set_material(material);

        let mut s2 = Sphere::new();
        let transform = Matrix4x4::scaling(0.5, 0.5, 0.5);
        s2.set_transform(transform);

        let objects = vec![s1, s2];

        World { objects, light }
    }
}

struct PreparedComputations<'a> {
    pub t: f64,
    pub object: &'a Sphere,
    pub point: Tuple4,
    pub eyev: Tuple4,
    pub normalv: Tuple4,
    pub inside: bool
}

impl PreparedComputations<'_> {
    pub fn new<'a>(intersection: &'a SphereIntersection, ray: &Ray) -> PreparedComputations<'a> {
        let t = intersection.t;
        let object = intersection.sphere;
        let point = ray.position(t);
        let eyev = -1.0 * ray.direction;
        let mut normalv = intersection.sphere.normal_at(point);
        let inside;

        if normalv.dot(&eyev) < 0.0 {
            inside = true;
            normalv = -1.0 * normalv;
        } else {
            inside = false;
        }

        PreparedComputations { t, object, point, eyev, normalv, inside }
    }
}

#[cfg(test)]
mod tests {
    use crate::{materials::Material, matrix::Matrix4x4, ray::Ray, tuple::Tuple4};
    use std::ptr;

    use super::*;

    #[test]
    fn test_new_world_is_empty() {
        let w = World::new();

        assert!(w.objects().is_empty());
        assert!(w.light().is_none());
    }

    #[test]
    fn test_default_world() {
        let light = PointLight::new(Tuple4::point(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));

        let mut s1 = Sphere::new();
        let mut m = Material::default();
        m.color = Color::new(0.8, 1.0, 0.6);
        m.diffuse = 0.7;
        m.specular = 0.2;
        s1.set_material(m);

        let mut s2 = Sphere::new();
        let transform = Matrix4x4::scaling(0.5, 0.5, 0.5);
        s2.set_transform(transform);

        let w = World::default();

        assert_eq!(w.light.unwrap(), light);
        assert!(w.objects().contains(&s1));
        assert!(w.objects().contains(&s2));
    }

    #[test]
    fn test_intersect_a_world_with_a_ray() {
        let w = World::default();
        let r = Ray::new(Tuple4::point(0.0, 0.0, -5.0), Tuple4::vector(0.0, 0.0, 1.0));

        let xs = w.intersect(&r);

        assert_eq!(xs.len(), 4);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 4.5);
        assert_eq!(xs[2].t, 5.5);
        assert_eq!(xs[3].t, 6.0);
    }

    #[test]
    fn test_precomputing_the_state_of_an_intersection() {
        let ray = Ray::new(Tuple4::point(0.0, 0.0, -5.0), Tuple4::vector(0.0, 0.0, 1.0));
        let shape = Sphere::new();
        let i = SphereIntersection::new(4.0, &shape);

        let comps = PreparedComputations::new(&i, &ray);

        assert_eq!(comps.t, i.t);
        assert!(ptr::eq(comps.object, i.sphere));
        assert_eq!(comps.point, Tuple4::point(0.0, 0.0, -1.0));
        assert_eq!(comps.eyev, Tuple4::vector(0.0, 0.0, -1.0));
        assert_eq!(comps.eyev, Tuple4::vector(0.0, 0.0, -1.0));
    }

    #[test]
    fn test_hit_when_an_intersection_occurs_on_the_outside() {
        let ray = Ray::new(Tuple4::point(0.0, 0.0, -5.0), Tuple4::vector(0.0, 0.0, 1.0));
        let shape = Sphere::new();
        let i = SphereIntersection::new(4.0, &shape);

        let comps = PreparedComputations::new(&i, &ray);

        assert!(!comps.inside);
    }
    
    #[test]
    fn test_hit_when_an_intersection_occurs_on_the_inside() {
        let ray = Ray::new(Tuple4::point(0.0, 0.0, 0.0), Tuple4::vector(0.0, 0.0, 1.0));
        let shape = Sphere::new();
        let i = SphereIntersection::new(1.0, &shape);

        let comps = PreparedComputations::new(&i, &ray);

        assert_eq!(comps.point, Tuple4::point(0.0, 0.0, 1.0));
        assert_eq!(comps.eyev, Tuple4::vector(0.0, 0.0, -1.0));
        assert!(comps.inside);
        assert_eq!(comps.normalv, Tuple4::vector(0.0, 0.0, -1.0));
    }

    #[test]
    fn test_shading_an_intersection() {
        let world = World::default();
        let ray = Ray::new(Tuple4::point(0.0, 0.0, -5.0), Tuple4::vector(0.0, 0.0, 1.0));
        let shape = &world.objects[0];
        let intersection = SphereIntersection::new(4.0, shape);
        let comps = PreparedComputations::new(&intersection, &ray);

        let color = world.shade_hit(&comps).unwrap();

        assert!((color.r - 0.38066).abs() < 1e-5);
        assert!((color.g - 0.47583).abs() < 1e-5);
        assert!((color.b - 0.2855).abs() < 1e-5);
    }

    #[test]
    fn test_shading_an_intersection_from_the_inside() {
        let mut world = World::default();
        world.light = Some(PointLight::new(Tuple4::point(0.0, 0.25, 0.0), Color::new(1.0, 1.0, 1.0)));
        let ray = Ray::new(Tuple4::point(0.0, 0.0, 0.0), Tuple4::vector(0.0, 0.0, 1.0));
        let shape = &world.objects[1];
        let intersection = SphereIntersection::new(0.5, shape);
        let comps = PreparedComputations::new(&intersection, &ray);

        let color = world.shade_hit(&comps).unwrap();

        assert!((color.r - 0.90498).abs() < 1e-5);
        assert!((color.g - 0.90498).abs() < 1e-5);
        assert!((color.b - 0.90498).abs() < 1e-5);
    }

    #[test]
    fn test_color_when_a_ray_misses() {
        let world = World::default();
        let ray = Ray::new(Tuple4::point(0.0, 0.0, -5.0), Tuple4::vector(0.0, 1.0, 0.0));

        let color = world.color_at(&ray);

        assert_eq!(color, Color::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn test_color_when_a_ray_hits() {
        let world = World::default();
        let ray = Ray::new(Tuple4::point(0.0, 0.0, -5.0), Tuple4::vector(0.0, 0.0, 1.0));

        let color = world.color_at(&ray);

        assert!((color.r - 0.38066).abs() < 1e-5);
        assert!((color.g - 0.47583).abs() < 1e-5);
        assert!((color.b - 0.2855).abs() < 1e-5);
    }

    #[test]
    fn test_color_with_an_intersection_behind_the_ray() {
        let mut world = World::default();
        let outer = &mut world.objects[0];
        let mut outer_material = outer.get_material().clone();
        outer_material.ambient = 1.0;
        outer.set_material(outer_material);
        let inner = &mut world.objects[1];
        let mut inner_material = inner.get_material().clone();
        inner_material.ambient = 1.0;
        inner.set_material(inner_material);

        let ray = Ray::new(Tuple4::point(0.0, 0.0, 0.75), Tuple4::vector(0.0, 0.0, -1.0));
        
        let color = world.color_at(&ray);

        assert_eq!(color, world.objects[1].get_material().color);
    }
}
