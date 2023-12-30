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

#[cfg_attr(doc, katexit::katexit)]
impl Shape for Sphere {
    /// Returns `HitInfo` if ray hits to itself.
    ///
    /// 1. Computes discriminant `D`.
    /// 2. If `D` > 0.0, it means ray hit.
    /// 3. Computes the `t` which is the time ray hits to sphere.
    /// 4. Computes the unit normal vector from center to the point at `t`.
    ///
    /// $$
    /// (\vec{p} - \vec{c}) \cdot (\vec{p} - \vec{c}) = r^2
    /// $$
    /// with the ray equation $\vec{p}(t) = \vec{o} + t\vec{d} $,
    /// $$
    /// (\vec{p}(t) - \vec{c}) \cdot (\vec{p}(t) - \vec{c}) = r^2
    /// $$
    /// and expands formula as `t`,
    /// $$
    /// (\vec{d} \cdot \vec{d})t^2 + 2(\vec{d} \cdot \vec{oc})t + (\vec{oc} \cdot \vec{oc}) - r^2 = 0
    /// $$
    ///
    /// finally, the solution of formula is as follows,
    /// $$
    /// t = \frac{-b \pm \sqrt{b^2 - 4ac}}{2a}
    /// $$
    ///
    /// # Arguments
    /// * `ray`         - Ray from camera.
    /// * `interval`    - Time range of ray.
    fn hit(&self, ray: &Ray, interval: Interval) -> Option<HitInfo> {
        let oc = *ray.origin() - self.center;
        let a = ray.direction().norm_squared();
        let b = 2.0 * oc.dot(*ray.direction());
        let c = oc.norm_squared() - self.radius.powi(2);
        let d = b.powi(2) - 4.0 * a * c;
        if d > 0.0 {
            let root = d.sqrt();
            let t = if (-b - root) > 0.0 {
                (-b - root) / (2.0 * a)
            } else {
                (-b + root) / (2.0 * a)
            };
            if interval.min < t && t < interval.max {
                let p = ray.at(t);
                return Some(HitInfo::new(
                    t,
                    p,
                    (p - self.center) / self.radius,
                    Arc::clone(&self.material),
                ));
            }
        }
        None
    }
}
