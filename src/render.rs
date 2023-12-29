use crate::{
    camera::Camera,
    color::Color,
    geometry::Vec3,
    interval::Interval,
    ray::Ray,
    shape::{Shape, ShapeList},
};

// TODO: allow to set as parameter.
const IMAGE_WIDTH: u32 = 300;
const IMAGE_HEIGHT: u32 = 200;
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
    fn camera(&self) -> Camera;
    fn trace(&self, ray: Ray) -> Color;

    /// Render scene with basic mode.
    fn render_basic(&self) -> String;

    /// Render scene with Anti Aliasing mode.
    ///
    /// 1. Sampling colors at each pixel around it.
    /// 2. Meaning color.
    fn render_aa(&self) -> String;

    /// Render scene with specified rendering mode.
    fn render(&self, mode: RenderMode) -> String {
        match mode {
            RenderMode::BASIC => self.render_basic(),
            RenderMode::AA => self.render_aa(),
        }
    }

    fn width(&self) -> u32 {
        IMAGE_WIDTH
    }
    fn height(&self) -> u32 {
        IMAGE_HEIGHT
    }
    fn aspect(&self) -> f64 {
        self.width() as f64 / self.height() as f64
    }
}

/// A struct to represent scene to render.
pub struct Scene {
    world: ShapeList,
}

impl Scene {
    pub fn new() -> Self {
        let world = ShapeList::new();
        Self { world }
    }

    pub fn push(&mut self, object: Box<dyn Shape>) {
        self.world.push(object)
    }

    fn background(&self, d: Vec3) -> Color {
        let t = 0.5 * (d.as_unit().y() + 1.0);
        Color::BLACK.lerp(Color::new(0.5, 0.7, 1.), t)
    }
}

impl Renderer for Scene {
    fn camera(&self) -> Camera {
        Camera::new(
            Vec3::new(4.0, 0.0, 0.0),
            Vec3::new(0.0, 2.0, 0.0),
            Vec3::new(-2.0, -1.0, -1.0),
        )
    }

    fn trace(&self, ray: Ray) -> Color {
        const REFLECTANCE: f64 = 0.5;
        if let Some(hit_info) = self.world.hit(&ray, Interval::new()) {
            REFLECTANCE
                * (Color::new(*hit_info.n.x(), *hit_info.n.y(), *hit_info.n.z()) + Color::BLACK)
        } else {
            self.background(ray.direction)
        }
    }

    fn render_basic(&self) -> String {
        let mut ppm_str = format!("P3\n{} {}\n255\n", self.width(), self.height());

        for y in 0..self.height() {
            for x in 0..self.width() {
                let u = x as f64 / (self.width() - 1) as f64;
                let v = y as f64 / (self.height() - 1) as f64;
                let ray = self.camera().ray(u, v);
                let color = self.trace(ray);
                ppm_str += &color.to_string();
            }
        }
        ppm_str
    }

    fn render_aa(&self) -> String {
        let mut ppm_str = format!("P3\n{} {}\n255\n", self.width(), self.height());

        for y in 0..self.height() {
            for x in 0..self.width() {
                let mut px_color =
                    (0..SAMPLES_PER_PIXEL)
                        .into_iter()
                        .fold(Color::WHITE, |acc, _| {
                            let r = Vec3::rand();
                            let u = (x as f64 + r.x()) / (self.width() - 1) as f64;
                            let v = (y as f64 + r.y()) / (self.height() - 1) as f64;
                            let ray = self.camera().ray(u, v);
                            acc + self.trace(ray)
                        });
                px_color /= SAMPLES_PER_PIXEL as f64;
                ppm_str += &px_color.to_string();
            }
        }
        ppm_str
    }
}
