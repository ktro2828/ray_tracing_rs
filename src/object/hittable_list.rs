use super::{hittable::Hittable, material::Material};

pub struct HittableList<T, U>
where
    T: Material,
    U: Hittable<T>,

{
    pub material: T,
    pub objects: Vec<U>,
}

impl<T, U> HittableList<T, U>
where
    T: Material,
    U: Hittable<T>,
{
    pub fn new() -> Self {
        let objects = Vec::new();
        HittableList { objects }
    }

    pub fn from(other: &HittableList<T, U>) -> Self {
        let objects = other.objects;
        HittableList { objects }
    }

    pub fn clear(&mut self) {
        self.objects.clear()
    }

    pub fn add(&mut self, object: T) {
        self.objects.push(object)
    }
}

impl<T, U> Hittable<T> for HittableList<T, U>
where
    T: Material,
    U: Hittable<T>
{
    fn get_record(&self, ray: &crate::ray::Ray, ray_t: &crate::interval::Interval) -> Option<super::hittable::HitRecord<T>> {
    }

    fn hit(&self, ray: &crate::ray::Ray, ray_t: &crate::interval::Interval) -> bool {
        
    }
}
