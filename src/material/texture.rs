use std::path::Path;

use crate::color::Color;

use super::Texture;
use crate::geometry::Vec3;

pub struct ColorTexture {
    color: Color,
}

impl ColorTexture {
    pub fn new(color: Color) -> Self {
        Self { color }
    }
}

impl Texture for ColorTexture {
    fn value(&self, _u: f64, _v: f64, _p: Vec3) -> Color {
        self.color
    }
}

pub struct CheckerTexture {
    odd: Box<dyn Texture>,
    even: Box<dyn Texture>,
    freq: f64,
}

impl CheckerTexture {
    pub fn new(odd: Box<dyn Texture>, even: Box<dyn Texture>, freq: f64) -> Self {
        Self { odd, even, freq }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: Vec3) -> Color {
        let sines = p.iter().fold(1.0, |acc, x| acc * (x * self.freq).sin());
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}

pub struct ImageTexture {
    pixels: Vec<Color>,
    width: usize,
    height: usize,
}

impl ImageTexture {
    pub fn new<P>(path: P) -> Self
    where
        P: AsRef<Path>,
    {
        let img = image::open(path).unwrap().to_rgb8();
        let (w, h) = img.dimensions();
        let mut pixels = vec![Color::BLACK; (w * h) as usize];
        for (c, (_, _, pixel)) in pixels.iter_mut().zip(img.enumerate_pixels()) {
            *c = Color::new(
                pixel[0] as f64 / 255.0,
                pixel[1] as f64 / 255.0,
                pixel[2] as f64 / 255.0,
            );
        }
        Self {
            pixels,
            width: w as usize,
            height: h as usize,
        }
    }

    fn sample(&self, u: i64, v: i64) -> Color {
        let tu = (u as usize).max(0).min(self.width - 1);
        let tv = (v as usize).max(0).min(self.height - 1);
        self.pixels[tu + self.width * tv]
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _p: Vec3) -> Color {
        let x = (u * self.width as f64) as i64;
        let y = (v * self.height as f64) as i64;
        self.sample(x, y)
    }
}
