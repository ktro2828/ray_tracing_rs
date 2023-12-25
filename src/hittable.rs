pub mod sphere;

use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Debug, Clone)]
pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new() -> Self {
        HitRecord {
            p: Vec3::zeros(),
            normal: Vec3::zeros(),
            t: 0.0,
            front_face: true,
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = ray.direction().dot(outward_normal) < 0.0;
        if self.front_face {
            self.normal = outward_normal;
        } else {
            self.normal = outward_normal * -1.0;
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_t: &Interval, record: &mut HitRecord) -> bool;
}

pub struct World {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl World {
    pub fn new() -> Self {
        let objects = Vec::new();
        World { objects }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object)
    }
}

impl Hittable for World {
    fn hit(&self, ray: &Ray, ray_t: &Interval, record: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;
        let mut tmp_record = HitRecord::new();

        self.objects.iter().for_each(|obj| {
            if obj.hit(
                ray,
                &Interval::from(ray_t.min, closest_so_far),
                &mut tmp_record,
            ) {
                hit_anything = true;
                closest_so_far = tmp_record.t;
                record.clone_from(&tmp_record);
            }
        });
        hit_anything
    }
}
