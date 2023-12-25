use crate::vec3::Vec3;

pub struct FOVParams {
    pub width: u32,
    pub height: u32,
    pub max_depth: u32,
    pub aspect_ratio: f64,
    pub v_fov: f64,
    pub focus_distance: f64,
    pub center: Vec3,
    pub v_up: Vec3,
    pub look_from: Vec3,
    pub look_at: Vec3,
}

impl FOVParams {
    pub fn new() -> Self {
        FOVParams {
            width: 100,
            height: 100,
            max_depth: 10,
            aspect_ratio: 1.0,
            v_fov: 90.0,
            focus_distance: 0.0,
            center: Vec3::zeros(),
            v_up: Vec3::from(0.0, 1.0, 0.0),
            look_from: Vec3::from(0.0, 0.0, -1.0),
            look_at: Vec3::zeros(),
        }
    }
}

pub struct PixelParams {
    pub pixel00_loc: Vec3,
    pub pixel_delta_u: Vec3,
    pub pixel_delta_v: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
}

impl PixelParams {
    pub fn new() -> Self {
        PixelParams {
            pixel00_loc: Vec3::zeros(),
            pixel_delta_u: Vec3::zeros(),
            pixel_delta_v: Vec3::zeros(),
            u: Vec3::zeros(),
            v: Vec3::zeros(),
            w: Vec3::zeros(),
        }
    }
}

pub struct DefocusParams {
    pub defocus_angle: f64,
    pub defocus_disk_u: Vec3,
    pub defocus_disk_v: Vec3,
}

impl DefocusParams {
    pub fn new() -> Self {
        DefocusParams {
            defocus_angle: 0.0,
            defocus_disk_u: Vec3::zeros(),
            defocus_disk_v: Vec3::zeros(),
        }
    }
}