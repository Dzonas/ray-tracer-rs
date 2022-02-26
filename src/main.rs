use std::f64::consts::PI;

use ray_tracer_rs::{canvas::Canvas, matrix::Matrix4x4, tuple::Tuple4};

const FULL_CIRCLE: f64 = 2.0 * PI;
const N: usize = 12;
const WIDTH: usize = 48;
const HEIGHT: usize = 48;

fn main() {
    let color = Tuple4::point(1.0, 1.0, 1.0);
    let mut canvas = Canvas::new(WIDTH, HEIGHT);
    let c = Tuple4::point(0.0, 0.0, 0.0);
    let scale = Matrix4x4::scaling(12.0, 12.0, 12.0);
    let translation = Matrix4x4::translation(0.0, 0.0, 1.0);

    for i in 0..N {
        let angle = FULL_CIRCLE / (N as f64) * i as f64;
        let rotation = Matrix4x4::rotation_y(angle);
        let transformation = scale * rotation * translation;
        let p = transformation * c;

        let x = (p.x + (WIDTH as f64) / 2.0) as usize;
        let y = (p.z + (HEIGHT as f64) / 2.0) as usize;

        canvas.put_pixel(color, (x, y));
    }

    println!("{}", canvas.to_ppm());
}
