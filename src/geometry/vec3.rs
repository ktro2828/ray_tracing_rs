use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    /// Returns Vec3 built from specified values.
    ///
    /// # Examples
    /// ```
    /// use raytrs::geometry::Vec3;
    ///
    /// let v = Vec3::new(1.0, 2.0, 3.0);
    /// ```
    pub fn new(e0: f64, e1: f64, e2: f64) -> Self {
        let e = [e0, e1, e2];
        Vec3 { e }
    }

    /// Returns Vec3 filled by 0.0.
    ///
    /// # Examples
    /// ```
    /// use raytrs::geometry::Vec3;
    ///
    /// let v = Vec3::zeros();
    /// assert_eq!(v, Vec3::new(0.0, 0.0, 0.0));
    /// ```
    pub fn zeros() -> Self {
        let e = [0.0, 0.0, 0.0];
        Vec3 { e }
    }

    /// Returns Vec3 filled by 1.0.
    ///
    /// # Examples
    /// ```
    /// use raytrs::geometry::Vec3;
    ///
    /// let v = Vec3::ones();
    /// assert_eq!(v, Vec3::new(1.0, 1.0, 1.0));
    /// ```
    pub fn ones() -> Self {
        let e = [1.0, 1.0, 1.0];
        Vec3 { e }
    }

    /// Returns Vec3 build from random values.
    ///
    /// # Examples
    /// ```
    /// use raytrs::geometry::Vec3;
    ///
    /// let v = Vec3::rand();
    /// ```
    pub fn rand() -> Self {
        let mut rng = rand::thread_rng();
        let e = [rng.gen(), rng.gen(), rng.gen()];
        Vec3 { e }
    }

    /// Returns x value reference.
    ///
    /// # Examples
    /// ```
    /// use raytrs::geometry::Vec3;
    ///
    /// let v = Vec3::zeros();
    /// assert_eq!(v.x(), &0.0);
    /// ```
    pub fn x(&self) -> &f64 {
        &self.e[0]
    }

    /// Returns y value as reference.
    ///
    /// # Examples
    /// ```
    /// use raytrs::geometry::Vec3;
    ///
    /// let v = Vec3::zeros();
    /// assert_eq!(v.y(), &0.0);
    /// ```
    pub fn y(&self) -> &f64 {
        &self.e[1]
    }

    /// Returns z value as reference.
    ///
    /// # Examples
    /// ```
    /// use raytrs::geometry::Vec3;
    ///
    /// let v = Vec3::zeros();
    /// assert_eq!(v.z(), &0.0);
    /// ```
    pub fn z(&self) -> &f64 {
        &self.e[2]
    }

    /// Returns dot product.
    ///
    /// # Example
    /// ```
    /// use raytrs::geometry::Vec3;
    ///
    /// let v1 = Vec3::ones();
    /// let v2 = Vec3::ones();
    /// let d = v1.dot(v2);
    /// assert_eq!(d, 3.0);
    /// ```
    pub fn dot(&self, rhs: Vec3) -> f64 {
        _dot(&self.e, &rhs.e)
    }

    /// Returns cross product.
    ///
    /// # Example
    /// ```
    /// use raytrs::geometry::Vec3;
    ///
    /// let v1 = Vec3::ones();
    /// let v2 = Vec3::ones();
    /// let c = v1.cross(v2);
    /// assert_eq!(c, Vec3::new(0.0, 0.0, 0.0));
    /// ```
    pub fn cross(&self, rhs: Vec3) -> Vec3 {
        let e = _cross(&self.e, &rhs.e);
        Vec3 { e }
    }

    pub fn reflect(&self, rhs: Vec3) -> Vec3 {
        reflect(*self, rhs)
    }

    pub fn refract(&self, rhs: Vec3, ratio: f64) -> Option<Vec3> {
        refract(*self, rhs, ratio)
    }

    /// Returns norm value.
    ///
    /// # Example
    /// ```
    /// use raytrs::geometry::Vec3;
    ///
    /// let v = Vec3::ones();
    /// let n = v.norm();
    /// assert_eq!(n, 1.7320508075688772);
    /// ```
    pub fn norm(&self) -> f64 {
        self.norm_squared().sqrt()
    }

    /// Returns squared norm value.
    ///
    /// # Example
    /// ```
    /// use raytrs::geometry::Vec3;
    ///
    /// let v = Vec3::ones();
    /// let n = v.norm_squared();
    /// assert_eq!(n, 3.0);
    /// ```
    pub fn norm_squared(&self) -> f64 {
        self.x().powi(2) + self.y().powi(2) + self.z().powi(2)
    }

    /// Returns Vec3 as unit vector.
    ///
    /// # Example
    /// ```
    /// use raytrs::geometry::Vec3;
    ///
    /// let v = Vec3::ones();
    /// let u = v.as_unit();
    /// assert_eq!(u, Vec3::new(1.0 / 1.7320508075688772, 1.0 / 1.7320508075688772, 1.0 / 1.7320508075688772));
    /// ```
    pub fn as_unit(&self) -> Self {
        *self / self.norm()
    }

    /// Returns random Vec3 as unit vector.
    ///
    /// # Example
    /// ```
    /// use raytrs::geometry::Vec3;
    ///
    /// let v = Vec3::rand_unit();
    /// ```
    pub fn rand_unit() -> Self {
        Vec3::rand().as_unit()
    }

    /// Returns whether elements are close to specified value.
    ///
    /// # Example
    /// ```
    /// use raytrs::geometry::Vec3;
    ///
    /// let v1 = Vec3::ones();
    /// assert!(v1.is_close(1.0));
    /// ```
    pub fn is_close(&self, x: f64) -> bool {
        const TOLERANCE: f64 = 1e-8;
        return (self.e[0] - x).abs() < TOLERANCE
            && (self.e[1] - x).abs() < TOLERANCE
            && (self.e[2] - x).abs() < TOLERANCE;
    }
}

