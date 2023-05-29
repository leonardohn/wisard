use std::{
    fmt::Debug,
    hash::{BuildHasher, Hash},
};

use bloom::{CountingBloomFilter, ASMS};

use crate::filter::{BuildFilter, CountingFilter, Filter};

/// A Filter structure based on Bloom filters.
pub struct BloomFilter<R, S>
where
    R: BuildHasher,
    S: BuildHasher,
{
    threshold: usize,
    bloom: CountingBloomFilter<R, S>,
}

impl<R, S> BloomFilter<R, S>
where
    R: BuildHasher,
    S: BuildHasher,
{
    /// Returns a new [`BloomFilter`](./struct.BloomFilter.html)
    /// instance.
    ///
    /// The `addr_size` parameter represents the size of the address in the
    /// lookup table, signifying the number of bits in the input. The
    /// `count_size` parameter determines the number of bits assigned to each
    /// counter. The `threshold` value specifies the minimum number of similar
    /// items required for them to be recognized as members by the filter.
    /// The `rate` parameter determines the expected rate of false positives
    /// for the Bloom filters. Both `hasher_one` and `hasher_two` must be
    /// distinct hash functions.
    pub fn with_rate_and_hashers(
        addr_size: usize,
        count_size: usize,
        threshold: usize,
        rate: f32,
        hasher_one: R,
        hasher_two: S,
    ) -> Self {
        let bloom = CountingBloomFilter::with_rate_and_hashers(
            count_size,
            rate,
            1 << addr_size,
            hasher_one,
            hasher_two,
        );
        Self { threshold, bloom }
    }
}

impl<R, S> Filter for BloomFilter<R, S>
where
    R: BuildHasher,
    S: BuildHasher,
{
    fn include<T: Hash>(&mut self, item: &T) -> bool {
        self.bloom.insert(item)
    }

    fn contains<T: Hash>(&self, item: &T) -> bool {
        self.bloom.estimate_count(item) as usize > self.threshold
    }
}

impl<R, S> CountingFilter for BloomFilter<R, S>
where
    R: BuildHasher,
    S: BuildHasher,
{
    fn counter<H: Hash>(&self, item: &H) -> Option<usize> {
        Some(self.bloom.estimate_count(&item) as usize)
    }
}

/// A builder for [`BloomFilter`](./struct.BloomFilter.html).
#[derive(Copy, Clone, Debug)]
pub struct BloomFilterBuilder<R, S>
where
    R: BuildHasher + Clone,
    S: BuildHasher + Clone,
{
    addr_size: usize,
    count_size: usize,
    threshold: usize,
    rate: f32,
    hasher_one: R,
    hasher_two: S,
}

impl<R, S> BloomFilterBuilder<R, S>
where
    R: BuildHasher + Clone,
    S: BuildHasher + Clone,
{
    pub fn with_rate_and_hashers(
        addr_size: usize,
        count_size: usize,
        threshold: usize,
        rate: f32,
        hasher_one: R,
        hasher_two: S,
    ) -> Self {
        Self {
            addr_size,
            count_size,
            threshold,
            rate,
            hasher_one,
            hasher_two,
        }
    }
}

impl<R, S> BuildFilter for BloomFilterBuilder<R, S>
where
    R: BuildHasher + Clone,
    S: BuildHasher + Clone,
{
    type Filter = BloomFilter<R, S>;
    fn build_filter(&self) -> Self::Filter {
        Self::Filter::with_rate_and_hashers(
            self.addr_size,
            self.count_size,
            self.threshold,
            self.rate,
            self.hasher_one.clone(),
            self.hasher_two.clone(),
        )
    }
}

#[cfg(test)]
mod tests {
    use std::collections::hash_map::RandomState;

    use super::*;

    #[test]
    fn bloom_filter_single() {
        let value = 0usize;
        let hasher_one = RandomState::new();
        let hasher_two = RandomState::new();
        let builder = BloomFilterBuilder::with_rate_and_hashers(
            1, 2, 1, 0.01, hasher_one, hasher_two,
        );
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
