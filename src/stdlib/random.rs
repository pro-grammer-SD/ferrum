//! Random module - random number generation
//!
//! This module provides functions for generating random numbers and making random selections.

use rand::Rng;

/// Generate a random integer between min (inclusive) and max (exclusive)
pub fn randint(min: i64, max: i64) -> i64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}

/// Generate a random float between 0.0 and 1.0
pub fn random() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen::<f64>()
}

/// Generate a random float between min and max
pub fn uniform(min: f64, max: f64) -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}

/// Randomly shuffle a vector
pub fn shuffle<T: Clone>(items: &[T]) -> Vec<T> {
    use rand::seq::SliceRandom;
    let mut rng = rand::thread_rng();
    let mut shuffled = items.to_vec();
    shuffled.shuffle(&mut rng);
    shuffled
}

/// Pick a random element from a list
pub fn choice<T: Clone>(items: &[T]) -> Option<T> {
    use rand::seq::SliceRandom;
    let mut rng = rand::thread_rng();
    items.choose(&mut rng).cloned()
}

/// Generate random boolean
pub fn random_bool() -> bool {
    let mut rng = rand::thread_rng();
    rng.gen_bool(0.5)
}
