use crate::color::Color;
use crate::materials::Material;
use crate::matrix::Matrix4x4;
use crate::ray::Ray;
use crate::sphere::SphereIntersections;
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

    pub fn intersect(&self, r: Ray) -> SphereIntersections {
        let mut all_intersections = SphereIntersections::new(Vec::new());

        for object in self.objects.iter() {
            let intersections = object.intersect(&r);
            all_intersections.append(intersections);
        }

        all_intersections.sort_by_t_ascending();

        all_intersections
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

#[cfg(test)]
mod tests {
    use crate::{materials::Material, matrix::Matrix4x4, ray::Ray, tuple::Tuple4};

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

        let xs = w.intersect(r);

        assert_eq!(xs.len(), 4);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 4.5);
        assert_eq!(xs[2].t, 5.5);
        assert_eq!(xs[3].t, 6.0);
    }
}
