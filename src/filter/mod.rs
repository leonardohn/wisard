use std::hash::Hash;

mod lut;

pub use lut::*;

/// A trait for basic set membership filters.
pub trait Filter {
    /// Includes an item as a member.
    fn include<T: Hash>(&mut self, item: T) -> bool;
    /// Checks the membership of an item.
    fn contains<T: Hash>(&self, item: T) -> bool;
}

/// A trait for set membership filters that uses counters.
pub trait CountingFilter: Filter {
    /// Returns the number of times a member was included.
    fn counter<T: Hash>(&self, item: T) -> Option<usize>;
}
