use crate::{
    camera::Camera,
    color::Color,
    geometry::Vec3,
    interval::Interval,
    ray::Ray,
    shape::{Shape, ShapeList},
};

const IMAGE_WIDTH: u32 = 300;
const IMAGE_HEIGHT: u32 = 200;

pub trait Renderer {
    fn camera(&self) -> Camera;
    fn trace(&self, ray: Ray) -> Color;
    fn render(&self) -> String;

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
        Color::new(1.0, 1.0, 1.0).lerp(Color::new(0.5, 0.7, 1.), t)
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
        if let Some(hit_info) = self.world.hit(&ray, Interval::new()) {
            0.5 * (Color::new(*hit_info.n.x(), *hit_info.n.y(), *hit_info.n.z())
                + Color::new(1.0, 1.0, 1.0))
        } else {
            self.background(ray.direction)
        }
    }

    fn render(&self) -> String {
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
}
