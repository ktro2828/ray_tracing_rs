use crate::hittable::HitRecord;
use crate::material::Material;
use crate::ray::Ray;
use crate::utils::random;
use crate::vec3::{dot, reflect, refract};

#[derive(Debug, Clone, Copy)]
pub struct Dilectric {
    pub ir: f64,
}

impl Material for Dilectric {
    fn scatter(&self, in_ray: &Ray, record: &HitRecord<Self>) -> Ray {
        let refraction_ratio = if record.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_dir = in_ray.direction().as_unit();
        let cos_theta = dot(unit_dir * -1.0, record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        let direction =
            if cannot_refract && self.reflectance(&cos_theta, &refraction_ratio) > random() {
                reflect(unit_dir, record.normal)
            } else {
                refract(unit_dir, record.normal, refraction_ratio)
            };

        let scattered = Ray::from(record.p, direction);
        scattered
    }
}

impl Dilectric {
    pub fn new(ir: f64) -> Self {
        Dilectric { ir }
    }
}
