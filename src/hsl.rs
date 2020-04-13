use image::Rgb;

pub struct HSL {
    hue: f32,
    sat: f32,
    lum: f32,
}

impl HSL {
    pub fn new(hue: f32, sat: f32, lum: f32) -> HSL {
        HSL {
            hue: hue,
            sat: sat,
            lum: lum,
        }
    }

    // http://www.w3.org/TR/css3-color/#hsl-color
    pub fn rgb(&self) -> Rgb<u8> {
        let hue = self.hue / 360.0;
        let sat = self.sat / 100.0;
        let lum = self.lum / 100.0;

        let b = if lum <= 0.5 {
            lum * (sat + 1.0)
        } else {
            lum + sat - lum * sat
        };
        let a = lum * 2.0 - b;

        let r = HSL::hue_to_rgb(a, b, hue + 1.0 / 3.0);
        let g = HSL::hue_to_rgb(a, b, hue);
        let b = HSL::hue_to_rgb(a, b, hue - 1.0 / 3.0);

        Rgb([
            (r * 255.0).round() as u8,
            (g * 255.0).round() as u8,
            (b * 255.0).round() as u8,
        ])
    }

    fn hue_to_rgb(a: f32, b: f32, hue: f32) -> f32 {
        let h = if hue < 0.0 {
            hue + 1.0
        } else if hue > 1.0 {
            hue - 1.0
        } else {
            hue
        };

        if h < 1.0 / 6.0 {
            return a + (b - a) * 6.0 * h;
        }

        if h < 1.0 / 2.0 {
            return b;
        }

        if h < 2.0 / 3.0 {
            return a + (b - a) * (2.0 / 3.0 - h) * 6.0;
        }

        a
    }
}

#[cfg(test)]
mod tests {
    use super::HSL;
    use image::Rgb;

    #[test]
    fn it_converts_black() {
        let black = Rgb([0, 0, 0]);
        let rgb = HSL::new(0.0, 0.0, 0.0).rgb();
        assert_eq!(black, rgb);
    }

    #[test]
    fn it_converts_white() {
        let white = Rgb([255, 255, 255]);
        let rgb = HSL::new(0.0, 0.0, 100.0).rgb();
        assert_eq!(white, rgb);
    }

    #[test]
    fn it_converts_red() {
        let red = Rgb([255, 0, 0]);
        let rgb = HSL::new(0.0, 100.0, 50.0).rgb();
        assert_eq!(red, rgb);
    }

    #[test]
    fn it_converts_green() {
        let green = Rgb([0, 255, 0]);
        let rgb = HSL::new(120.0, 100.0, 50.0).rgb();
        assert_eq!(green, rgb);
    }

    #[test]
    fn it_converts_blue() {
        let blue = Rgb([0, 0, 255]);
        let rgb = HSL::new(240.0, 100.0, 50.0).rgb();
        assert_eq!(blue, rgb);
    }
}
