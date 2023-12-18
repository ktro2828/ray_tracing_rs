use super::hittable::HitRecord;
use crate::color::Color;
use crate::ray::Ray;
use crate::vec3::{dot, reflect, refract, Vec3};

pub trait Material: Copy {
    fn emitted(&self, _u: &f64, _v: &f64, _p: &Vec3) -> Color {
        Color::new(0.0, 0.0, 0.0)
    }

    fn scatter<T: Material>(&self, in_ray: &Ray, record: &HitRecord<T>) -> (Color, Ray);

    fn reflectance(&self, cosine: &f64, ref_idx: &f64) -> f64 {
        let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}


#[derive(Debug, Clone, Copy)]
pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter<T: Material>(&self, _in_ray: &Ray, record: &HitRecord<T>) -> (Color, Ray) {
        let mut scatter_dir = record.normal + Vec3::rand_unit();

        if scatter_dir.is_close(0.0) {
            scatter_dir = record.normal;
        }

        let scattered = Ray::from(record.p, scatter_dir);

        (self.albedo, scattered)
    }
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Lambertian { albedo }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Material for Metal {
    fn scatter<T: Material>(&self, in_ray: &Ray, record: &HitRecord<T>) -> (Color, Ray) {
        let reflected = reflect(in_ray.direction().as_unit(), record.normal);
        let scattered = Ray::from(record.p, reflected + Vec3::rand_unit() * self.fuzz);
        (self.albedo, scattered)
    }
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Metal { albedo, fuzz }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Dilectric {
    pub ir: f64,
}

impl Material for Dilectric {
    fn scatter<T: Material>(&self, in_ray: &Ray, record: &HitRecord<T>) -> (Color, Ray) {
        let refraction_ratio = if record.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_dir = in_ray.direction().as_unit();
        let cos_theta = dot(unit_dir * -1.0, record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        // TODO: use random_double() not 1.0
        let direction = if cannot_refract && self.reflectance(&cos_theta, &refraction_ratio) > 1.0 {
            reflect(unit_dir, record.normal)
        } else {
            refract(unit_dir, record.normal, refraction_ratio)
        };

        let scattered = Ray::from(record.p, direction);
        let attenuation = Color::new(1.0, 1.0, 1.0);
        (attenuation, scattered)
    }
}

impl Dilectric {
    pub fn new(ir: f64) -> Self {
        Dilectric { ir }
    }
}
