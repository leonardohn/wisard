use std::collections::{HashMap, HashSet};

use bitvec::{prelude::BitOrder, store::BitStore};

use crate::{
    encode::{Permute, SampleEncoder},
    filter::{BuildFilter, Filter, PackedLUTFilter, PackedLUTFilterBuilder},
    model::Discriminator,
    sample::{Label, Sample},
};

/// A wrapper around [`WisardBase`](./struct.WisardBase.html) for a traditional
/// WiSARD model, using boolean values to store its internal state.
#[derive(Clone, Debug)]
pub struct BinaryWisard<L: Label> {
    base: WisardBase<L, PackedLUTFilter>,
    seed: [u8; 32],
}

impl<L: Label> BinaryWisard<L> {
    /// Creates a new [`BinaryWisard`](./struct.BinaryWisard.html) instance
    /// using `rand::random()` as the permutation seed.
    ///
    /// The `input_size` value determines the total number of input bits.
    /// The `addr_size` value corresponds to the address size of the RAMs.
    /// The `labels` set must contain all the expected sample labels.
    pub fn new(
        input_size: usize,
        addr_size: usize,
        labels: HashSet<L>,
    ) -> Self {
        Self::with_seed(input_size, addr_size, labels, rand::random())
    }

    /// Creates a new [`BinaryWisard`](./struct.BinaryWisard.html) instance
    /// using a given permutation seed.
    ///
    /// The `input_size` value determines the total number of input bits.
    /// The `addr_size` value corresponds to the address size of the RAMs.
    /// The `labels` set must contain all the expected sample labels.
    /// The `seed` value determines the permutation seed.
    pub fn with_seed(
        input_size: usize,
        addr_size: usize,
        labels: HashSet<L>,
        seed: [u8; 32],
    ) -> Self {
        let builder = PackedLUTFilterBuilder::new(addr_size, 1, 0);
        let base = WisardBase::from_filter_builder(
            input_size, addr_size, labels, &builder,
        );
        Self { base, seed }
    }

    /// Returns the internal random seed for the model.
    pub fn seed(&self) -> [u8; 32] {
        self.seed
    }

    /// Fits (trains) the model with a given input sample.
    pub fn fit(&mut self, sample: &Sample<L>) {
        let encoder = <Permute>::with_seed(self.seed);
        let sample = encoder.encode(sample.clone());
        self.base.fit(&sample)
    }

    /// Returns the model scores for a given input sample.
    pub fn scores(&self, sample: &Sample<L>) -> Vec<(usize, L)> {
        let encoder = <Permute>::with_seed(self.seed);
        let sample = encoder.encode(sample.clone());
        self.base.scores(&sample)
    }

    /// Returns the model prediction for a given input sample.
    pub fn predict(&self, sample: &Sample<L>) -> L {
        let encoder = <Permute>::with_seed(self.seed);
        let sample = encoder.encode(sample.clone());
        self.base.predict(&sample)
    }
}

/// The base for a WiSARD model that only includes the discriminators.
#[derive(Clone, Debug)]
pub struct WisardBase<L, F>
where
    L: Label,
    F: Filter,
{
    disc: HashMap<L, Discriminator<F>>,
}

impl<L, F> WisardBase<L, F>
where
    L: Label,
    F: Filter,
{
    /// Creates a new [`WisardBase`](./struct.WisardBase.html) instance.
    ///
    /// The `input_size` value determines the total number of input bits.
    /// The `addr_size` value corresponds to the address size of the RAMs.
    /// The `labels` set must contain all the expected sample labels.
    /// The `builder` value must be an instance of a type which implements
    /// the [`FilterBuilder`](./trait.FilterBuilder.html) trait, using the same
    /// `addr_size` as provided before and serving as a backend for the RAMs.
    pub fn from_filter_builder<B>(
        input_size: usize,
        addr_size: usize,
        labels: HashSet<L>,
        builder: &B,
    ) -> Self
    where
        B: BuildFilter<Filter = F>,
    {
        Self {
            disc: labels
                .into_iter()
                .map(|label| {
                    (
                        label,
                        Discriminator::from_filter_builder(
                            input_size, addr_size, builder,
                        ),
                    )
                })
                .collect(),
        }
    }

    /// Fits (trains) the model with a given input sample.
    pub fn fit<S, O>(&mut self, sample: &Sample<L, S, O>)
    where
        O: BitOrder + Clone,
        S: BitStore + Clone,
    {
        self.disc.get_mut(sample.label()).unwrap().fit(sample)
    }

    /// Returns the model scores for a given input sample.
    pub fn scores<S, O>(&self, sample: &Sample<L, S, O>) -> Vec<(usize, L)>
    where
        O: BitOrder + Clone,
        S: BitStore + Clone,
    {
        self.disc
            .keys()
            .map(|label| (self.disc[label].score(sample), label.clone()))
            .collect()
    }

    /// Returns the model prediction for a given input sample.
    pub fn predict<S, O>(&self, sample: &Sample<L, S, O>) -> L
    where
        O: BitOrder + Clone,
        S: BitStore + Clone,
    {
        self.scores(sample)
            .into_iter()
            .max_by(|a, b| a.0.cmp(&b.0))
            .unwrap()
            .1
    }
}

#[cfg(test)]
mod tests {
    use bitvec::prelude::*;

    use crate::sample::Sample;

    use super::*;

    #[test]
    fn binary_wisard_hot_cold() {
        let input_size = 8;
        let addr_size = 2;
        let labels = HashSet::from_iter(vec!["cold", "hot"].into_iter());
        let mut model = BinaryWisard::new(input_size, addr_size, labels);

        let samples = vec![
            (bitvec![1, 1, 1, 0, 0, 0, 0, 0], "cold"),
            (bitvec![1, 1, 1, 1, 0, 0, 0, 0], "cold"),
            (bitvec![0, 0, 0, 0, 1, 1, 1, 1], "hot"),
            (bitvec![0, 0, 0, 0, 0, 1, 1, 1], "hot"),
        ];

        let encoded_samples = samples
            .into_iter()
            .map(|(v, l)| Sample::from_raw_parts(v, addr_size, l))
            .collect::<Vec<_>>();

        for sample in encoded_samples.iter() {
            model.fit(sample);
        }

        for sample in encoded_samples.iter() {
            let pred = model.predict(sample);
            assert_eq!(&pred, sample.label());
        }
    }
}
