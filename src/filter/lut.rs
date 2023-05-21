use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
};

use bitvec::{bitvec, order::Lsb0, vec::BitVec, view::BitView};

use num_traits::{Saturating, Unsigned};

use crate::{
    filter::{CountingFilter, Filter},
    utils::PrimitiveHasher,
};

/// A trait for primitive unsigned integers to be used as saturating counters.
pub trait Counter:
    Copy
    + Clone
    + Debug
    + Default
    + Eq
    + PartialEq
    + Ord
    + PartialOrd
    + Hash
    + Unsigned
    + Saturating
    + Into<usize>
{
}

impl<T> Counter for T where
    T: Copy
        + Clone
        + Debug
        + Default
        + Eq
        + PartialEq
        + Ord
        + PartialOrd
        + Hash
        + Unsigned
        + Saturating
        + Into<usize>
{
}

/// A Filter structure based on dense, integer-aligned lookup tables (LUTs).
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct LUTFilter<T: Counter = u8> {
    addr_size: usize,
    threshold: T,
    lut: Vec<T>,
}

impl<T: Counter> LUTFilter<T> {
    /// Creates a new [`LUTFilter`](./struct.LUTFilter.html) instance.
    ///
    /// The `addr_size` parameter represents the address size of the lookup
    /// table, indicating the number of bits in the filter input. The
    /// `threshold` value specifies the minimum number of similar items
    /// required for them to be recognized as members by the filter.
    pub fn new(addr_size: usize, threshold: T) -> Self {
        let lut = vec![T::zero(); 1 << addr_size];
        Self {
            addr_size,
            threshold,
            lut,
        }
    }
}

impl<T: Counter> Filter for LUTFilter<T> {
    fn include<H: Hash>(&mut self, item: H) -> bool {
        let mut hasher = PrimitiveHasher::default();
        item.hash(&mut hasher);
        let index = hasher.finish() as usize;
        self.lut
            .get_mut(index)
            .map(|count| {
                *count = count.saturating_add(T::one());
            })
            .is_some()
    }

    fn contains<H: Hash>(&self, item: H) -> bool {
        let mut hasher = PrimitiveHasher::default();
        item.hash(&mut hasher);
        let index = hasher.finish() as usize;
        self.lut
            .get(index)
            .map(|count| *count > self.threshold)
            .unwrap_or(false)
    }
}

impl<T: Counter> CountingFilter for LUTFilter<T> {
    fn counter<H: Hash>(&self, item: H) -> Option<usize> {
        let mut hasher = PrimitiveHasher::default();
        item.hash(&mut hasher);
        let index = hasher.finish() as usize;
        self.lut.get(index).map(|v| (*v).into())
    }
}

/// A Filter structure based on dense, bit-packed lookup tables (LUTs).
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct PackedLUTFilter {
    addr_size: usize,
    count_size: usize,
    threshold: usize,
    lut: BitVec,
}

impl PackedLUTFilter {
    /// Returns a new [`PackedLUTFilter`](./struct.PackedLUTFilter.html)
    /// instance.
    ///
    /// The `addr_size` parameter represents the size of the address in the
    /// lookup table, signifying the number of bits in the input. The
    /// `count_size` parameter determines the number of bits assigned to each
    /// counter. The threshold value specifies the minimum number of similar
    /// items required for them to be recognized as members by the filter.
    pub fn new(addr_size: usize, count_size: usize, threshold: usize) -> Self {
        Self {
            addr_size,
            count_size,
            threshold,
            lut: bitvec![usize, Lsb0; 0; count_size << addr_size],
        }
    }
}

impl Filter for PackedLUTFilter {
    fn include<H: Hash>(&mut self, item: H) -> bool {
        let mut hasher = PrimitiveHasher::default();
        item.hash(&mut hasher);
        let max_value = (1 << self.count_size) - 1;
        let index = self.count_size * hasher.finish() as usize;
        self.lut
            .get_mut(index..index + self.count_size)
            .map(|count| {
                let mut value = 0usize;
                value.view_bits_mut::<Lsb0>()[..self.count_size]
                    .clone_from_bitslice(count);
                value = max_value.min(value + 1);
                count.clone_from_bitslice(
                    &value.view_bits::<Lsb0>()[..self.count_size],
                );
            })
            .is_some()
    }

    fn contains<H: Hash>(&self, item: H) -> bool {
        self.counter(item)
            .map(|count| count > self.threshold)
            .unwrap_or(false)
    }
}

impl CountingFilter for PackedLUTFilter {
    fn counter<H: Hash>(&self, item: H) -> Option<usize> {
        let mut hasher = PrimitiveHasher::default();
        item.hash(&mut hasher);
        let index = self.count_size * hasher.finish() as usize;
        self.lut.get(index..index + self.count_size).map(|count| {
            let mut value = 0usize;
            value.view_bits_mut::<Lsb0>()[..self.count_size]
                .clone_from_bitslice(count);
            value
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lut_filter_single() {
        let value = 0usize;
        let mut filter = LUTFilter::new(0, 1u8);
        assert_eq!(filter.counter(value), Some(0));
        assert!(!filter.contains(value));
        filter.include(value);
        assert_eq!(filter.counter(value), Some(1));
        assert!(!filter.contains(value));
        filter.include(value);
        assert_eq!(filter.counter(value), Some(2));
        assert!(filter.contains(value));
    }

    #[test]
    fn packed_lut_filter_single() {
        let value = 0usize;
        let mut filter = PackedLUTFilter::new(0, 2, 1);
        assert_eq!(filter.counter(value), Some(0));
        assert!(!filter.contains(value));
        filter.include(value);
        assert_eq!(filter.counter(value), Some(1));
        assert!(!filter.contains(value));
        filter.include(value);
        assert_eq!(filter.counter(value), Some(2));
        assert!(filter.contains(value));
    }
}
