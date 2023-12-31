// use crate::interval::Interval;
use crate::utils::random;

/// A struct to represent RGB color.
#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    /// A black color.
    pub const BLACK: Color = Color::new(0.0, 0.0, 0.0);

    /// A white color.
    pub const WHITE: Color = Color::new(1.0, 1.0, 1.0);

    /// A red color.
    pub const RED: Color = Color::new(1.0, 0.0, 0.0);

    /// A green color.
    pub const GREEN: Color = Color::new(0.0, 1.0, 0.0);

    /// A blue color.
    pub const BLUE: Color = Color::new(0.0, 0.0, 1.0);

    /// Constructs RGB with specified values in `[0.0, 1.0]`.
    ///
    /// # Arguments
    /// * `r`   - R channel value.
    /// * `g`   - G channel value.
    /// * `b`   - B channel value.
    pub const fn new(r: f64, g: f64, b: f64) -> Self {
        Color { r, g, b }
    }

    /// Constructs random color.
    pub fn random() -> Self {
        let r = random::<f64>();
        let g = random::<f64>();
        let b = random::<f64>();
        Color { r, g, b }
    }

    pub fn lerp(&self, c: Self, t: f64) -> Self {
        *self + (c - *self) * t
    }

    pub fn linear2gamma(&self, factor: f64) -> Self {
        let inv = factor.recip();
        Color::new(self.r.powf(inv), self.g.powf(inv), self.b.powf(inv))
    }

    pub fn to_string_gamma(&self, samples_per_pixel: usize) -> String {
        let factor = 1.0 / samples_per_pixel as f64;
        self.linear2gamma(factor).to_string()
    }

    /// Convert normalized float values to RGB in u8
    ///
    /// # Examples
    /// ```
    /// use raytrs::color::Color;
    ///
    /// let [r, g, b] = Color::RED.to_rgb();
    /// assert_eq!(r, 255);
    /// assert_eq!(g, 0);
    /// assert_eq!(b, 0);
    /// ```
    pub fn to_rgb(&self) -> [u8; 3] {
        let r = (255.99 * self.r) as u8;
        let g = (255.99 * self.g) as u8;
        let b = (255.99 * self.b) as u8;
        [r, g, b]
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

impl std::ops::Div<Color> for Color {
    type Output = Color;

    fn div(self, rhs: Color) -> Self::Output {
        Color::new(self.r / rhs.r, self.g / rhs.g, self.b / rhs.b)
    }
}

impl std::ops::Div<f64> for Color {
    type Output = Color;

    fn div(self, rhs: f64) -> Self::Output {
        Color::new(self.r / rhs, self.g / rhs, self.b / rhs)
    }
}

impl std::ops::DivAssign<Color> for Color {
    fn div_assign(&mut self, rhs: Color) {
        self.r /= rhs.r;
        self.g /= rhs.g;
        self.b /= rhs.b;
    }
}

impl std::ops::DivAssign<f64> for Color {
    fn div_assign(&mut self, rhs: f64) {
        self.r /= rhs;
        self.g /= rhs;
        self.b /= rhs;
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
