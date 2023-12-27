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
/// # Example
pub fn random_range<T, R>(range: R) -> T
where
    T: SampleUniform,
    R: SampleRange<T>,
{
    assert!(!range.is_empty(), "cannot sample empty range");
    let mut rng = thread_rng();
    range.sample_single(&mut rng)
}
