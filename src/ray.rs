use crate::geometry::Vec3;

/// A struct to represent ray.
///
/// # Arguments
/// * `origin`      - Ray origin.
/// * `direction`   - Ray direction.
#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

#[cfg_attr(doc, katexit::katexit)]
impl Ray {
    /// Constructs ray from specified values.
    ///
    /// # Arguments
    /// * `origin`      - Ray origin.
    /// * `direction`   - Ray direction.
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }

    /// Returns the point of ray at specified time.
    ///
    /// $$
    /// \vec{p} = \vec{o} + t * \vec{d}
    /// $$
    ///
    /// # Arguments
    /// * `time`    - Specific time.
    pub fn at(&self, time: f64) -> Vec3 {
        self.origin + self.direction * time
    }

    /// Returns the reference of `origin`.
    pub fn origin(&self) -> &Vec3 {
        &self.origin
    }

    /// Returns the reference of `direction`.
    pub fn direction(&self) -> &Vec3 {
        &self.direction
    }
}
