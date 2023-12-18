use rand::Rng;

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    pub fn zeros() -> Self {
        let e = [0.0, 0.0, 0.0];
        Vec3 { e }
    }

    pub fn from(e0: f64, e1: f64, e2: f64) -> Self {
        let e = [e0, e1, e2];
        Vec3 { e }
    }

    pub fn rand() -> Self {
        let mut rng = rand::thread_rng();
        let e = [rng.gen(), rng.gen(), rng.gen()];
        Vec3 { e }
    }

    pub fn x(&self) -> &f64 {
        &self.e[0]
    }

    pub fn y(&self) -> &f64 {
        &self.e[1]
    }

    pub fn z(&self) -> &f64 {
        &self.e[2]
    }

    pub fn dot(&self, rhs: Vec3) -> f64 {
        _dot(&self.e, &rhs.e)
    }

    pub fn cross(&self, rhs: Vec3) -> Vec3 {
        let e = _cross(&self.e, &rhs.e);
        Vec3 { e }
    }

    pub fn norm(&self) -> f64 {
        self.norm_squared().sqrt()
    }

    pub fn norm_squared(&self) -> f64 {
        self.x().powi(2) + self.y().powi(2) + self.z().powi(2)
    }

    pub fn as_unit(&self) -> Self {
        *self / self.norm()
    }

    pub fn rand_unit() -> Self {
        Vec3::rand().as_unit()
    }

    pub fn is_close(&self, x: f64) -> bool {
        const TOLERANCE: f64 = 1e-8;
        return (self.e[0] - x).abs() < TOLERANCE
            && (self.e[1] - x).abs() < TOLERANCE
            && (self.e[2] - x).abs() < TOLERANCE;
    }
}

impl std::ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3::from(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}

impl std::ops::Add<f64> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: f64) -> Self::Output {
        Vec3::from(self.x() + rhs, self.y() + rhs, self.z() + rhs)
    }
}

impl std::ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.e[0] += rhs.x();
        self.e[1] += rhs.y();
        self.e[2] += rhs.z();
    }
}

impl std::ops::AddAssign<f64> for Vec3 {
    fn add_assign(&mut self, rhs: f64) {
        self.e[0] += rhs;
        self.e[1] += rhs;
        self.e[2] += rhs;
    }
}

impl std::ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3::from(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
    }
}

impl std::ops::Sub<f64> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: f64) -> Self::Output {
        Vec3::from(self.x() - rhs, self.y() - rhs, self.z() - rhs)
    }
}

impl std::ops::SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        self.e[0] -= rhs.x();
        self.e[1] -= rhs.y();
        self.e[2] -= rhs.z();
    }
}

impl std::ops::SubAssign<f64> for Vec3 {
    fn sub_assign(&mut self, rhs: f64) {
        self.e[0] -= rhs;
        self.e[1] -= rhs;
        self.e[2] -= rhs;
    }
}

impl std::ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::from(self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z())
    }
}

impl std::ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3::from(self.x() * rhs, self.y() * rhs, self.z() * rhs)
    }
}

impl std::ops::MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, rhs: Vec3) {
        self.e[0] *= rhs.x();
        self.e[1] *= rhs.y();
        self.e[2] *= rhs.z();
    }
}

impl std::ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.e[0] *= rhs;
        self.e[1] *= rhs;
        self.e[2] *= rhs;
    }
}

impl std::ops::Div<Vec3> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Self::Output {
        Vec3::from(self.x() / rhs.x(), self.y() / rhs.y(), self.z() / rhs.z())
    }
}

impl std::ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        Vec3::from(self.x() / rhs, self.y() / rhs, self.z() / rhs)
    }
}

impl std::ops::DivAssign<Vec3> for Vec3 {
    fn div_assign(&mut self, rhs: Vec3) {
        self.e[0] /= rhs.x();
        self.e[1] /= rhs.y();
        self.e[2] /= rhs.z();
    }
}

impl std::ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.e[0] /= rhs;
        self.e[1] /= rhs;
        self.e[2] /= rhs;
    }
}

pub fn dot(v1: Vec3, v2: Vec3) -> f64 {
    v1.dot(v2)
}

pub fn cross(v1: Vec3, v2: Vec3) -> Vec3 {
    v1.cross(v2)
}

pub fn reflect(v1: Vec3, v2: Vec3) -> Vec3 {
    v1 - (v2 * dot(v1, v2) * 2.0)
}

pub fn refract(v1: Vec3, v2: Vec3, ratio: f64) -> Vec3 {
    let cos_theta = dot(v1 * -1.0, v2).min(1.0);
    let r_out_perp = (v1 + (v2 * cos_theta)) * ratio;
    let r_out_parallel = v2 * -(1.0 - r_out_perp.norm_squared()).abs().sqrt();
    r_out_perp + r_out_parallel
}

fn _dot(v1: &[f64; 3], v2: &[f64; 3]) -> f64 {
    v1[0] * v2[0] + v1[1] * v2[2] + v1[3] * v2[3]
}

fn _cross(v1: &[f64; 3], v2: &[f64; 3]) -> [f64; 3] {
    [
        v1[1] * v2[2] - v1[2] * v2[1],
        v1[2] * v2[0] - v1[0] * v2[2],
        v1[0] * v2[1] - v1[1] * v2[0],
    ]
}
