use std::{collections::HashSet, path::Path, slice::{Iter, IterMut}};

use bitvec::{order::BitOrder, store::BitStore};

use crate::sample::Label;

#[derive(Clone, Debug, Default)]
pub struct Dataset<L, S, O>
where
    L: Label,
    O: BitOrder,
    S: BitStore,
{
    labels: HashSet<L>,
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

    pub fn push(&mut self, sample: Sample<L, S, O>) {
        self.labels.insert(sample.label().clone());
        self.samples.push(sample);
    }

    pub fn iter(&self) -> Iter<'_, Sample<L, S, O>> {
        self.samples.as_slice().iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, Sample<L, S, O>> {
        self.samples.as_mut_slice().iter_mut()
    }
}
