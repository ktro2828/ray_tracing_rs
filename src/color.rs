// use crate::interval::Interval;
use crate::utils::random;

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    // const INTENSITY: Interval = Interval {
    //     min: 0.000,
    //     max: 0.999,
    // };

    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Color { r, g, b }
    }

    pub fn random() -> Self {
        let r = random::<f64>();
        let g = random::<f64>();
        let b = random::<f64>();
        Color { r, g, b }
    }

    pub fn lerp(&self, c: Self, t: f64) -> Self {
        *self + (c - *self) * t
    }

    pub fn linear2gamma(&self, linear_component: f64) -> f64 {
        linear_component.sqrt()
    }

    pub fn to_string_with_scale(&self, samples_per_pixel: &u32) -> String {
        let scale = 1.0 / *samples_per_pixel as f64;
        let r = self.linear2gamma(self.r * scale);
        let g = self.linear2gamma(self.g * scale);
        let b = self.linear2gamma(self.b * scale);
        Color::new(r, g, b).to_string()
    }
}

impl std::ops::Add<Color> for Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Self::Output {
        Color::new(self.r + rhs.r, self.g + rhs.g, self.b + rhs.b)
    }
}

impl std::ops::Add<f64> for Color {
    type Output = Color;

    fn add(self, rhs: f64) -> Self::Output {
        Color::new(self.r + rhs, self.g + rhs, self.b + rhs)
    }
}

impl std::ops::Add<Color> for f64 {
    type Output = Color;

    fn add(self, rhs: Color) -> Self::Output {
        Color::new(self + rhs.r, self + rhs.g, self + rhs.b)
    }
}

impl std::ops::AddAssign<Color> for Color {
    fn add_assign(&mut self, rhs: Color) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
    }
}

impl std::ops::AddAssign<f64> for Color {
    fn add_assign(&mut self, rhs: f64) {
        self.r += rhs;
        self.g += rhs;
        self.b += rhs;
    }
}

impl std::ops::Sub<Color> for Color {
    type Output = Color;

    fn sub(self, rhs: Color) -> Self::Output {
        Color::new(self.r - rhs.r, self.g - rhs.g, self.b - rhs.b)
    }
}

impl std::ops::Sub<f64> for Color {
    type Output = Color;

    fn sub(self, rhs: f64) -> Self::Output {
        Color::new(self.r - rhs, self.g - rhs, self.b - rhs)
    }
}

impl std::ops::SubAssign<Color> for Color {
    fn sub_assign(&mut self, rhs: Color) {
        self.r -= rhs.r;
        self.g -= rhs.g;
        self.b -= rhs.b;
    }
}

impl std::ops::Mul<Color> for Color {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        Color::new(self.r * rhs.r, self.g * rhs.r, self.b * rhs.r)
    }
}

impl std::ops::Mul<f64> for Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Self::Output {
        Color::new(self.r * rhs, self.g * rhs, self.b * rhs)
    }
}

impl std::ops::Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        Color::new(self * rhs.r, self * rhs.r, self * rhs.r)
    }
}

impl std::ops::MulAssign<Color> for Color {
    fn mul_assign(&mut self, rhs: Color) {
        self.r *= rhs.r;
        self.g *= rhs.g;
        self.b *= rhs.b;
    }
}

impl std::ops::MulAssign<f64> for Color {
    fn mul_assign(&mut self, rhs: f64) {
        self.r *= rhs;
        self.g *= rhs;
        self.b *= rhs;
    }
}

impl std::string::ToString for Color {
    fn to_string(&self) -> String {
        let ir = (self.r * 255.99) as i32;
        let ig = (self.g * 255.99) as i32;
        let ib = (self.b * 255.99) as i32;
        format!("{} {} {}\n", ir, ig, ib)
    }
}
