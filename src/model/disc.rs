use bitvec::{order::BitOrder, store::BitStore, view::BitView};

use crate::{
    filter::{BuildFilter, Filter},
    sample::{Label, Sample},
};

/// A WiSARD discriminator structure.
#[derive(Clone, Debug)]
pub struct Discriminator<F>
where
    F: Filter,
{
    input_size: usize,
    addr_size: usize,
    filters: Vec<F>,
}

impl<F> Discriminator<F>
where
    F: Filter,
{
    /// Creates a new [`Discriminator`](./struct.Discriminator.html) instance.
    ///
    /// The `input_size` value determines the total number of input bits.
    /// The `addr_size` value corresponds to the address size of the RAMs.
    /// The `builder` value must be an instance of a type which implements
    /// the [`FilterBuilder`](./trait.FilterBuilder.html) trait, using the same
    /// `addr_size` as provided before and serving as a backend for the RAMs.
    pub fn from_filter_builder<B>(
        input_size: usize,
        addr_size: usize,
        builder: &B,
    ) -> Self
    where
        B: BuildFilter<Filter = F>,
    {
        let num_filters = (input_size + addr_size - 1) / addr_size;
        let mut filters = Vec::with_capacity(num_filters);

        for _ in 0..num_filters {
            filters.push(builder.build_filter());
        }

        Self {
            input_size,
            addr_size,
            filters,
        }
    }

    /// Returns the discriminator input size.
    pub fn input_size(&self) -> usize {
        self.input_size
    }

    /// Returns the discriminator address size.
    pub fn addr_size(&self) -> usize {
        self.addr_size
    }

    /// Fits (trains) the discriminator with a given input sample.
    pub fn fit<L, S, O>(&mut self, sample: &Sample<L, S, O>)
    where
        L: Label,
        O: BitOrder,
        S: BitStore,
    {
        sample
            .raw_bits()
            .chunks(self.addr_size)
            .enumerate()
            .for_each(|(i, v)| {
                let mut addr = 0usize;
                addr.view_bits_mut::<O>()[..v.len()].clone_from_bitslice(v);
                self.filters[i].include(addr);
            })
    }

    /// Returns the discriminator score for a given input sample.
    pub fn score<L, S, O>(&self, sample: &Sample<L, S, O>) -> usize
    where
        L: Label,
        O: BitOrder,
        S: BitStore,
    {
        sample
            .raw_bits()
            .chunks(self.addr_size)
            .enumerate()
            .map(|(i, v)| {
                let mut addr = 0usize;
                addr.view_bits_mut::<O>()[..v.len()].clone_from_bitslice(v);
                self.filters[i].contains(addr) as usize
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use bitvec::prelude::*;

    use super::*;
    use crate::filter::PackedLUTFilterBuilder;

    fn simple_disc_test(
        input_size: usize,
        addr_size: usize,
        samples: Vec<BitVec>,
    ) -> Vec<usize> {
        let builder = PackedLUTFilterBuilder::new(addr_size, 4, 0);
        let mut disc =
            Discriminator::from_filter_builder(input_size, addr_size, &builder);
        let samples = samples
            .into_iter()
            .map(|v| Sample::from_raw_parts(v, addr_size, 0usize))
            .collect::<Vec<_>>();

        for sample in samples.iter() {
            disc.fit(sample);
        }

        samples.iter().map(|sample| disc.score(sample)).collect()
    }

    #[test]
    fn discriminator_1ram_4size() {
        let input_size = 4;
        let addr_size = 4;
        let samples = vec![
            bitvec![0, 0, 0, 0],
            bitvec![1, 0, 0, 0],
            bitvec![0, 1, 0, 0],
            bitvec![1, 1, 0, 0],
            bitvec![0, 0, 1, 0],
            bitvec![1, 0, 1, 0],
            bitvec![0, 1, 1, 0],
            bitvec![1, 1, 1, 0],
            bitvec![0, 0, 0, 1],
            bitvec![1, 0, 0, 1],
            bitvec![0, 1, 0, 1],
            bitvec![1, 1, 0, 1],
            bitvec![0, 0, 1, 1],
            bitvec![1, 0, 1, 1],
            bitvec![0, 1, 1, 1],
            bitvec![1, 1, 1, 1],
        ];
        let expected = vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1];
        let found = simple_disc_test(input_size, addr_size, samples);
        assert_eq!(expected, found);
    }

    #[test]
    fn discriminator_2ram_4size() {
        let input_size = 4;
        let addr_size = 2;
        let samples = vec![bitvec![0, 0, 0, 0], bitvec![1, 1, 1, 1]];
        let expected = vec![2, 2];
        let found = simple_disc_test(input_size, addr_size, samples);
        assert_eq!(expected, found);
    }

    #[test]
    fn discriminator_4ram_4size() {
        let input_size = 4;
        let addr_size = 1;
        let samples = vec![
            bitvec![1, 1, 0, 0],
            bitvec![0, 1, 1, 0],
            bitvec![0, 0, 1, 1],
            bitvec![1, 0, 0, 1],
        ];
        let expected = vec![4, 4, 4, 4];
        let found = simple_disc_test(input_size, addr_size, samples);
        assert_eq!(expected, found);
    }
}
