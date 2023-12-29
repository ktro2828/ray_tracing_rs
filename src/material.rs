pub(crate) mod dilectric;
pub(crate) mod lambertian;
pub(crate) mod metal;
pub(crate) mod scatter;

use crate::color::Color;
use crate::geometry::Vec3;
use crate::ray::Ray;
use crate::shape::HitInfo;

use self::dilectric::Dilectric as _Dilectric;
use self::lambertian::Lambertian as _Lambertian;
use self::metal::Metal as _Metal;
use self::scatter::ScatterInfo;

pub type Dilectric = _Dilectric;
pub type Lambertian = _Lambertian;
pub type Metal = _Metal;

/// A trait for object's material.
pub trait Material: Sync + Send {
    fn emitted(&self, _u: &f64, _v: &f64, _p: &Vec3) -> Color {
        Color::BLACK
    }

    fn scatter(&self, ray: &Ray, info: &HitInfo) -> Option<ScatterInfo>;

    fn reflectance(&self, cosine: &f64, ref_idx: &f64) -> f64 {
        let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}
