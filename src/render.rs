use image::{Rgb, RgbImage};
use rayon::prelude::*;

use crate::{
    camera::Camera,
    color::Color,
    geometry::Vec3,
    interval::Interval,
    ray::Ray,
    shape::{Shape, ShapeList},
};

// TODO: allow to set as parameter.
const SAMPLES_PER_PIXEL: usize = 10;

/// Represents the rendering mode.
///
/// # Variants
/// * `BASIC`   - Basic mode.
/// * `AA`      - Anti-Aliasing mode.
pub enum RenderMode {
    BASIC,
    AA,
}

/// A trait to render scene.
pub trait Renderer {
    fn camera(&self) -> &Camera;
    fn trace(&self, ray: Ray) -> Color;

    /// Render scene with basic mode.
    fn render_basic(&self) -> RgbImage;

    /// Render scene with Anti Aliasing mode.
    ///
    /// 1. Sampling colors at each pixel around it.
    /// 2. Meaning color.
    fn render_aa(&self) -> RgbImage;

    /// Render scene with specified rendering mode.
    fn render(&self, mode: RenderMode) -> RgbImage {
        match mode {
            RenderMode::BASIC => self.render_basic(),
            RenderMode::AA => self.render_aa(),
        }
    }

    fn width(&self) -> u32 {
        self.camera().width
    }

    fn height(&self) -> u32 {
        self.camera().height
    }

    fn aspect(&self) -> f64 {
        self.width() as f64 / self.height() as f64
    }
}

/// A struct to represent scene to render.
pub struct Scene {
    camera: Camera,
    world: ShapeList,
}

impl Scene {
    pub fn new(camera: Camera) -> Self {
        let world = ShapeList::new();
        Self { camera, world }
    }

    pub fn push(&mut self, object: Box<dyn Shape>) {
        self.world.push(object)
    }

    fn background(&self, d: Vec3) -> Color {
        let t = 0.5 * (d.as_unit().y() + 1.0);
        Color::WHITE.lerp(Color::new(0.5, 0.7, 1.0), t)
    }
}

impl Renderer for Scene {
    fn camera(&self) -> &Camera {
        &self.camera
    }

    fn trace(&self, ray: Ray) -> Color {
        const REFLECTANCE: f64 = 0.5;
        // TODO: fix the scatter becomes infinite loop
        if let Some(hit_info) = self.world.hit(&ray, Interval::new()) {
            REFLECTANCE
                * (Color::new(*hit_info.n.x(), *hit_info.n.y(), *hit_info.n.z()) + Color::WHITE)
        } else {
            self.background(ray.direction)
        }
    }

    fn render_basic(&self) -> RgbImage {
        let mut img = RgbImage::new(self.width(), self.height());
        img.enumerate_pixels_mut()
            .collect::<Vec<(u32, u32, &mut Rgb<u8>)>>()
            .par_iter_mut()
            .for_each(|(x, y, pixel)| {
                let u = *x as f64 / (self.width() - 1) as f64;
                let v = *y as f64 / (self.height() - 1) as f64;
                let ray = self.camera().ray(u, v);
                let rgb = self.trace(ray).to_rgb();
                pixel[0] = rgb[0];
                pixel[1] = rgb[1];
                pixel[2] = rgb[2];
            });
        img
    }

    fn render_aa(&self) -> RgbImage {
        let mut img = RgbImage::new(self.width(), self.height());
        img.enumerate_pixels_mut()
            .collect::<Vec<(u32, u32, &mut Rgb<u8>)>>()
            .par_iter_mut()
            .for_each(|(x, y, pixel)| {
                let mut px_color =
                    (0..SAMPLES_PER_PIXEL)
                        .into_iter()
                        .fold(Color::BLACK, |acc, _| {
                            let r = Vec3::rand();
                            let u = (*x as f64 + r.x()) / (self.width() - 1) as f64;
                            let v = (*y as f64 + r.y()) / (self.height() - 1) as f64;
                            let ray = self.camera().ray(u, v);
                            acc + self.trace(ray)
                        });
                px_color /= SAMPLES_PER_PIXEL as f64;
                let rgb = px_color.to_rgb();
                pixel[0] = rgb[0];
                pixel[1] = rgb[1];
                pixel[2] = rgb[2];
            });
        img
    }
}
