use super::hittable::HitRecord;
use super::hittable::Hittable;
use super::material::Material;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Debug)]
pub struct Sphere<T>
where
    T: Material,
{
    center: Vec3,
    radius: f64,
    material: T,
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

impl<T> Hittable<T> for Sphere<T>
where
    T: Material,
{
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> bool {
        let origin = *ray.origin() - self.center;
        let a = ray.direction().norm_squared();
        let half_b = origin.dot(*ray.direction());
        let c = origin.norm_squared() - self.radius.powi(2);

        let discriminant = half_b.powi(2) - a * c;
        if discriminant < 0.0 {
            false
        } else {
            let root = -(half_b + discriminant.sqrt()) / a;
            if !ray_t.surrounds(root) {
                let root = (-half_b + discriminant.sqrt()) / a;
                ray_t.surrounds(root)
            } else {
                true
            }
        }
    }

    fn get_record(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord<T>> {
        if !self.hit(ray, ray_t) {
            None
        } else {
            let mut ret = HitRecord::new(&self.material);
            let oc = *ray.origin() - self.center;
            let a = ray.direction().norm_squared();
            let b_half = oc.dot(*ray.direction());
            let c = oc.norm_squared() - self.radius.powi(2);

            let discriminant = b_half.powi(2) - a * c;
            let mut root = -(b_half - discriminant.sqrt()) / a;
            if !ray_t.surrounds(root) {
                root = (-b_half + discriminant.sqrt()) / a;
            }

            ret.t = root;
            ret.p = ray.at(ret.t);
            let outward_normal = (ret.p - self.center) / self.radius;
            ret.set_face_normal(ray, &outward_normal);
            Some(ret)
        }
    }
}
