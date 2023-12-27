use std::sync::Arc;

use crate::geometry::Vec3;
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::shape::HitInfo;
use crate::shape::Shape;

/// A object shape with sphere.
///
/// # Arguments
/// * `center` - The center position.
/// * `radius` - The radius.
/// * `material` - The material.
pub struct Sphere {
    center: Vec3,
    radius: f64,
    material: Arc<dyn Material>,
}

impl Sphere {
    /// Constructs `Sphere` from values.
    ///
    /// # Arguments
    /// * `center` - The center position.
    /// * `radius` - The radius.
    /// * `material` - The material.
    ///
    /// # Examples
    /// ```
    /// use std::sync::Arc;
    ///
    /// use raytrs::shape::Sphere;
    /// use raytrs::color::Color;
    /// use raytrs::geometry::Vec3;
    /// use raytrs::material::Lambertian;
    ///
    /// let mut world = ShapeList::new();
    /// let sphere = Sphere::new(Vec3::new(1.0, 1.0, 1.0), 10.0, Arc::new(Lambertian::new(Color::random())));
    /// ```
    pub fn new(center: Vec3, radius: f64, material: Arc<dyn Material>) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Shape for Sphere {
    fn hit(&self, ray: &Ray, interval: Interval) -> Option<HitInfo> {
        let oc = *ray.origin() - self.center;
        let a = ray.direction().norm_squared();
        let b = 2.0 * oc.dot(*ray.direction());
        let c = oc.norm_squared() - self.radius.powi(2);
        let d = b.powi(2) - 4.0 * a * c;
        if d > 0.0 {
            let root = d.sqrt();
            let tmp = (-b - root) / (2.0 * a);
            if interval.min < tmp && tmp < interval.max {
                let p = ray.at(tmp);
                return Some(HitInfo::new(tmp, p, (p - self.center) / self.radius));
            }
            let tmp = (-b + root) / (2.0 * a);
            if interval.min < tmp && tmp < interval.max {
                let p = ray.at(tmp);
                return Some(HitInfo::new(tmp, p, (p - self.center) / self.radius));
            }
        }
        None
    }
}
