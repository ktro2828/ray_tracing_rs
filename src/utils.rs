use rand::{
    distributions::{
        uniform::{SampleRange, SampleUniform},
        Distribution, Standard,
    },
    thread_rng,
};

/// Generate a random value.
///
/// # Example
///
/// ```
/// use raytrs::utils::random;
///
/// let v = random::<f64>();
/// assert!(0.0 <= v && v <= 1.0);
/// ```
pub fn random<T>() -> T
where
    Standard: Distribution<T>,
{
    let mut rng = thread_rng();
    Standard.sample(&mut rng)
}

/// Generate a random value in specified range.
///
/// # Arguments
/// * `range`   - The Range to sample number.
///
/// # Example
/// ```
/// use raytrs::utils::random_range;
///
/// let v = random_range(0..100);
/// assert!(0 <= v && v <= 100);
/// ```
pub fn random_range<T, R>(range: R) -> T
where
    T: SampleUniform,
    R: SampleRange<T>,
{
    assert!(!range.is_empty(), "cannot sample empty range");
    let mut rng = thread_rng();
    range.sample_single(&mut rng)
}
