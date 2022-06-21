use crate::{color::Color, ppm::PPM};

pub struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<Color>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        let pixels = vec![Color::new(0.0, 0.0, 0.0); width * height];

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

    pub fn put_pixel(&mut self, pixel: Color, at: (usize, usize)) {
        let i = self.to_index(at);
        self.pixels[i] = pixel;
    }

    pub fn get_pixel(&self, at: (usize, usize)) -> &Color {
        let i = self.to_index(at);
        &self.pixels[i]
    }
}

impl IntoIterator for Canvas {
    type Item = Color;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.pixels.into_iter()
    }
}

impl PPM<Color> for Canvas {
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn colors(&self) -> &[Color] {
        &self.pixels
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
        assert_eq!(data, vec![Color::new(0.0, 0.0, 0.0); 200]);
    }

    #[test]
    fn test_putting_pixel() {
        let mut canvas = Canvas::new(10, 20);
        let pixel = Color::new(1.0, 2.0, 3.0);

        canvas.put_pixel(pixel, (2, 3));

        assert_eq!(*canvas.get_pixel((2, 3)), pixel);
    }
}
