use crate::{matrix::Matrix4x4, tuple::Tuple4};

#[expect(dead_code)]
fn view_transform(from: Tuple4, to: Tuple4, up: Tuple4) -> Matrix4x4 {
    let forward = (to - from).normalize();
    let upn = up.normalize();
    let left = forward.cross(upn);
    let true_up = left.cross(forward);

    let orientation = Matrix4x4::new([
        left.x, left.y, left.z, 0.0, true_up.x, true_up.y, true_up.z, 0.0, -forward.x, -forward.y,
        -forward.z, 0.0, 0.0, 0.0, 0.0, 1.0,
    ]);

    orientation * Matrix4x4::translation(-from.x, -from.y, -from.z)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transformation_matrix_for_the_default_orientation() {
        let from = Tuple4::point(0, 0, 0);
        let to = Tuple4::point(0, 0, -1);
        let up = Tuple4::vector(0, 1, 0);

        let t = view_transform(from, to, up);

        assert_eq!(t, Matrix4x4::identity());
    }

    #[test]
    fn test_a_view_transformation_matrix_looking_in_positive_z_direction() {
        let from = Tuple4::point(0, 0, 0);
        let to = Tuple4::point(0, 0, 1);
        let up = Tuple4::vector(0, 1, 0);

        let t = view_transform(from, to, up);

        assert_eq!(t, Matrix4x4::scaling(-1.0, 1.0, -1.0));
    }

    #[test]
    fn test_the_view_transformation_moves_the_world() {
        let from = Tuple4::point(0, 0, 8);
        let to = Tuple4::point(0, 0, 0);
        let up = Tuple4::vector(0, 1, 0);

        let t = view_transform(from, to, up);

        assert_eq!(t, Matrix4x4::translation(0.0, 0.0, -8.0));
    }

    #[test]
    fn test_an_arbitrary_view_transformation() {
        let from = Tuple4::point(1, 3, 2);
        let to = Tuple4::point(4, -2, 8);
        let up = Tuple4::vector(1, 1, 0);

        let t = view_transform(from, to, up);

        assert!((t.get(0, 0) - -0.50709).abs() < 1e-5);
        assert!((t.get(0, 1) - 0.50709).abs() < 1e-5);
        assert!((t.get(0, 2) - 0.67612).abs() < 1e-5);
        assert!((t.get(0, 3) - -2.36643).abs() < 1e-5);
        assert!((t.get(1, 0) - 0.76772).abs() < 1e-5);
        assert!((t.get(1, 1) - 0.60609).abs() < 1e-5);
        assert!((t.get(1, 2) - 0.12122).abs() < 1e-5);
        assert!((t.get(1, 3) - -2.82843).abs() < 1e-5);
        assert!((t.get(2, 0) - -0.35857).abs() < 1e-5);
        assert!((t.get(2, 1) - 0.59761).abs() < 1e-5);
        assert!((t.get(2, 2) - -0.71714).abs() < 1e-5);
        assert!(t.get(2, 3).abs() < 1e-5);
        assert!(t.get(3, 0).abs() < 1e-5);
        assert!(t.get(3, 1).abs() < 1e-5);
        assert!(t.get(3, 2).abs() < 1e-5);
        assert!((t.get(3, 3).abs() - 1.0) < 1e-5);
    }
}
