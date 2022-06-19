use crate::tuple::Tuple4;

#[derive(Debug)]
pub struct PointLight {
    position: Tuple4,
    intensity: Tuple4,
}

impl PointLight {
    pub fn new(position: Tuple4, intensity: Tuple4) -> Self {
        PointLight {
            position,
            intensity,
        }
    }

    pub fn position(&self) -> &Tuple4 {
        &self.position
    }

    pub fn intensity(&self) -> &Tuple4 {
        &self.intensity
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_light_has_position_and_intensity() {
        let intensity = Tuple4::point(1.0, 1.0, 1.0);
        let position = Tuple4::point(0.0, 0.0, 0.0);

        let point_light = PointLight::new(position, intensity);

        assert_eq!(point_light.intensity, intensity);
        assert_eq!(point_light.position, position);
    }
}
