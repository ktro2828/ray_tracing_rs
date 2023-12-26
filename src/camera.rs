pub mod params;

use crate::color::Color;
use crate::hittable::{HitRecord, Hittable, World};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::Vec3;

use self::params::{DefocusParams, FOVParams, PixelParams};

pub struct Camera {
    pub fov: FOVParams,
    pub pixel: PixelParams,
    pub defocus: DefocusParams,
}

impl Camera {
    pub fn new() -> Self {
        Camera::init()
    }

    /// Render world
    pub fn render(&self, world: &World) {
        println!("P3\n{} {}\n255\n", self.fov.width, self.fov.height);

        let samples_per_pixel = 10;
        for j in 0..=self.fov.height {
            for i in 0..self.fov.width {
                let mut px_color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..samples_per_pixel {
                    let ray = self.get_ray(i, j);
                    px_color += self.get_ray_color(&ray, &self.fov.max_depth, world);
                }
                px_color.write_color(&samples_per_pixel);
            }
        }
    }

    fn init() -> Self {
        let mut fov = FOVParams::new();
        let mut pixel = PixelParams::new();
        let mut defocus = DefocusParams::new();

        fov.center = fov.look_from;
        let theta = fov.v_fov.to_radians();
        let h = (0.5 * theta).tan();
        let viewport_height = 2.0 * h * fov.focus_distance;
        let viewport_width = viewport_height * ((fov.height / fov.width) as f64);

        // calculate the u,v,w unit basis vectors for the camera coordinate frame
        pixel.w = (fov.look_from - fov.look_at).as_unit();
        pixel.u = fov.v_up.cross(pixel.w).as_unit();
        pixel.v = pixel.w.cross(pixel.u);

        // calculate the vectors across the horizontal and down the vertical viewport edges
        let viewport_u = pixel.u * viewport_width;
        let viewport_v = pixel.v * -viewport_height;

        // calculate the horizontal and vertical delta vectors to the next pixel
        pixel.pixel_delta_u = viewport_u / (fov.width as f64);
        pixel.pixel_delta_v = viewport_v / (fov.height as f64);

        // calculate the location of the upper left pixel
        let viewport_upper_left =
            fov.center - (pixel.w * fov.focus_distance) - (viewport_u + viewport_v) * 0.5;
        pixel.pixel00_loc = viewport_upper_left + (pixel.pixel_delta_u + pixel.pixel_delta_v) * 0.5;

        // calculate the camera defocus disk basis vectors
        let defocus_radius = fov.focus_distance * (0.5 * defocus.defocus_angle).to_radians().tan();
        defocus.defocus_disk_u = pixel.u * defocus_radius;
        defocus.defocus_disk_v = pixel.v * defocus_radius;

        Camera {
            fov,
            pixel,
            defocus,
        }
    }

    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let i_ = i as f64;
        let j_ = j as f64;

        let pixel_center = self.pixel.pixel00_loc
            + (self.pixel.pixel_delta_u * i_)
            + (self.pixel.pixel_delta_v * j_);
        let pixel_sample = pixel_center + self.pixel_sample_square();

        let ray_origin = if self.defocus.defocus_angle <= 0.0 {
            self.fov.center
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample - ray_origin;

        Ray::from(ray_origin, ray_direction)
    }

    /// Returns a random point in the square surrounding a pixel at the origin.
    fn pixel_sample_square(&self) -> Vec3 {
        let px = Vec3::rand() - 0.5;
        let py = Vec3::rand() + 0.5;
        px * self.pixel.pixel_delta_u + py * self.pixel.pixel_delta_v
    }

    /// Returns a random point in the camera defocus disk.
    fn pixel_sample_disk(&self, radius: f64) -> Vec3 {
        let p = Vec3::rand_unit() * radius;
        self.pixel.pixel_delta_u * *p.x() + self.pixel.pixel_delta_v * *p.y()
    }

    fn defocus_disk_sample(&self) -> Vec3 {
        let p = Vec3::rand_unit();
        self.fov.center
            + self.defocus.defocus_disk_u * *p.x()
            + self.defocus.defocus_disk_v * *p.y()
    }

    fn get_ray_color(&self, ray: &Ray, depth: &u32, world: &World) -> Color {
        if *depth <= 0 {
            Color::new(0.0, 0.0, 0.0)
        } else {
            let mut record = HitRecord::new();
            if world.hit(ray, &Interval::from(0.001, f64::INFINITY), &mut record) {
                let scattered = Ray::from(Vec3::zeros(), Vec3::ones());
                let attenuation = Color::new(0.0, 0.0, 0.0);
                attenuation * self.get_ray_color(&scattered, &(*depth - 1), world)
            } else {
                Color::new(0.0, 0.0, 0.0)
            }
        }
    }
}
