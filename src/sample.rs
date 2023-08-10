use std::{fmt::Debug, hash::Hash};

use bitvec::{
    order::{BitOrder, LocalBits},
    ptr::{BitRef, Const},
    slice::BitSlice,
    store::BitStore,
    vec::BitVec,
};

/// A trait for the sample labels.
pub trait Label: Copy + Clone + Debug + Eq + PartialEq + Hash {}

impl<T: Copy + Clone + Debug + Eq + PartialEq + Hash> Label for T {}

/// Represents a labeled sample.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Sample<L, S = usize, O = LocalBits>
where
    L: Label,
    O: BitOrder,
    S: BitStore,
{
    bits: BitVec<S, O>,
    vsize: usize,
    label: L,
}

impl<L, S, O> Sample<L, S, O>
where
    L: Label,
    O: BitOrder,
    S: BitStore,
{
    /// Creates a [`Sample`](./struct.Sample.html) instance from its raw parts.
    pub fn from_raw_parts(bits: BitVec<S, O>, vsize: usize, label: L) -> Self {
        assert!(
            bits.len() >= vsize,
            "There are not enough bits for `vsize` (bits: {}, vsize: {})",
            bits.len(),
            vsize,
        );
        assert!(
            bits.len() % vsize == 0,
            "The bits are not divisible by `vsize` (bits: {}, vsize: {})",
            bits.len(),
            vsize,
        );
        Self { bits, vsize, label }
    }

    /// Breaks a [`Sample`](./struct.Sample.html) instance into its raw parts.
    pub fn into_raw_parts(self) -> (BitVec<S, O>, usize, L) {
        let Self { bits, vsize, label } = self;
        (bits, vsize, label)
    }

    /// Returns an iterator over the individual sample bits
    pub fn iter_bits(&self) -> impl Iterator<Item = BitRef<'_, Const, S, O>> {
        self.bits.iter()
    }

    /// Returns an iterator over the sample bit chunks.
    pub fn iter_values(&self) -> impl Iterator<Item = &BitSlice<S, O>> {
        self.bits.chunks(self.vsize)
    }

    /// Returns the number of bits of a sample.
    pub fn len(&self) -> usize {
        self.bits.len()
    }

    /// Returns `true` if the sample has no bits.
    pub fn is_empty(&self) -> bool {
        self.bits.is_empty()
    }

    /// Returns a slice over the raw sample bits.
    pub fn raw_bits(&self) -> &BitSlice<S, O> {
        &self.bits
    }

    /// Returns a mutable slice over the raw sample bits.
    pub fn raw_bits_mut(&mut self) -> &mut BitSlice<S, O> {
        &mut self.bits
    }

    /// Replaces the raw sample bits with a given `BitVec`.
    pub fn set_raw_bits(&mut self, bits: BitVec<S, O>) {
        self.bits = bits;
    }

    /// Returns the value size (number of bits for each element in the sample).
    pub fn vsize(&self) -> usize {
        self.vsize
    }

    /// Updates the value size (number of bits for each element in the sample).
    pub fn set_vsize(&mut self, vsize: usize) {
        assert!(
            self.len() >= vsize,
            "There are not enough bits for `vsize` (bits: {}, vsize: {})",
            self.len(),
            vsize,
        );
        assert!(
            self.len() % vsize == 0,
            "The bits are not divisible by `vsize` (bits: {}, vsize: {})",
            self.len(),
            vsize,
        );
        self.vsize = vsize;
    }

    /// Returns the associated label.
    pub fn label(&self) -> &L {
        &self.label
    }

    /// Sets the associated label.
    pub fn set_label(&mut self, label: L) {
        self.label = label;
    }
}

#[cfg(test)]
mod tests {
    use bitvec::{bitvec, field::BitField, order::Lsb0};

    use super::*;

    #[test]
    fn from_into_parts() {
        let bits = bitvec![0, 1];
        let vsize = 2usize;
        let label = 0usize;
        let sample = Sample::from_raw_parts(bits, vsize, label);
        let bits = bitvec![0, 1];
        let parts = sample.into_raw_parts();
        assert_eq!(parts.0, bits);
        assert_eq!(parts.1, vsize);
        assert_eq!(parts.2, label);
    }

    #[test]
    fn into_bits() {
        let sample = Sample {
            bits: bitvec![0, 1, 0, 1, 0, 1],
            vsize: 3,
            label: 0,
        };
        let bits = sample.iter_bits().map(|b| *b).collect::<Vec<bool>>();
        assert_eq!(bits, vec![false, true, false, true, false, true]);
    }

    #[test]
    fn into_values() {
        let sample = Sample {
            bits: bitvec![0, 1, 0, 1, 0, 1],
            vsize: 3,
            label: 0,
        };
        let bits = sample.iter_values().map(|b| b.load()).collect::<Vec<u8>>();
        assert_eq!(bits, vec![0b010, 0b101]);
    }
}
