extern crate png;

use std::num::Float;
use std::slice::Items;

use png::{Image, RGB8};

#[deriving(Show)]
struct HSL {
    hue: f32,
    sat: f32,
    lum: f32,
}

impl HSL {
    pub fn new(hue: f32, sat: f32, lum: f32) -> HSL {
        HSL { hue: hue, sat: sat, lum: lum }
    }

    // http://www.w3.org/TR/css3-color/#hsl-color
    pub fn rgb(&self) -> RGB {
        let hue = self.hue / 360.0;
        let sat = self.sat / 100.0;
        let lum = self.lum / 100.0;

        let b =
            if lum <= 0.5 {
                lum * (sat + 1.0)
            } else {
                lum + sat - lum * sat
            };
        let a = lum * 2.0 - b;

        let r = HSL::hue_to_rgb(a, b, hue + 1.0/3.0);
        let g = HSL::hue_to_rgb(a, b, hue);
        let b = HSL::hue_to_rgb(a, b, hue - 1.0/3.0);

        RGB::new(
            (r * 255.0).round() as u8,
            (g * 255.0).round() as u8,
            (b * 255.0).round() as u8)
    }

    fn hue_to_rgb(a: f32, b: f32, hue: f32) -> f32 {
        let h =
            if hue < 0.0 {
                hue + 1.0
            } else if hue > 1.0 {
                hue - 1.0
            } else {
                hue
            };
        if h < 1.0/6.0 { return a + (b - a) * 6.0 * h; }
        if h < 1.0/2.0 { return b; }
        if h < 2.0/3.0 { return a + (b - a) * (2.0/3.0 - h) * 6.0; }
        a
    }
}

#[deriving(Show)]
struct RGB {
    red: u8,
    green: u8,
    blue: u8,
}

impl RGB {
    pub fn new(red: u8, green: u8, blue: u8) -> RGB {
        RGB { red: red, green: green, blue: blue }
    }
}

struct Canvas<'a> {
    width: uint,
    height: uint,
    pixels: Vec<Vec<&'a RGB>>,
}

impl<'a> Canvas<'a> {
    pub fn new(width: uint, height: uint, background: &'a RGB) -> Canvas<'a> {
        let pixels = Vec::from_fn(height, |_| Vec::from_elem(width, background));
        Canvas {
            width: width,
            height: height,
            pixels: pixels,
        }
    }

    pub fn rect(&mut self, x0: uint, y0: uint, x1: uint, y1: uint, color: &'a RGB) {
        for y in range(y0, y1) {
            for x in range(x0, x1) {
                self.pixels[y][x] = color;
            }
        }
    }

    pub fn image(&self) -> Image {
        let mut pixels = Vec::with_capacity(self.width * self.height * 3);
        for row in self.pixels.iter() {
            for color in row.iter() {
                pixels.push(color.red);
                pixels.push(color.green);
                pixels.push(color.blue);
            }
        }

        Image {
            width: self.width as u32,
            height: self.height as u32,
            pixels: RGB8(pixels),
        }
    }
}

struct Nibbler<'a> {
    byte: Option<u8>,
    bytes: &'a mut Items<'a, u8>,
}

impl<'a> Nibbler<'a> {
    pub fn new(bytes: &'a mut Items<'a, u8>) -> Nibbler<'a> {
        Nibbler { bytes: bytes, byte: None }
    }
}

impl<'a> Iterator<u8> for Nibbler<'a> {
    fn next(&mut self) -> Option<u8> {
        match self.byte {
            Some(value) => {
                self.byte = None;
                Some(value)
            },
            None => {
                match self.bytes.next() {
                    Some(value) => {
                        let hi = *value & 0xf0;
                        let lo = *value & 0x0f;
                        self.byte = Some(lo);
                        Some(hi >> 4)
                    },
                    None => None
                }
            },
        }
    }
}

pub struct Identicon {
    source: Vec<u8>,
    size: uint,
}

impl Identicon {
    pub fn new(source: Vec<u8>) -> Identicon {
        Identicon { source: source, size: 420 }
    }

    // https://processing.org/reference/map_.html
    fn map(value: u32, vmin: u32, vmax: u32, dmin: u32, dmax: u32) -> f32 {
        ((value - vmin) * (dmax - dmin)) as f32 / ((vmax - vmin) + dmin) as f32
    }

    fn foreground(&self) -> RGB {
        // Use last 28 bits to determine HSL values.
        let h1 = (self.source[12] as u16 & 0x0f) << 8;
        let h2 = self.source[13] as u16;

        let h = (h1 | h2) as u32;
        let s = self.source[14] as u32;
        let l = self.source[15] as u32;

        let hue = Identicon::map(h, 0, 4095, 0, 360);
        let sat = Identicon::map(s, 0, 255, 0, 20);
        let lum = Identicon::map(l, 0, 255, 0, 20);

        HSL::new(hue, 65.0 - sat, 75.0 - lum).rgb()
    }

    pub fn image(&self) -> Image {
        let pixel_size = 70;
        let sprite_size = 5;
        let inner_size = sprite_size * pixel_size;

        let background = RGB::new(240, 240, 240);
        let foreground = self.foreground();

        let mut canvas = Canvas::new(self.size, self.size, &background);

        let half_axis = (sprite_size - 1) / 2;
        let margin: int = pixel_size / 2;

        let mut iter = &mut self.source.iter();
        let mut nibbles = Nibbler::new(iter);
        let mut x = half_axis * pixel_size;

        while x >= 0 {
            let mut y = 0;
            while y < inner_size {
                if nibbles.next().unwrap() % 2 == 0 {
                    canvas.rect(
                        (x + margin) as uint,
                        (y + margin) as uint,
                        (x + pixel_size + margin) as uint,
                        (y + pixel_size + margin) as uint,
                        &foreground);

                    // Mirror blocks across axis.
                    if x != half_axis * pixel_size {
                        let x_start: int = 2 * half_axis * pixel_size - x;
                        canvas.rect(
                            (x_start + margin) as uint,
                            (y + margin) as uint,
                            (x_start + pixel_size + margin) as uint,
                            (y + pixel_size + margin) as uint,
                            &foreground);
                    }
                }
                y += pixel_size;
            }
            x -= pixel_size;
        }

        canvas.image()
    }
}
