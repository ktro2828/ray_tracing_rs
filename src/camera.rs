use std::usize;

use crate::color::Color;
use crate::interval::Interval;
use crate::object::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Camera {
    pub fov: FOVParams,
    pub pixel: PixelParams,
    pub defocus: DefocusParams,
}

pub struct FOVParams {
    pub width: u32,
    pub height: u32,
    pub max_depth: u32,
    pub aspect_ratio: f64,
    pub v_fov: f64,
    pub focus_distance: f64,
    pub center: Vec3,
    pub v_up: Vec3,
    pub look_from: Vec3,
    pub look_at: Vec3,
}

impl FOVParams {
    pub fn new() -> Self {
        FOVParams {
            width: 100,
            height: 100,
            max_depth: 10,
            aspect_ratio: 1.0,
            v_fov: 90.0,
            focus_distance: 0.0,
            center: Vec3::zeros(),
            v_up: Vec3::from(0.0, 1.0, 0.0),
            look_from: Vec3::from(0.0, 0.0, -1.0),
            look_at: Vec3::zeros(),
        }
    }
}

pub struct PixelParams {
    pub pixel00_loc: Vec3,
    pub pixel_delta_u: Vec3,
    pub pixel_delta_v: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
}

impl PixelParams {
    pub fn new() -> Self {
        PixelParams {
            pixel00_loc: Vec3::zeros(),
            pixel_delta_u: Vec3::zeros(),
            pixel_delta_v: Vec3::zeros(),
            u: Vec3::zeros(),
            v: Vec3::zeros(),
            w: Vec3::zeros(),
        }
    }
}

pub struct DefocusParams {
    pub defocus_angle: f64,
    pub defocus_disk_u: Vec3,
    pub defocus_disk_v: Vec3,
}

impl DefocusParams {
    pub fn new() -> Self {
        DefocusParams {
            defocus_angle: 0.0,
            defocus_disk_u: Vec3::zeros(),
            defocus_disk_v: Vec3::zeros(),
        }
    }
}

impl Camera {
    pub fn new() -> Self {
        Camera::init()
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

    fn get_ray(&self, i: &usize, j: &usize) -> Ray {
        let i_ = *i as f64;
        let j_ = *j as f64;

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

    fn pixel_sample_square(&self) -> Vec3 {
        let px = Vec3::rand() - 0.5;
        let py = Vec3::rand() + 0.5;
        px * self.pixel.pixel_delta_u + py * self.pixel.pixel_delta_v
    }

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

    // fn ray_color(ray: &Ray, depth: &usize, world: &dyn Hittable) -> Color {
    //     if *depth <= 0 {
    //         Color::new(0.0, 0.0, 0.0)
    //     } else {
    //         let record = &HitRecord::new();
    //         if (world.hit(ray, &Interval::from(0.001, f64::INFINITY), record)) {}
    //     }
    // }
}
