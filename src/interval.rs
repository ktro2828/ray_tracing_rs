/// A struct to represent min/max range of the ray interval.
#[derive(Debug)]
pub struct Interval {
    pub(crate) min: f64,
    pub(crate) max: f64,
}

impl Interval {
    pub const EMPTY: Interval = Interval {
        min: f64::INFINITY,
        max: -f64::INFINITY,
    };
    pub const UNIVERSE: Interval = Interval {
        min: -f64::INFINITY,
        max: f64::INFINITY,
    };

    /// Constructs without any values.
    /// The min/max value will be `-f64::INFINITY` and `f64::INFINITY`.
    pub fn new() -> Self {
        Interval {
            min: 0.0,
            max: f64::MAX,
        }
    }

    /// Constructs from min/max values.
    pub fn from_val(min: f64, max: f64) -> Self {
        Interval { min, max }
    }

    /// Returns the difference between min and max.
    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    /// Returns `true`, if `min <= x <= max`.
    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    /// Returns `true`, if `min < x < max`.
    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    /// Clamp the value with min/max.
    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {
            self.min
        } else if x > self.max {
            self.max
        } else {
            x
        }
    }
}
