use bitvec::prelude::*;
use serde::{de::DeserializeOwned, Serialize};

use crate::sample::{Label, Sample};

mod permute;
mod slice;
mod therm;

pub use permute::*;
pub use slice::*;
pub use therm::*;

/// A trait for sample encoders, i.e. transformations over the sample bits.
pub trait SampleEncoder<L, T, O>
where
    L: Label,
    T: BitStore + DeserializeOwned,
    T::Mem: Serialize,
    O: BitOrder,
{
    /// Encodes the sample in-place.
    fn encode_inplace(&self, sample: &mut Sample<L, T, O>);

    /// Consumes the sample and return its encoded version.
    fn encode(&self, mut sample: Sample<L, T, O>) -> Sample<L, T, O> {
        self.encode_inplace(&mut sample);
        sample
    }
}
