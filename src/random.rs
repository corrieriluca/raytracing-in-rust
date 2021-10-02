use rand::{thread_rng, Rng};

/// Returns a random real number in [0.0, 1.0[
pub fn canonical_random() -> f64 {
    thread_rng().gen_range(0.0..1.0)
}

/// Returns a random real number in [`min`, `max`[
pub fn random_range(min: f64, max: f64) -> f64 {
    thread_rng().gen_range(min..max)
}
