use crate::{lights::PointLight, tuple::Tuple4};

#[derive(Debug, PartialEq, Clone)]
pub struct Material {
    pub color: Tuple4,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
}

impl Material {
    pub fn new(color: Tuple4, ambient: f64, diffuse: f64, specular: f64, shininess: f64) -> Self {
        Material {
            color,
            ambient,
            diffuse,
            specular,
            shininess,
        }
    }

    pub fn lighting(
        &self,
        light: PointLight,
        point: Tuple4,
        eyev: Tuple4,
        normalv: Tuple4,
    ) -> Tuple4 {
        let effective_color = Tuple4::point(
            self.color.x * light.intensity().x,
            self.color.y * light.intensity().y,
            self.color.z * light.intensity().z,
        );
        let lightv = (*light.position() - point).normalize();
        let ambient = effective_color * self.ambient;

        let light_dot_normal = lightv.dot(&normalv);
        let diffuse;
        let specular;
        if light_dot_normal < 0.0 {
            diffuse = Tuple4::point(0.0, 0.0, 0.0);
            specular = Tuple4::point(0.0, 0.0, 0.0);
        } else {
            diffuse = effective_color * self.diffuse * light_dot_normal;

            let reflectv = (-1.0 * lightv).reflect(normalv);
            let reflect_dot_eye = reflectv.dot(&eyev);

            if reflect_dot_eye <= 0.0 {
                specular = Tuple4::point(0.0, 0.0, 0.0);
            } else {
                let factor = reflect_dot_eye.powf(self.shininess);
                specular = *light.intensity() * self.specular * factor;
            }
        }

        let mut out = ambient + diffuse + specular;
        out.w = 1.0;

        out
    }
}

impl Default for Material {
    fn default() -> Self {
        Material {
            color: Tuple4::point(1.0, 1.0, 1.0),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{lights::PointLight, tuple::Tuple4};

    use super::Material;

    const EPSILON: f64 = 1e-6;

    fn equal(a: f64, b: f64) -> bool {
        (a - b).abs() < EPSILON
    }

    #[test]
    fn test_default_material() {
        let m = Material::default();

        assert_eq!(m.color, Tuple4::point(1.0, 1.0, 1.0));
        assert_eq!(m.ambient, 0.1);
        assert_eq!(m.diffuse, 0.9);
        assert_eq!(m.specular, 0.9);
        assert_eq!(m.shininess, 200.0);
    }

    #[test]
    fn test_lighting_with_eye_between_the_light_and_the_surface() {
        let m = Material::default();
        let position = Tuple4::point(0.0, 0.0, 0.0);
        let eyev = Tuple4::vector(0.0, 0.0, -1.0);
        let normalv = Tuple4::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple4::point(0.0, 0.0, -10.0), Tuple4::point(1.0, 1.0, 1.0));

        let result = m.lighting(light, position, eyev, normalv);

        assert_eq!(result, Tuple4::point(1.9, 1.9, 1.9));
    }

    #[test]
    fn test_lighting_with_eye_between_the_light_and_the_surface_eye_offset_45_deg() {
        let m = Material::default();
        let position = Tuple4::point(0.0, 0.0, 0.0);
        let eyev = Tuple4::vector(0.0, 2.0_f64.sqrt(), -(2.0_f64.sqrt()) / 2.0);
        let normalv = Tuple4::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple4::point(0.0, 0.0, -10.0), Tuple4::point(1.0, 1.0, 1.0));

        let result = m.lighting(light, position, eyev, normalv);

        assert_eq!(result, Tuple4::point(1.0, 1.0, 1.0));
    }

    #[test]
    fn test_lighting_with_eye_opposite_surface_light_offset_45() {
        let m = Material::default();
        let position = Tuple4::point(0.0, 0.0, 0.0);
        let eyev = Tuple4::vector(0.0, 0.0, -1.0);
        let normalv = Tuple4::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(
            Tuple4::point(0.0, 10.0, -10.0),
            Tuple4::point(1.0, 1.0, 1.0),
        );

        let result = m.lighting(light, position, eyev, normalv);

        assert!(equal(result.x, 0.736396));
        assert!(equal(result.y, 0.736396));
        assert!(equal(result.z, 0.736396));
    }

    #[test]
    fn test_lighting_with_eye_in_the_path_of_the_reflection_vector() {
        let m = Material::default();
        let position = Tuple4::point(0.0, 0.0, 0.0);
        let eyev = Tuple4::vector(0.0, -(2.0_f64.sqrt() / 2.0), -(2.0_f64.sqrt()) / 2.0);
        let normalv = Tuple4::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(
            Tuple4::point(0.0, 10.0, -10.0),
            Tuple4::point(1.0, 1.0, 1.0),
        );

        let result = m.lighting(light, position, eyev, normalv);

        assert!(equal(result.x, 1.636396));
        assert!(equal(result.y, 1.636396));
        assert!(equal(result.z, 1.636396));
    }

    #[test]
    fn test_lighting_with_the_light_behind_the_surface() {
        let m = Material::default();
        let position = Tuple4::point(0.0, 0.0, 0.0);
        let eyev = Tuple4::vector(0.0, 0.0, -1.0);
        let normalv = Tuple4::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple4::point(0.0, 0.0, 10.0), Tuple4::point(1.0, 1.0, 1.0));

        let result = m.lighting(light, position, eyev, normalv);

        assert_eq!(result, Tuple4::point(0.1, 0.1, 0.1));
    }
}
