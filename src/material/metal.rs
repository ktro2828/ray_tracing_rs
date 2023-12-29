use crate::color::Color;
use crate::geometry::vec3::reflect;
use crate::geometry::Vec3;
use crate::material::Material;
use crate::ray::Ray;
use crate::shape::HitInfo;

use super::scatter::ScatterInfo;

/// A struct to represent metal material.
pub struct Metal {
    pub(crate) albedo: Color,
    pub(crate) fuzz: f64,
}

impl Material for Metal {
    /// Returns `ScatterInfo`.
    ///
    /// The hit ray will be reflected randomly.
    /// If the scatter ray became reflected vector,
    /// which equals to the dot product of vector `v` and normal `n` becomes `> 0.0`, the returns scatter info.
    /// Otherwise, returns `None`.
    fn scatter(&self, ray: &Ray, info: &HitInfo) -> Option<ScatterInfo> {
        let mut reflected = reflect(ray.direction().as_unit(), info.n);
        reflected += Vec3::rand_unit() * self.fuzz;
        if reflected.dot(info.n) > 0.0 {
            Some(ScatterInfo::new(Ray::new(info.p, reflected), self.albedo))
        } else {
            None
        }
    }
}

impl Metal {
    /// Constructs `Metal`.
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Metal { albedo, fuzz }
    }
}
