use crate::interval::Interval;

#[derive(Debug, Clone, Copy)]
pub struct Color {
    r: f64,
    g: f64,
    b: f64,
}

impl Color {
    const INTENSITY: Interval = Interval {
        min: 0.000,
        max: 0.999,
    };

    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Color { r, g, b }
    }

    pub fn linear2gamma(&self, linear_component: f64) -> f64 {
        linear_component.sqrt()
    }

    pub fn write_color(&self, pixel_color: &Color, samples_per_pixel: &usize) {
        // divide the color by the number os samples
        // apply a linear to gamma transform for gamma 2
        let scale = 1.0 / *samples_per_pixel as f64;
        let r = self.linear2gamma(pixel_color.r * scale);
        let g = self.linear2gamma(pixel_color.g * scale);
        let b = self.linear2gamma(pixel_color.b * scale);

        // write the translated [0,255] value of each color component
        let r_ = 256 * Color::INTENSITY.clamp(r).round() as i64;
        let g_ = 256 * Color::INTENSITY.clamp(g).round() as i64;
        let b_ = 256 * Color::INTENSITY.clamp(b).round() as i64;
        println!("{r_} {g_} {b_}");
    }
}

impl std::ops::Mul<Color> for Color {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        Color::new(self.r * rhs.r, self.g * rhs.r, self.b * rhs.r)
    }
}
