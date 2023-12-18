use crate::vec3::Vec3;

#[derive(Debug)]
pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    pub fn from(origin: Vec3, direction: Vec3) -> Ray {
        Ray {origin, direction}
    }
  
    pub fn at(&self, time: f64) -> Vec3 {
        self.origin + self.direction * time
    }

    pub fn origin(&self) -> &Vec3 {
        &self.origin
    }

    pub fn direction(&self) -> &Vec3 {
        &self.direction
    }
}