use std::hash::Hash;

mod bloom;
mod lut;

pub use self::bloom::*;
pub use lut::*;

/// A trait for basic set membership filters.
pub trait Filter {
    /// Includes an item as a member.
    fn include<T: Hash>(&mut self, item: &T) -> bool;
    /// Checks the membership of an item.
    fn contains<T: Hash>(&self, item: &T) -> bool;
}

/// A trait for set membership filters that uses counters.
pub trait CountingFilter: Filter {
    /// Returns the number of times a member was included.
    fn counter<T: Hash>(&self, item: &T) -> Option<usize>;
}

/// A trait for filter builders.
pub trait BuildFilter {
    /// The type of the associated filter.
    type Filter: Filter;
    /// Builds a new filter.
    fn build_filter(&self) -> Self::Filter;
}
