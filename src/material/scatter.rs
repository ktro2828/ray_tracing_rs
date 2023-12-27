use crate::color::Color;
use crate::ray::Ray;

pub struct ScatterInfo {
    pub ray: Ray,
    pub albedo: Color,
}

impl ScatterInfo {
    pub(crate) fn new(ray: Ray, albedo: Color) -> Self {
        Self { ray, albedo }
    }
}
