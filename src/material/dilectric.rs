use crate::color::Color;
use crate::material::Material;
use crate::ray::Ray;
use crate::shape::HitInfo;

use super::scatter::ScatterInfo;

/// A struct to represent dilectric material.
#[derive(Debug)]
pub struct Dilectric {
    pub ir: f64,
}

impl Material for Dilectric {
    /// Returns `ScatterInfo`.
    /// If the ray is refracted returns scatter with refracted vector.
    /// Otherwise, returns with reflected vector.
    ///
    /// # Arguments
    /// * `ray`     - `Ray` instance.
    /// * `info`    - `HitInfo` instance.
    fn scatter(&self, ray: &Ray, info: &HitInfo) -> Option<ScatterInfo> {
        let reflected = ray.direction().reflect(info.n);
        let (outward_normal, ni_over_nt) = {
            if ray.direction().dot(info.n) > 0.0 {
                (-info.n, self.ir)
            } else {
                (info.n, self.ir.recip())
            }
        };
        if let Some(refracted) = (-ray.direction).refract(outward_normal, ni_over_nt) {
            Some(ScatterInfo::new(Ray::new(info.p, refracted), Color::WHITE))
        } else {
            Some(ScatterInfo::new(Ray::new(info.p, reflected), Color::WHITE))
        }
    }
}

impl Dilectric {
    /// Constructs `Dilectric`.
    ///
    /// # Arguments
    /// * `ir`  - The reflectance.
    pub fn new(ir: f64) -> Self {
        Dilectric { ir }
    }
}
