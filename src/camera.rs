pub mod params;

use crate::geometry::Vec3;
use crate::ray::Ray;

/// A struct to represent camera.
///
/// # Arguments
/// * `origin`  - An origin position of camera, a.k.a lookfrom.
/// * `u`
/// * `v`
/// * `w`
#[derive(Debug)]
pub struct Camera {
    pub origin: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
    pub width: u32,
    pub height: u32,
}

impl Camera {
    /// Constructs `Camera` instance from uvw vectors.
    ///
    /// # Arguments
    /// * `u`       -
    /// * `v`       -
    /// * `w`       -
    /// * `width`   - The image width.
    /// * `height`  - The image height.
    ///
    /// # Examples
    /// ```
    /// use raytrs::camera::Camera;
    /// use raytrs::geometry::Vec3;
    ///
    /// let cam = Camera::new(
    ///     Vec3::new(4.0, 0.0, 0.0),
    ///     Vec3::new(0.0, 2.0, 0.0),
    ///     Vec3::new(-2.0, -1.0, -1.0),
    ///     300,
    ///     300,
    /// );
    /// ```
    pub fn new(u: Vec3, v: Vec3, w: Vec3, width: u32, height: u32) -> Self {
        Self {
            origin: Vec3::zeros(),
            u,
            v,
            w,
            width,
            height,
        }
    }

    /// Constructs `Camera` instance from the position where the camera looks at.
    ///
    /// # Arguments
    /// * `origin`  - The Camera origin.
    /// * `lookat`  - The position where the camera looks at.
    /// * `vup`     - The normalized vector of up direction.
    /// * `vov`     - The vertical angle of FOV \[deg\].
    /// * `width`   - The image width.
    /// * `height`  - The image height.
    ///
    /// # Examples
    /// ```
    /// use raytrs::camera::Camera;
    /// use raytrs::geometry::Vec3;
    ///
    /// let cam = Camera::from_lookat(
    ///     Vec3::new(0.0, 0.0, 0.0),
    ///     Vec3::new(0.0, 0.0, -1.0),
    ///     Vec3::new(0.0, 1.0, 0.0),
    ///     60.0,
    ///     300,
    ///     200,
    /// );
    /// ```
    pub fn from_lookat(
        origin: Vec3,
        lookat: Vec3,
        vup: Vec3,
        vfov: f64,
        width: u32,
        height: u32,
    ) -> Self {
        let half_h = (vfov.to_radians() * 0.5).tan();
        let half_w = (width as f64 / height as f64) * half_h;
        let w = (origin - lookat).as_unit();
        let u = vup.cross(w).as_unit();
        let v = w.cross(u);
        let uw = half_w * u;
        let vh = half_h * v;
        Self {
            origin,
            u: 2.0 * uw,
            v: 2.0 * vh,
            w: origin - uw - vh - w,
            width,
            height,
        }
    }

    pub fn ray(&self, u: f64, v: f64) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.w + self.u * u + self.v * v - self.origin,
        }
    }
}
