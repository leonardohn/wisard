use std::marker::PhantomData;

use bitvec::{order::BitOrder, store::BitStore};
use rand::{RngCore, SeedableRng};
use rand_xoshiro::Xoshiro256PlusPlus;

use crate::encode::SampleEncoder;
use crate::sample::{Label, Sample};

/// An encoder that shuffles the sample bits according to a given random seed.
#[derive(Clone, Debug)]
pub struct Shuffle<R = Xoshiro256PlusPlus>
where
    R: RngCore + SeedableRng,
{
    seed: u64,
    _phantom: PhantomData<R>,
}

impl<R: RngCore + SeedableRng> Shuffle<R> {
    /// Creates a new [`Shuffle`](./structs.Shuffle.html) instance using
    /// `rand::random()` as the random seed.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new [`Shuffle`](./structs.Shuffle.html) instance using
    /// a given random seed.
    pub fn with_seed(seed: u64) -> Self {
        let _phantom = PhantomData;
        Self { seed, _phantom }
    }

    /// Returns the internal random seed.
    pub fn seed(&self) -> u64 {
        self.seed
    }
}

impl<R: RngCore + SeedableRng> Default for Shuffle<R> {
    fn default() -> Self {
        Self::with_seed(rand::random())
    }
}

impl<L, S, O, R> SampleEncoder<L, S, O> for Shuffle<R>
where
    L: Label,
    O: BitOrder,
    R: RngCore + SeedableRng,
    S: BitStore,
{
    fn encode_inplace(&self, sample: &mut Sample<L, S, O>) {
        let mut rng = R::seed_from_u64(self.seed);
        let bits = sample.raw_bits_mut();
        for i in (0..bits.len()).rev() {
            let j = (rng.next_u64() as usize) % (i + 1);
            bits.swap(i, j);
        }
    }
}

#[cfg(test)]
mod tests {
    use bitvec::{bitvec, order::Lsb0};

    use super::*;

    #[test]
    fn shuffle_fixed_seed() {
        let sample_1 =
            Sample::from_raw_parts(bitvec![0, 0, 0, 0, 1, 1, 1, 1], 1, 0usize);
        let sample_2 =
            Sample::from_raw_parts(bitvec![0, 1, 0, 1, 0, 1, 0, 1], 1, 0usize);
        let sample_1_shuf =
            Sample::from_raw_parts(bitvec![1, 0, 0, 1, 0, 0, 1, 1], 1, 0usize);
        let sample_2_shuf =
            Sample::from_raw_parts(bitvec![0, 1, 0, 1, 1, 0, 0, 1], 1, 0usize);
        let shuffle = <Shuffle>::with_seed(7);
        assert_eq!(shuffle.encode(sample_1), sample_1_shuf);
        assert_eq!(shuffle.encode(sample_2), sample_2_shuf);
    }
}
