use crate::color::Color;
use crate::hittable::HitRecord;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::reflect;
use crate::vec3::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Material for Metal {
    fn scatter(&self, in_ray: &Ray, record: &HitRecord) -> Ray {
        let reflected = reflect(in_ray.direction().as_unit(), record.normal);
        Ray::from(record.p, reflected + Vec3::rand_unit() * self.fuzz)
    }

    fn get_albedo(&self) -> Color {
        self.albedo
    }
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Metal { albedo, fuzz }
    }
}
