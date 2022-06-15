use std::io;

use ray_tracer_rs::{canvas::Canvas, ppm::PPMEncoder, ray::Ray, sphere::Sphere, tuple::Tuple4};

const WALL_Z: f64 = 10.0;
const WALL_SIZE: f64 = 7.0;
const CANVAS_PIXELS: usize = 800;
const PIXEL_SIZE: f64 = WALL_SIZE / CANVAS_PIXELS as f64;
const HALF: f64 = WALL_SIZE / 2.0;

fn main() -> io::Result<()> {
    let mut canvas = Canvas::new(CANVAS_PIXELS, CANVAS_PIXELS);
    let ray_origin = Tuple4::point(0.0, 0.0, -5.0);
    let color = Tuple4::point(1.0, 0.0, 0.0);
    let sphere = Sphere::new();

    for y in 0..CANVAS_PIXELS {
        let world_y = -HALF + PIXEL_SIZE * y as f64;
        for x in 0..CANVAS_PIXELS {
            let world_x = -HALF + PIXEL_SIZE * x as f64;
            let pos = Tuple4::point(world_x, world_y, WALL_Z);
            let ray = Ray::new(ray_origin, (pos - ray_origin).normalize());
            let xs = sphere.intersect(&ray);

            if xs.hit().is_some() {
                canvas.put_pixel(color, (x as usize, y as usize));
            }
        }
    }

    let stdout = io::stdout();
    let mut handle = stdout.lock();
    let mut encoder = PPMEncoder::new(&mut handle);

    encoder.write(&canvas)
}
