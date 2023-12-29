pub(crate) mod sphere;

use std::sync::Arc;

use crate::geometry::Vec3;
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;

use self::sphere::Sphere as _Sphere;

pub type Sphere = _Sphere;

/// A container to store hit information
///
/// # Arguments
/// * `t` - A parameter of the ray.
/// * `p` - A point where ray collides with an object.
/// * `n` - A normal vector on the collision point.
pub struct HitInfo {
    pub t: f64,
    pub p: Vec3,
    pub n: Vec3,
    pub m: Arc<dyn Material>,
}

impl HitInfo {
    /// Constructs `HitInfo` from values.
    ///
    /// # Arguments
    /// * `t` - A parameter of the ray.
    /// * `p` - A point where ray collides with an object.
    /// * `n` - A normal vector on the collision point.
    ///
    /// # Examples
    /// ```
    /// use raytrs::shape::HitInfo;
    /// use raytrs::geometry::Vec3;
    ///
    /// let info = HitInfo::new(1.0, Vec3::ones(), Vec3::ones());
    /// ```
    pub fn new(t: f64, p: Vec3, n: Vec3, m: Arc<dyn Material>) -> Self {
        HitInfo { t, p, n, m }
    }
}

/// A trait for objects can be hit.
pub trait Shape: Sync {
    /// Returns `HitInfo` if ray collides with the object.
    ///
    /// # Arguments
    /// * `ray` - A `Ray` instance.
    /// * `interval` - Interval of the ray.
    fn hit(&self, ray: &Ray, interval: Interval) -> Option<HitInfo>;
}

/// A container to store objects in the world.
pub struct ShapeList {
    pub objects: Vec<Box<dyn Shape>>,
}

impl ShapeList {
    /// Constructs `ShapeList`.
    ///
    /// # Examples
    /// ```
    /// use raytrs::shape::ShapeList;
    ///
    /// let world = ShapeList::new();
    /// ```
    pub fn new() -> Self {
        let objects = Vec::new();
        ShapeList { objects }
    }

    /// Appends object into `ShapeList`.
    ///
    /// # Arguments
    /// * `object` - Any objects can be hit.
    ///
    /// # Examples
    /// ```
    /// use std::sync::Arc;
    ///
    /// use raytrs::shape::{ShapeList, Sphere};
    /// use raytrs::color::Color;
    /// use raytrs::geometry::Vec3;
    /// use raytrs::material::Lambertian;
    ///
    /// let mut world = ShapeList::new();
    /// let sphere = Sphere::new(Vec3::ones(), 10.0, Arc::new(Lambertian::new(Color::random())));
    ///
    /// world.push(Box::new(sphere));
    /// ```
    pub fn push(&mut self, object: Box<dyn Shape>) {
        self.objects.push(object)
    }
}

impl Shape for ShapeList {
    fn hit(&self, ray: &Ray, interval: Interval) -> Option<HitInfo> {
        let mut hit_info: Option<HitInfo> = None;
        let mut closest_so_far = interval.max;

        for obj in &self.objects {
            if let Some(info) = obj.hit(ray, Interval::from_val(interval.min, closest_so_far)) {
                closest_so_far = info.t;
                hit_info = Some(info);
            }
        }
        hit_info
    }
}
