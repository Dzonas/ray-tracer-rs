use crate::tuple::Tuple;

pub struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<Tuple>,
}

impl Canvas {
    const PPM_HEADER: &'static str = "P3";

    pub fn new(width: usize, height: usize) -> Canvas {
        let pixels = vec![Tuple::new(0.0, 0.0, 0.0, 0.0); width * height];

        Canvas {
            width,
            height,
            pixels,
        }
    }

    fn to_index(&self, pos: (usize, usize)) -> usize {
        let (x, y) = pos;

        y * self.width + x
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn put_pixel(&mut self, pixel: Tuple, at: (usize, usize)) {
        let i = self.to_index(at);
        self.pixels[i] = pixel;
    }

    pub fn get_pixel(&self, at: (usize, usize)) -> &Tuple {
        let i = self.to_index(at);
        &self.pixels[i]
    }

    pub fn to_ppm(&self) -> String {
        let header = format!(
            "{}\n{} {}\n{}\n",
            Canvas::PPM_HEADER,
            self.width,
            self.height,
            Tuple::PPM_MAX
        );

        let ppm_pixels: String = self
            .pixels
            .iter()
            .map(|t| t.to_ppm())
            .enumerate()
            .map(|(i, s)| {
                if (i + 1) % self.width == 0 {
                    s + "\n"
                } else {
                    s + " "
                }
            })
            .collect();

        header + &ppm_pixels
    }
}

impl IntoIterator for Canvas {
    type Item = Tuple;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.pixels.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creating_new_canvas() {
        let canvas = Canvas::new(10, 20);

        let width = canvas.get_width();
        let height = canvas.get_height();
        let data: Vec<_> = canvas.into_iter().collect();

        assert_eq!(width, 10);
        assert_eq!(height, 20);
        assert_eq!(data, vec![Tuple::new(0.0, 0.0, 0.0, 0.0); 200]);
    }

    #[test]
    fn test_putting_pixel() {
        let mut canvas = Canvas::new(10, 20);
        let pixel = Tuple::new(1.0, 2.0, 3.0, 0.0);

        canvas.put_pixel(pixel, (2, 3));

        assert_eq!(*canvas.get_pixel((2, 3)), pixel);
    }

    #[test]
    fn test_to_ppm_header() {
        let c = Canvas::new(5, 3);

        let s = c.to_ppm();

        let mut l = s.lines();
        assert_eq!(Some("P3"), l.next());
        assert_eq!(Some("5 3"), l.next());
        assert_eq!(Some("255"), l.next());
    }

    #[test]
    fn test_to_ppm_pixel_data() {
        let mut c = Canvas::new(5, 3);
        c.put_pixel(Tuple::new(1.5, 0.0, 0.0, 0.0), (0, 0));
        c.put_pixel(Tuple::new(0.0, 0.5, 0.0, 0.0), (2, 1));
        c.put_pixel(Tuple::new(-0.5, 0.0, 1.0, 0.0), (4, 2));

        let s = c.to_ppm();

        let mut l = s.lines().skip(3);
        assert_eq!(Some("255 0 0 0 0 0 0 0 0 0 0 0 0 0 0"), l.next());
        assert_eq!(Some("0 0 0 0 0 0 0 128 0 0 0 0 0 0 0"), l.next());
        assert_eq!(Some("0 0 0 0 0 0 0 0 0 0 0 0 0 0 255"), l.next());
    }
}
