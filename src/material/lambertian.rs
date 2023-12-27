use crate::color::Color;
use crate::geometry::Vec3;
use crate::material::Material;
use crate::ray::Ray;
use crate::shape::HitInfo;

use super::scatter::ScatterInfo;

pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, info: &HitInfo) -> Option<ScatterInfo> {
        let mut scatter_dir = info.n + Vec3::rand_unit();

        if scatter_dir.is_close(0.0) {
            scatter_dir = info.n;
        }
        Some(ScatterInfo::new(Ray::new(info.p, scatter_dir), self.albedo))
    }
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Lambertian { albedo }
    }
}
