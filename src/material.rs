pub(crate) mod dilectric;
pub(crate) mod lambertian;
pub(crate) mod metal;
pub(crate) mod scatter;
pub(crate) mod texture;

use crate::color::Color;
use crate::geometry::Vec3;
use crate::ray::Ray;
use crate::shape::HitInfo;

use self::dilectric::Dilectric as _Dilectric;
use self::lambertian::Lambertian as _Lambertian;
use self::metal::Metal as _Metal;
use self::scatter::ScatterInfo;
use self::texture::CheckerTexture as _CheckerTexture;
use self::texture::ColorTexture as _ColorTexture;
use self::texture::ImageTexture as _ImageTexture;

pub type Dilectric = _Dilectric;
pub type Lambertian = _Lambertian;
pub type Metal = _Metal;
pub type CheckerTexture = _CheckerTexture;
pub type ColorTexture = _ColorTexture;
pub type ImageTexture = _ImageTexture;

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

pub trait Texture: Sync + Send {
    fn value(&self, u: f64, v: f64, p: Vec3) -> Color;
}
