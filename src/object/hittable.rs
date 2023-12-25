use super::material::Material;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Debug)]
pub struct HitRecord<T> {
    pub p: Vec3,
    pub normal: Vec3,
    pub material: T,
    pub t: f64,
    pub front_face: bool,
}

impl<T> HitRecord<T>
where
    T: Material,
{
    pub fn new(material: &T) -> Self {
        HitRecord {
            p: Vec3::zeros(),
            normal: Vec3::zeros(),
            material: *material,
            t: 0.0,
            front_face: true,
        }
    }
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = ray.direction().dot(*outward_normal) < 0.0;
        if self.front_face {
            self.normal = *outward_normal;
        } else {
            self.normal = *outward_normal * -1.0;
        }
    }
}

pub trait Hittable<T>
where
    Self: Sized + Copy + Clone,
{
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> bool;
    fn get_record(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord<T>>;
}
