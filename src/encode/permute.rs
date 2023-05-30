use std::marker::PhantomData;

use bitvec::{order::BitOrder, store::BitStore};
use rand::{Rng, RngCore, SeedableRng};
use rand_xoshiro::SplitMix64;

use crate::encode::SampleEncoder;
use crate::sample::{Label, Sample};

/// An encoder that permutes the sample bits according to a given random seed.
#[derive(Clone, Debug)]
pub struct Permute<R = SplitMix64>
where
    R: RngCore + SeedableRng,
{
    seed: u64,
    _phantom: PhantomData<R>,
}

impl<R: RngCore + SeedableRng> Permute<R> {
    /// Creates a new [`Permute`](./structs.Permute.html) encoder instance
    /// using `rand::random()` as the permutation seed.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new [`Permute`](./structs.Permute.html) encoder instance
    /// using a given `seed` as the permutation seed.
    pub fn with_seed(seed: u64) -> Self {
        let _phantom = PhantomData;
        Self { seed, _phantom }
    }

    /// Returns the internal permutation seed.
    pub fn seed(&self) -> u64 {
        self.seed
    }
}

impl<R: RngCore + SeedableRng> Default for Permute<R> {
    fn default() -> Self {
        Self::with_seed(rand::random())
    }
}

impl<L, S, O, R> SampleEncoder<L, S, O> for Permute<R>
where
    L: Label,
    O: BitOrder,
    R: RngCore + SeedableRng,
    S: BitStore,
{
    fn encode_inplace(&self, sample: &mut Sample<L, S, O>) {
        let mut rng = R::seed_from_u64(self.seed);
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
            Sample::from_raw_parts(bitvec![0, 0, 1, 1, 0, 1, 1, 0], 1, 0usize);
        let sample_2_perm =
            Sample::from_raw_parts(bitvec![1, 1, 1, 1, 0, 0, 0, 0], 1, 0usize);
        let permute = <Permute>::with_seed(7);
        assert_eq!(permute.encode(sample_1), sample_1_perm);
        assert_eq!(permute.encode(sample_2), sample_2_perm);
    }
}
