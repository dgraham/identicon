use image::{ImageBuffer, Rgb, RgbImage};

use hsl::Hsl;
use nibbler::Nibbler;

mod hsl;
mod nibbler;

pub struct Identicon<'a> {
    source: &'a [u8],
    size: u32,
}

impl<'a> Identicon<'a> {
    pub fn new(source: &[u8]) -> Identicon {
        Identicon { source, size: 420 }
    }

    // https://processing.org/reference/map_.html
    fn map(value: u32, vmin: u32, vmax: u32, dmin: u32, dmax: u32) -> f32 {
        ((value - vmin) * (dmax - dmin)) as f32 / ((vmax - vmin) + dmin) as f32
    }

    fn foreground(&self) -> Rgb<u8> {
        // Use last 28 bits to determine Hsl values.
        let h1 = (self.source[12] as u16 & 0x0f) << 8;
        let h2 = self.source[13] as u16;

        let h = (h1 | h2) as u32;
        let s = self.source[14] as u32;
        let l = self.source[15] as u32;

        let hue = Identicon::map(h, 0, 4095, 0, 360);
        let sat = Identicon::map(s, 0, 255, 0, 20);
        let lum = Identicon::map(l, 0, 255, 0, 20);

        Hsl::new(hue, 65.0 - sat, 75.0 - lum).rgb()
    }

    fn rect(image: &mut RgbImage, x0: u32, y0: u32, x1: u32, y1: u32, color: Rgb<u8>) {
        for x in x0..x1 {
            for y in y0..y1 {
                image.put_pixel(x, y, color);
            }
        }
    }

    fn pixels(&self) -> [bool; 25] {
        let mut nibbles = Nibbler::new(self.source).map(|x| x % 2 == 0);
        let mut pixels = [false; 25];
        for col in (0..3).rev() {
            for row in 0..5 {
                let ix = col + (row * 5);
                let mirror_col = 4 - col;
                let mirror_ix = mirror_col + (row * 5);
                let paint = nibbles.next().unwrap_or(false);
                pixels[ix] = paint;
                pixels[mirror_ix] = paint;
            }
        }
        pixels
    }

    pub fn image(&self) -> RgbImage {
        let pixel_size = 70;
        let sprite_size = 5;
        let margin = pixel_size / 2;

        let background = Rgb([240, 240, 240]);
        let foreground = self.foreground();

        let mut image: RgbImage = ImageBuffer::from_pixel(self.size, self.size, background);

        for (row, pix) in self.pixels().chunks(sprite_size).enumerate() {
            for (col, painted) in pix.iter().enumerate() {
                if *painted {
                    let x = col * pixel_size;
                    let y = row * pixel_size;
                    Identicon::rect(
                        &mut image,
                        (x + margin) as u32,
                        (y + margin) as u32,
                        (x + pixel_size + margin) as u32,
                        (y + pixel_size + margin) as u32,
                        foreground,
                    );
                }
            }
        }

        image
    }
}
