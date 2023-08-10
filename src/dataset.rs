use bitvec::{order::BitOrder, store::BitStore};
use std::collections::HashSet;
use std::ops::{Index, IndexMut};

use crate::sample::Label;
use crate::sample::Sample;

pub type DatasetResult<T> = Result<T, DatasetError>;

#[non_exhaustive]
pub enum DatasetError {
    IO(std::io::Error),
}

#[derive(Clone, Debug)]
pub struct Dataset<L, S, O>
where
    L: Label,
    O: BitOrder,
    S: BitStore,
{
    samples: Vec<Sample<L, S, O>>,
}

impl<L, S, O> Dataset<L, S, O>
where
    L: Label,
    O: BitOrder,
    S: BitStore,
{
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_samples(samples: Vec<Sample<L, S, O>>) -> Self {
        Self { samples }
    }

    pub fn push(&mut self, sample: Sample<L, S, O>) {
        self.samples.push(sample);
    }

    pub fn len(&self) -> usize {
        self.samples.len()
    }

    pub fn is_empty(&self) -> bool {
        self.samples.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Sample<L, S, O>> {
        self.samples.as_slice().iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Sample<L, S, O>> {
        self.samples.as_mut_slice().iter_mut()
    }

    pub fn labels(&self) -> HashSet<L> {
        self.samples.iter().map(|s| s.label().clone()).collect()
    }
}

impl<L, S, O> Default for Dataset<L, S, O>
where
    L: Label,
    O: BitOrder,
    S: BitStore,
{
    fn default() -> Self {
        Self {
            samples: Vec::new(),
        }
    }
}

impl<L, S, O> Index<usize> for Dataset<L, S, O>
where
    L: Label,
    O: BitOrder,
    S: BitStore,
{
    type Output = Sample<L, S, O>;

    fn index(&self, index: usize) -> &Self::Output {
        self.samples.index(index)
    }
}

impl<L, S, O> IndexMut<usize> for Dataset<L, S, O>
where
    L: Label,
    O: BitOrder,
    S: BitStore,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.samples.index_mut(index)
    }
}
