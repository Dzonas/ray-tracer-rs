use std::io::{self, Write};

pub trait RGB {
    fn r(&self) -> u8;
    fn g(&self) -> u8;
    fn b(&self) -> u8;
}

pub trait PPM<T> {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn colors(&self) -> &[T];
}

pub struct PPMEncoder<'a, T: Write> {
    writer: &'a mut T,
}

impl<'a, T: Write> PPMEncoder<'a, T> {
    const PPM_HEADER: &'static str = "P3";
    const PPM_MAX: &'static str = "255";

    pub fn new(writer: &'a mut T) -> Self {
        PPMEncoder { writer }
    }

    fn write_header(&mut self, width: usize, height: usize) -> io::Result<()> {
        let header = format!(
            "{}\n{} {}\n{}\n",
            Self::PPM_HEADER,
            width,
            height,
            Self::PPM_MAX
        );
        self.writer.write_all(header.as_bytes())
    }

    fn write_data<H: RGB>(&mut self, width: usize, colors: &[H]) -> io::Result<()> {
        for (i, color) in colors.iter().enumerate() {
            let s = if (i + 1) % width == 0 {
                format!("{} {} {}\n", color.r(), color.g(), color.b())
            } else {
                format!("{} {} {} ", color.r(), color.g(), color.b())
            };

            self.writer.write_all(s.as_bytes())?;
        }

        Ok(())
    }

    pub fn write<H: RGB, P: PPM<H>>(&mut self, ppm: &P) -> io::Result<()> {
        self.write_header(ppm.width(), ppm.height())?;
        self.write_data(ppm.width(), ppm.colors())?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Copy)]
    struct Tuple3(u8, u8, u8);

    impl RGB for Tuple3 {
        fn r(&self) -> u8 {
            self.0
        }

        fn g(&self) -> u8 {
            self.1
        }

        fn b(&self) -> u8 {
            self.2
        }
    }

    struct Canvas {
        width: usize,
        height: usize,
        colors: Vec<Tuple3>,
    }

    impl PPM<Tuple3> for Canvas {
        fn width(&self) -> usize {
            self.width
        }

        fn height(&self) -> usize {
            self.height
        }

        fn colors(&self) -> &[Tuple3] {
            &self.colors
        }
    }

    #[test]
    fn test_to_ppm_header() {
        let c = Canvas {
            width: 5,
            height: 3,
            colors: Vec::new(),
        };
        let mut buffer = Vec::new();
        let mut encoder = PPMEncoder::new(&mut buffer);

        encoder.write(&c).unwrap();

        let s = String::from_utf8(buffer).unwrap();
        assert_eq!("P3\n5 3\n255\n", &s);
    }

    #[test]
    fn test_to_ppm_pixel_data() {
        let mut c = Canvas {
            width: 5,
            height: 3,
            colors: vec![Tuple3(0, 0, 0); 15],
        };
        c.colors[0] = Tuple3(255, 0, 0);
        c.colors[7] = Tuple3(0, 128, 0);
        c.colors[14] = Tuple3(0, 0, 255);
        let mut buffer = Vec::new();
        let mut encoder = PPMEncoder::new(&mut buffer);

        encoder.write(&c).unwrap();

        let s = String::from_utf8(buffer).unwrap();
        let mut l = s.lines().skip(3);
        assert_eq!(Some("255 0 0 0 0 0 0 0 0 0 0 0 0 0 0"), l.next());
        assert_eq!(Some("0 0 0 0 0 0 0 128 0 0 0 0 0 0 0"), l.next());
        assert_eq!(Some("0 0 0 0 0 0 0 0 0 0 0 0 0 0 255"), l.next());
    }
}
