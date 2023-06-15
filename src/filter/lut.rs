use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
};

use bitvec::{bitvec, order::Lsb0, vec::BitVec, view::BitView};

use num_traits::{Saturating, Unsigned};

use crate::{
    filter::{BuildFilter, CountingFilter, Filter},
    util::RawIntHasher,
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
pub struct LUTFilter<C: Counter = u8> {
    addr_size: usize,
    threshold: C,
    lut: Vec<C>,
}

impl<C: Counter> LUTFilter<C> {
    /// Creates a new [`LUTFilter`](./struct.LUTFilter.html) instance.
    ///
    /// The `addr_size` parameter represents the address size of the lookup
    /// table, indicating the number of bits in the filter input. The
    /// `threshold` value specifies the minimum number of similar items
    /// required for them to be recognized as members by the filter.
    pub fn new(addr_size: usize, threshold: C) -> Self {
        let lut = vec![C::zero(); 1 << addr_size];
        Self {
            addr_size,
            threshold,
            lut,
        }
    }
}

impl<C: Counter> Filter for LUTFilter<C> {
    fn include<T: Hash>(&mut self, item: &T) -> bool {
        let mut hasher = RawIntHasher::default();
        item.hash(&mut hasher);
        let index = hasher.finish() as usize;
        self.lut
            .get_mut(index)
            .map(|count| {
                *count = count.saturating_add(C::one());
            })
            .is_some()
    }

    fn contains<T: Hash>(&self, item: &T) -> bool {
        let mut hasher = RawIntHasher::default();
        item.hash(&mut hasher);
        let index = hasher.finish() as usize;
        self.lut
            .get(index)
            .map(|count| *count > self.threshold)
            .unwrap_or(false)
    }
}

impl<C: Counter> CountingFilter for LUTFilter<C> {
    fn counter<T: Hash>(&self, item: &T) -> Option<usize> {
        let mut hasher = RawIntHasher::default();
        item.hash(&mut hasher);
        let index = hasher.finish() as usize;
        self.lut.get(index).map(|v| (*v).into())
    }
}

/// A builder for [`LUTFilter`](./struct.LUTFilter.html).
#[derive(Copy, Clone, Debug)]
pub struct LUTFilterBuilder<C: Counter = u8> {
    addr_size: usize,
    threshold: C,
}

impl<C: Counter> LUTFilterBuilder<C> {
    pub fn new(addr_size: usize, threshold: C) -> Self {
        Self {
            addr_size,
            threshold,
        }
    }
}

impl<C: Counter> BuildFilter for LUTFilterBuilder<C> {
    type Filter = LUTFilter<C>;
    fn build_filter(&self) -> Self::Filter {
        Self::Filter::new(self.addr_size, self.threshold)
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
    fn include<T: Hash>(&mut self, item: &T) -> bool {
        let mut hasher = RawIntHasher::default();
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

    fn contains<T: Hash>(&self, item: &T) -> bool {
        self.counter(item)
            .map(|count| count > self.threshold)
            .unwrap_or(false)
    }
}

impl CountingFilter for PackedLUTFilter {
    fn counter<T: Hash>(&self, item: &T) -> Option<usize> {
        let mut hasher = RawIntHasher::default();
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

/// A builder for [`PackedLUTFilter`](./struct.PackedLUTFilter.html).
#[derive(Copy, Clone, Debug)]
pub struct PackedLUTFilterBuilder {
    addr_size: usize,
    count_size: usize,
    threshold: usize,
}

impl PackedLUTFilterBuilder {
    pub fn new(addr_size: usize, count_size: usize, threshold: usize) -> Self {
        Self {
            addr_size,
            count_size,
            threshold,
        }
    }
}

impl BuildFilter for PackedLUTFilterBuilder {
    type Filter = PackedLUTFilter;
    fn build_filter(&self) -> Self::Filter {
        Self::Filter::new(self.addr_size, self.count_size, self.threshold)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lut_filter_single() {
        let value = 0usize;
        let builder = LUTFilterBuilder::new(0, 1u8);
        let mut filter = builder.build_filter();
        assert_eq!(filter.counter(&value), Some(0));
        assert!(!filter.contains(&value));
        filter.include(&value);
        assert_eq!(filter.counter(&value), Some(1));
        assert!(!filter.contains(&value));
        filter.include(&value);
        assert_eq!(filter.counter(&value), Some(2));
        assert!(filter.contains(&value));
    }

    #[test]
    fn packed_lut_filter_single() {
        let value = 0usize;
        let builder = PackedLUTFilterBuilder::new(0, 2, 1);
        let mut filter = builder.build_filter();
        assert_eq!(filter.counter(&value), Some(0));
        assert!(!filter.contains(&value));
        filter.include(&value);
        assert_eq!(filter.counter(&value), Some(1));
        assert!(!filter.contains(&value));
        filter.include(&value);
        assert_eq!(filter.counter(&value), Some(2));
        assert!(filter.contains(&value));
    }
}
