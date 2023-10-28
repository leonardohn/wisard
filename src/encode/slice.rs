use bitvec::{order::BitOrder, store::BitStore, vec::BitVec, view::BitView};
use serde::{de::DeserializeOwned, Serialize};

use crate::encode::SampleEncoder;
use crate::sample::{Label, Sample};

/// A logarithmic thermometer encoder.
#[derive(Debug)]
pub struct Slice {
    start: u8,
    end: u8,
}

impl Slice {
    pub fn new(start: u8, end: u8) -> Self {
        Self { start, end }
    }
}

impl<L, T, O> SampleEncoder<L, T, O> for Slice
where
    L: Label,
    T: BitStore + DeserializeOwned,
    T::Mem: Serialize,
    O: BitOrder,
{
    fn encode_inplace(&self, sample: &mut Sample<L, T, O>) {
        let start = self.start as usize;
        let end = self.end as usize;
        let size = end - start;
        let out_size = (sample.len() / sample.vsize()) * size;
        let mut bits = BitVec::<T, O>::with_capacity(out_size);

        for value in sample.iter_values() {
            let mut orig_value = 0usize;
            orig_value.view_bits_mut::<O>()[..value.len()]
                .clone_from_bitslice(value);

            let slice_value = &orig_value.view_bits::<O>()[start..end];
            bits.extend_from_bitslice(slice_value);
        }

        sample.set_raw_bits(bits);
        sample.set_vsize(end - start);
    }
}

#[cfg(test)]
mod tests {
    use bitvec::prelude::*;

    use super::*;

    #[test]
    fn slice_in2_out1() {
        let mut sample = Sample::from_raw_parts(
            bitvec![
                0, 0, //
                1, 0, //
                0, 1, //
                1, 1, //
            ],
            2,
            0usize,
        );
        let sample_slice = Sample::from_raw_parts(
            bitvec![
                0, //
                0, //
                1, //
                1, //
            ],
            1,
            0usize,
        );
        Slice::new(1, 2).encode_inplace(&mut sample);
        assert_eq!(sample, sample_slice);
    }

    #[test]
    fn slice_in3_out1() {
        let mut sample = Sample::from_raw_parts(
            bitvec![
                0, 0, 0, //
                0, 1, 0, //
                0, 0, 1, //
                0, 1, 1, //
            ],
            3,
            0usize,
        );
        let sample_slice = Sample::from_raw_parts(
            bitvec![
                0, //
                1, //
                0, //
                1, //
            ],
            1,
            0usize,
        );
        Slice::new(1, 2).encode_inplace(&mut sample);
        assert_eq!(sample, sample_slice);
    }
}
