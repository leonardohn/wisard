use bitvec::{order::BitOrder, store::BitStore};

use crate::sample::{Label, Sample};

mod permute;
mod therm;

pub use permute::*;
pub use therm::*;

/// A trait for sample encoders, i.e. transformations over the sample bits.
pub trait SampleEncoder<L, S, O>
where
    L: Label,
    O: BitOrder,
    S: BitStore,
{
    /// Encodes the sample in-place.
    fn encode_inplace(&self, sample: &mut Sample<L, S, O>);

    /// Consumes the sample and return its encoded version.
    fn encode(&self, mut sample: Sample<L, S, O>) -> Sample<L, S, O> {
        self.encode_inplace(&mut sample);
        sample
    }
}
