use bitvec::{order::BitOrder, store::BitStore};
use rand::{Rng, RngCore, SeedableRng};
use rand_xoshiro::Xoshiro256PlusPlus;

use crate::encode::SampleEncoder;
use crate::sample::{Label, Sample};

/// An encoder that permutes the sample bits according to a given random seed.
#[derive(Clone)]
pub struct Permute<R = Xoshiro256PlusPlus>
where
    R: RngCore + SeedableRng,
    <R as SeedableRng>::Seed: Clone,
{
    seed: <R as SeedableRng>::Seed,
}

impl<R> Permute<R>
where
    R: RngCore + SeedableRng,
    <R as SeedableRng>::Seed: Clone,
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
    <R as SeedableRng>::Seed: Clone,
{
    fn default() -> Self {
        let mut seed = <R as SeedableRng>::Seed::default();
        let mut rng = rand::thread_rng();
        rng.fill_bytes(seed.as_mut());
        Self::with_seed(seed.clone())
    }
}

impl<L, T, O, R> SampleEncoder<L, T, O> for Permute<R>
where
    L: Label,
    T: BitStore,
    O: BitOrder,
    R: RngCore + SeedableRng,
    <R as SeedableRng>::Seed: Clone,
{
    fn encode_inplace(&self, sample: &mut Sample<L, T, O>) {
        let mut rng = R::from_seed(self.seed.clone());
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
            Sample::from_raw_parts(bitvec![1, 1, 1, 0, 0, 0, 1, 0], 1, 0usize);
        let sample_2_perm =
            Sample::from_raw_parts(bitvec![0, 0, 1, 1, 0, 0, 1, 1], 1, 0usize);
        let seed = 0xBAD_5EED_u32.to_le_bytes().repeat(8).try_into().unwrap();
        let permute = <Permute>::with_seed(seed);
        assert_eq!(permute.encode(sample_1), sample_1_perm);
        assert_eq!(permute.encode(sample_2), sample_2_perm);
    }
}
