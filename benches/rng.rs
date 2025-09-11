/// A tiny deterministic linear congruential generator so we don't need external crates.
/// Not cryptographically secure — only for reproducible pseudo-random inputs in benchmarks.
#[derive(Clone)]
pub struct Lcg(u64);

impl Lcg {
    pub fn new(seed: u64) -> Self {
        Self(seed)
    }

    pub fn next_u64(&mut self) -> u64 {
        // Parameters from Numerical Recipes (common LCG)
        self.0 = self
            .0
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        self.0
    }

    pub fn next_usize(&mut self, max: usize) -> usize {
        (self.next_u64() as usize) % max
    }
}
