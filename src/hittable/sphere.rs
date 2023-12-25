use crate::hittable::HitRecord;
use crate::hittable::Hittable;
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Debug, Copy, Clone)]
pub struct Sphere<T> {
    pub center: Vec3,
    pub radius: f64,
    pub material: T,
}

impl<T> Sphere<T>
where
    T: Material,
{
    pub fn new(center: Vec3, radius: f64, material: T) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl<T> Hittable for Sphere<T>
where
    T: Material,
{
    fn hit(&self, ray: &Ray, ray_t: &Interval, record: &mut HitRecord) -> bool {
        let origin = *ray.origin() - self.center;
        let a = ray.direction().norm_squared();
        let half_b = origin.dot(*ray.direction());
        let c = origin.norm_squared() - self.radius.powi(2);

        let discriminant = half_b.powi(2) - a * c;
        if discriminant < 0.0 {
            false
        } else {
            let root = -(half_b + discriminant.sqrt()) / a;
            let mut is_ok = true;
            if !ray_t.surrounds(root) {
                let root = (-half_b + discriminant.sqrt()) / a;
                is_ok = ray_t.surrounds(root);
            }

            if is_ok {
                record.t = root;
                record.p = ray.at(record.t);
                let outward_normal = (record.p - self.center) / self.radius;
                record.set_face_normal(ray, outward_normal);

                true
            } else {
                false
            }
        }
    }
}
