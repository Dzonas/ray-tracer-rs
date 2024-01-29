use std::io;

use ray_tracer_rs::{
    canvas::Canvas, color::Color, lights::PointLight, materials::Material, ppm::PPMEncoder,
    ray::Ray, sphere::Sphere, tuple::Tuple4,
};

const WALL_Z: f64 = 10.0;
const WALL_SIZE: f64 = 7.0;
const CANVAS_PIXELS: usize = 800;
const PIXEL_SIZE: f64 = WALL_SIZE / CANVAS_PIXELS as f64;
const HALF: f64 = WALL_SIZE / 2.0;

fn main() -> io::Result<()> {
    let mut canvas = Canvas::new(CANVAS_PIXELS, CANVAS_PIXELS);
    let ray_origin = Tuple4::point(0.0, 0.0, -5.0);
    let mut sphere = Sphere::new();
    let material = Material {
        color: Color::new(1.0, 0.2, 1.0),
        ..Default::default()
    };
    sphere.set_material(material);
    let light = PointLight::new(
        Tuple4::point(-10.0, -10.0, -10.0),
        Color::new(1.0, 1.0, 1.0),
    );

    for y in 0..CANVAS_PIXELS {
        let world_y = -HALF + PIXEL_SIZE * y as f64;
        for x in 0..CANVAS_PIXELS {
            let world_x = -HALF + PIXEL_SIZE * x as f64;
            let pos = Tuple4::point(world_x, world_y, WALL_Z);
            let ray = Ray::new(ray_origin, (pos - ray_origin).normalize());
            let xs = sphere.intersect(&ray);

            if let Some(hit) = xs.hit() {
                let point = ray.position(hit.t);
                let normal = hit.sphere.normal_at(point);
                let eye = -1.0 * ray.direction;
                let color = hit
                    .sphere
                    .get_material()
                    .lighting(light, point, eye, normal);
                canvas.put_pixel(color, (x, y));
            }
        }
    }

    let stdout = io::stdout();
    let mut handle = stdout.lock();
    let mut encoder = PPMEncoder::new(&mut handle);

    encoder.write(&canvas)
}
