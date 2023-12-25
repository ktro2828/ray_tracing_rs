use crate::color::Color;
use crate::material::Material;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(&self, _in_ray: &Ray, record: &HitRecord<Self>) -> Ray {
        let mut scatter_dir = record.normal + Vec3::rand_unit();

        if scatter_dir.is_close(0.0) {
            scatter_dir = record.normal;
        }
        Ray::from(record.p, scatter_dir)
    }

    fn get_albedo(&self) -> Color {
        self.albedo
    }
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Lambertian { albedo }
    }
}