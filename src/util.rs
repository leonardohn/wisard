use std::fmt::Debug;
use std::hash::{Hash, Hasher};

use num_traits::{Saturating, Unsigned};

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

/// A hasher that only accepts integers and use their raw values as indices.
#[derive(Copy, Clone, Debug, Default)]
pub struct RawIntHasher(Option<u64>);

impl Hasher for RawIntHasher {
    fn finish(&self) -> u64 {
        self.0.unwrap_or_else(|| {
            panic!("RawIntHasher have not hashed any values")
        })
    }

    fn write(&mut self, _: &[u8]) {
        panic!("RawIntHasher can only hash integers");
    }

    fn write_u64(&mut self, i: u64) {
        self.0.map_or_else(
            || self.0 = Some(i),
            |_| panic!("RawIntHasher can only hash once"),
        )
    }

    fn write_u32(&mut self, i: u32) {
        self.write_u64(i as u64)
    }

    fn write_u16(&mut self, i: u16) {
        self.write_u64(i as u64)
    }

    fn write_u8(&mut self, i: u8) {
        self.write_u64(i as u64)
    }

    fn write_usize(&mut self, i: usize) {
        self.write_u64(i as u64)
    }
}
