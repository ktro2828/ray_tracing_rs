pub mod dilectric;
pub mod lambertian;
pub mod metal;

use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub trait Material
where
    Self: Sized + Clone + Copy,
{
    fn emitted(&self, _u: &f64, _v: &f64, _p: &Vec3) -> Color {
        Color::new(0.0, 0.0, 0.0)
    }

    fn scatter(&self, in_ray: &Ray, record: &HitRecord<Self>) -> Ray;

    fn reflectance(&self, cosine: &f64, ref_idx: &f64) -> f64 {
        let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }

    fn get_albedo(&self) -> Color {
        Color::new(1.0, 1.0, 1.0)
    }
}
