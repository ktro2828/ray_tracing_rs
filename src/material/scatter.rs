use crate::color::Color;
use crate::ray::Ray;

/// A container to store ray's scatter information for each material.
#[allow(unused)]
pub struct ScatterInfo {
    pub(crate) ray: Ray,
    pub(crate) albedo: Color,
}

impl ScatterInfo {
    /// Constructs `ScatterInfo`.
    pub(crate) fn new(ray: Ray, albedo: Color) -> Self {
        Self { ray, albedo }
    }
}
