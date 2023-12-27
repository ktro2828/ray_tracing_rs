use crate::color::Color;
use crate::material::Material;
use crate::ray::Ray;
use crate::shape::HitInfo;

use super::scatter::ScatterInfo;

pub struct Dilectric {
    pub ir: f64,
}

impl Material for Dilectric {
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
            Some(ScatterInfo::new(
                Ray::new(info.p, refracted),
                Color::new(1.0, 1.0, 1.0),
            ))
        } else {
            Some(ScatterInfo::new(
                Ray::new(info.p, reflected),
                Color::new(1.0, 1.0, 1.0),
            ))
        }
    }
}

impl Dilectric {
    pub fn new(ir: f64) -> Self {
        Dilectric { ir }
    }
}
