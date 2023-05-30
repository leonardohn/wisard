use bitvec::{order::BitOrder, store::BitStore};
use rand::{Rng, RngCore, SeedableRng};
use rand_xoshiro::SplitMix64;

use crate::encode::SampleEncoder;
use crate::sample::{Label, Sample};

/// An encoder that permutes the sample bits according to a given random seed.
#[derive(Copy, Clone)]
pub struct Permute<R = SplitMix64>
where
    R: RngCore + SeedableRng,
    <R as SeedableRng>::Seed: Copy,
{
    seed: <R as SeedableRng>::Seed,
}

impl<R> Permute<R>
where
    R: RngCore + SeedableRng,
    <R as SeedableRng>::Seed: Copy,
{
    /// Creates a new [`Permute`](./structs.Permute.html) encoder instance
    /// using `rand::random()` as the permutation seed.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new [`Permute`](./structs.Permute.html) encoder instance
    /// using a given `seed` as the permutation seed.
    pub fn with_seed(seed: <R as SeedableRng>::Seed) -> Self {
        Self { seed }
    }

    /// Returns the internal permutation seed.
    pub fn seed(&self) -> &<R as SeedableRng>::Seed {
        &self.seed
    }
}

impl<R> Default for Permute<R>
where
    R: RngCore + SeedableRng,
    <R as SeedableRng>::Seed: Copy,
{
    fn default() -> Self {
        let mut seed = <R as SeedableRng>::Seed::default();
        let mut rng = rand::thread_rng();
        rng.fill_bytes(seed.as_mut());
        Self::with_seed(seed)
    }
}

impl<L, S, O, R> SampleEncoder<L, S, O> for Permute<R>
where
    L: Label,
    O: BitOrder,
    R: RngCore + SeedableRng,
    S: BitStore,
    <R as SeedableRng>::Seed: Copy,
{
    fn encode_inplace(&self, sample: &mut Sample<L, S, O>) {
        let mut rng = R::from_seed(self.seed);
        let bits = sample.raw_bits_mut();
        let m = bits.len() - 1;
        for i in 0..m {
            bits.swap(i, rng.gen_range(i..=m));
        }
    }
}

#[cfg(test)]
mod tests {
    use bitvec::prelude::*;

    use super::*;

    #[test]
    fn permute_fixed_seed() {
        let sample_1 =
            Sample::from_raw_parts(bitvec![0, 0, 0, 0, 1, 1, 1, 1], 1, 0usize);
        let sample_2 =
            Sample::from_raw_parts(bitvec![0, 1, 0, 1, 0, 1, 0, 1], 1, 0usize);
        let sample_1_perm =
            Sample::from_raw_parts(bitvec![1, 0, 0, 0, 0, 1, 1, 1], 1, 0usize);
        let sample_2_perm =
            Sample::from_raw_parts(bitvec![0, 0, 1, 1, 0, 1, 0, 1], 1, 0usize);
        let seed = (0xABAD5EED_u64).to_le_bytes();
        let permute = <Permute>::with_seed(seed);
        assert_eq!(permute.encode(sample_1), sample_1_perm);
        assert_eq!(permute.encode(sample_2), sample_2_perm);
    }
}
