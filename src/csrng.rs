use rand::Rng;
use rand_chacha::rand_core::SeedableRng;
use rand_chacha::ChaCha20Rng;

pub struct Csrng {
    generator: ChaCha20Rng,
}

impl Csrng {
    pub fn new() -> Self {
        Self {
            generator: ChaCha20Rng::from_entropy(),
        }
    }

    pub fn random(&mut self, below: usize) -> usize {
        self.generator.gen_range(0..below)
    }
}