/// The addition operator `Vec3 + Vec3`.
///
/// # Example
/// ```
/// use raytrs::geometry::Vec3;
///
/// let v1 = Vec3::ones();
/// let v2 = Vec3::ones();
/// let a = v1 + v2;
/// assert_eq!(a, Vec3::new(2.0, 2.0, 2.0));
/// ```
impl std::ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}

/// The addition operator `Vec3 + f64`.
///
/// # Example
/// ```
/// use raytrs::geometry::Vec3;
///
/// let v = Vec3::ones();
/// let a = v + 1.0;
/// assert_eq!(a, Vec3::new(2.0, 2.0, 2.0));
/// ```
impl std::ops::Add<f64> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: f64) -> Self::Output {
        Vec3::new(self.x() + rhs, self.y() + rhs, self.z() + rhs)
    }
}

/// The addition assignment operator `+= Vec3`.
///
/// # Example
/// ```
/// use raytrs::geometry::Vec3;
///
/// let mut v1 = Vec3::ones();
/// let v2 = Vec3::ones();
/// v1 += v2;
/// assert_eq!(v1, Vec3::new(2.0, 2.0, 2.0));
/// ```
impl std::ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.e[0] += rhs.x();
        self.e[1] += rhs.y();
        self.e[2] += rhs.z();
    }
}

/// The addition assignment operator `+= f64`.
///
/// # Example
/// ```
/// use raytrs::geometry::Vec3;
///
/// let mut v = Vec3::ones();
/// v += 1.0;
/// assert_eq!(v, Vec3::new(2.0, 2.0, 2.0));
/// ```
impl std::ops::AddAssign<f64> for Vec3 {
    fn add_assign(&mut self, rhs: f64) {
        self.e[0] += rhs;
        self.e[1] += rhs;
        self.e[2] += rhs;
    }
}

/// The subtraction operator `Vec3 - Vec3`
///
/// # Example
/// ```
/// use raytrs::geometry::Vec3;
///
/// let v1 = Vec3::ones();
/// let v2 = Vec3::ones();
/// let a = v1 - v2;
/// assert_eq!(a, Vec3::zeros());
/// ```
impl std::ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
    }
}

/// The subtraction operator `Vec3 - f64`.
///
/// # Example
/// ```
/// use raytrs::geometry::Vec3;
///
/// let v = Vec3::ones();
/// let a = v - 1.0;
/// assert_eq!(a, Vec3::zeros());
/// ```
impl std::ops::Sub<f64> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: f64) -> Self::Output {
        Vec3::new(self.x() - rhs, self.y() - rhs, self.z() - rhs)
    }
}

/// The subtraction assignment operator `-= Vec3`.
///
/// # Example
/// ```
/// use raytrs::geometry::Vec3;
///
/// let mut v1 = Vec3::ones();
/// let v2 = Vec3::ones();
/// v1 -= v2;
/// assert_eq!(v1, Vec3::zeros());
/// ```
impl std::ops::SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        self.e[0] -= rhs.x();
        self.e[1] -= rhs.y();
        self.e[2] -= rhs.z();
    }
}

/// The subtraction assignment operator `-= f64`.
///
/// # Example
/// ```
/// use raytrs::geometry::Vec3;
///
/// let mut v = Vec3::ones();
/// v -= 1.0;
/// assert_eq!(v, Vec3::zeros());
/// ```
impl std::ops::SubAssign<f64> for Vec3 {
    fn sub_assign(&mut self, rhs: f64) {
        self.e[0] -= rhs;
        self.e[1] -= rhs;
        self.e[2] -= rhs;
    }
}

