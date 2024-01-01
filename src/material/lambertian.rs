use crate::geometry::Vec3;
use crate::material::Material;
use crate::ray::Ray;
use crate::shape::HitInfo;

use super::scatter::ScatterInfo;
use super::Texture;

/// A struct to represent lambertian material.
#[derive(Debug)]
pub struct Lambertian {
    pub albedo: Box<dyn Texture>,
}

impl Material for Lambertian {
    /// Returns `ScatterInfo`.
    /// If the scatter direction is close to `0.0` the normal vector will be used as scatter direction.
    fn scatter(&self, _ray: &Ray, info: &HitInfo) -> Option<ScatterInfo> {
        let mut scatter_dir = info.n + Vec3::rand_unit();

        if scatter_dir.is_close(0.0) {
            scatter_dir = info.n;
        }

        let albedo = self.albedo.value(info.u, info.v, info.p);
        Some(ScatterInfo::new(Ray::new(info.p, scatter_dir), albedo))
    }
}

impl Lambertian {
    /// Constructs `Lambertian`.
    pub fn new(albedo: Box<dyn Texture>) -> Self {
        Lambertian { albedo }
    }
}
