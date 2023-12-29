use crate::color::Color;
use crate::geometry::vec3::reflect;
use crate::geometry::Vec3;
use crate::material::Material;
use crate::ray::Ray;
use crate::shape::HitInfo;

use super::scatter::ScatterInfo;

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Material for Metal {
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
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Metal { albedo, fuzz }
    }
}
