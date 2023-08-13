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
pub struct Dataset<L, T, O>
where
    L: Label,
    T: BitStore,
    O: BitOrder,
{
    samples: Vec<Sample<L, T, O>>,
}

impl<L, T, O> Dataset<L, T, O>
where
    L: Label,
    T: BitStore,
    O: BitOrder,
{
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_samples(samples: Vec<Sample<L, T, O>>) -> Self {
        Self { samples }
    }

    pub fn push(&mut self, sample: Sample<L, T, O>) {
        self.samples.push(sample);
    }

    pub fn len(&self) -> usize {
        self.samples.len()
    }

    pub fn is_empty(&self) -> bool {
        self.samples.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Sample<L, T, O>> {
        self.samples.as_slice().iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Sample<L, T, O>> {
        self.samples.as_mut_slice().iter_mut()
    }

    pub fn labels(&self) -> HashSet<L> {
        self.samples.iter().map(|s| *s.label()).collect()
    }
}

impl<L, T, O> Default for Dataset<L, T, O>
where
    L: Label,
    T: BitStore,
    O: BitOrder,
{
    fn default() -> Self {
        Self {
            samples: Vec::new(),
        }
    }
}

impl<L, T, O> Index<usize> for Dataset<L, T, O>
where
    L: Label,
    T: BitStore,
    O: BitOrder,
{
    type Output = Sample<L, T, O>;

    fn index(&self, index: usize) -> &Self::Output {
        self.samples.index(index)
    }
}

impl<L, T, O> IndexMut<usize> for Dataset<L, T, O>
where
    L: Label,
    T: BitStore,
    O: BitOrder,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.samples.index_mut(index)
    }
}