/// The multiplication operator `Vec3 * Vec3`.
///
/// # Example
/// ```
/// use raytrs::geometry::Vec3;
///
/// let v1 = Vec3::ones();
/// let v2 = Vec3::zeros();
/// let a = v1 * v2;
/// assert_eq!(a, Vec3::zeros());
/// ```
impl std::ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z())
    }
}

/// The multiplication operator `Vec3 * f64`.
///
/// # Example
/// ```
/// use raytrs::geometry::Vec3;
///
/// let v = Vec3::ones();
/// let a = v * 0.0;
/// assert_eq!(a, Vec3::zeros());
/// ```
impl std::ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3::new(self.x() * rhs, self.y() * rhs, self.z() * rhs)
    }
}

impl std::ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self * rhs.x(), self * rhs.y(), self * rhs.z())
    }
}

/// The multiplication assignment operator `*= Vec3`.
///
/// # Example
/// ```
/// use raytrs::geometry::Vec3;
///
/// let mut v1 = Vec3::ones();
/// let v2 = Vec3::zeros();
/// v1 *= v2;
/// assert_eq!(v1, Vec3::zeros());
/// ```
impl std::ops::MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, rhs: Vec3) {
        self.e[0] *= rhs.x();
        self.e[1] *= rhs.y();
        self.e[2] *= rhs.z();
    }
}

/// The multiplication assignment operator `*= f64`.
///
/// # Example
/// ```
/// use raytrs::geometry::Vec3;
///
/// let mut v = Vec3::ones();
/// v *= 0.0;
/// assert_eq!(v, Vec3::zeros());
/// ```
impl std::ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.e[0] *= rhs;
        self.e[1] *= rhs;
        self.e[2] *= rhs;
    }
}

/// The division operator `Vec3 / Vec3`.
///
/// # Example
/// ```
/// use raytrs::geometry::Vec3;
///
/// let v1 = Vec3::ones();
/// let v2 = Vec3::ones();
/// let a = v1 / v2;
/// assert_eq!(a, Vec3::ones());
/// ```
impl std::ops::Div<Vec3> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self.x() / rhs.x(), self.y() / rhs.y(), self.z() / rhs.z())
    }
}

/// The division operator `Vec3 / f64`.
///
/// # Example
/// ```
/// use raytrs::geometry::Vec3;
///
/// let v = Vec3::ones();
/// let a = v / 1.0;
/// assert_eq!(v, Vec3::ones());
/// ```
impl std::ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        Vec3::new(self.x() / rhs, self.y() / rhs, self.z() / rhs)
    }
}

/// The division assignment operator `/= Vec3`.
///
/// # Example
/// ```
/// use raytrs::geometry::Vec3;
///
/// let mut v1 = Vec3::ones();
/// let v2 = Vec3::ones();
/// v1 /= v2;
/// assert_eq!(v1, Vec3::ones());
/// ```
impl std::ops::DivAssign<Vec3> for Vec3 {
    fn div_assign(&mut self, rhs: Vec3) {
        self.e[0] /= rhs.x();
        self.e[1] /= rhs.y();
        self.e[2] /= rhs.z();
    }
}

/// The division assignment operator `/= f64`.
///
/// # Example
/// ```
/// use raytrs::geometry::Vec3;
///
/// let mut v = Vec3::ones();
/// v /= 1.0;
/// assert_eq!(v, Vec3::ones());
/// ```
impl std::ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.e[0] /= rhs;
        self.e[1] /= rhs;
        self.e[2] /= rhs;
    }
}

impl std::ops::Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        Vec3::new(-self.e[0], -self.e[1], -self.e[2])
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

pub fn refract(v1: Vec3, v2: Vec3, ratio: f64) -> Option<Vec3> {
    let uv = v1.as_unit();
    let dt = uv.dot(v2);
    let d = 1.0 - ratio.powi(2) * (1.0 - dt.powi(2));
    if d > 0.0 {
        Some(-ratio * (uv - v2 * dt) - v2 * d.sqrt())
    } else {
        None
    }
}

fn _dot(v1: &[f64; 3], v2: &[f64; 3]) -> f64 {
    v1[0] * v2[0] + v1[1] * v2[1] + v1[2] * v2[2]
}

fn _cross(v1: &[f64; 3], v2: &[f64; 3]) -> [f64; 3] {
    [
        v1[1] * v2[2] - v1[2] * v2[1],
        v1[2] * v2[0] - v1[0] * v2[2],
        v1[0] * v2[1] - v1[1] * v2[0],
    ]
}
