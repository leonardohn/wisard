use bitvec::{
    field::BitField, order::BitOrder, slice::BitSlice, store::BitStore,
    vec::BitVec, view::BitView,
};

use crate::encode::SampleEncoder;
use crate::sample::{Label, Sample};

/// A logarithmic thermometer encoder.
#[derive(Debug)]
pub struct LogThermometer {
    /// The resolution (output size), in bits.
    resolution: u8,
}

impl LogThermometer {
    /// Creates a new [`LogThermometer`](./struct.LogThermometer.html) instance
    /// with a resolution (output size) of `resolution` bits. The `resolution`
    /// must be a power of two.
    pub fn with_resolution(resolution: u8) -> Self {
        assert!(
            resolution.is_power_of_two(),
            "LogThermometer only supports resolutions that are powers of two",
        );
        Self { resolution }
    }
}

impl<L, S, O> SampleEncoder<L, S, O> for LogThermometer
where
    L: Label,
    O: BitOrder,
    S: BitStore,
{
    fn encode_inplace(&self, sample: &mut Sample<L, S, O>) {
        let max_bits = std::mem::size_of::<usize>() << 3;

        if sample.size() > max_bits {
            panic!(
                "LogThermometer can only encode values up to {} bits",
                max_bits,
            );
        }

        if !sample.size().is_power_of_two() {
            panic!("Sample size must be a power of two");
        }

        let resolution = self.resolution as usize;
        let out_size = (sample.len() / sample.size()) * resolution;
        let mut bits = BitVec::<S, O>::with_capacity(out_size);

        for value in sample.iter_values() {
            let mut orig_value = 0usize;
            orig_value.view_bits_mut::<O>()[..value.len()]
                .clone_from_bitslice(value);
            orig_value = (orig_value + 1).next_power_of_two().ilog2() as usize;

            if sample.size() < resolution {
                orig_value *= resolution / sample.size();
            } else {
                orig_value /= sample.size() / resolution;
            };

            let therm_value = (1usize << orig_value) - 1;
            let therm_value = &therm_value.view_bits::<O>()[..resolution];
            bits.extend_from_bitslice(therm_value);
        }

        sample.set_raw_bits(bits);
        sample.set_size(resolution);
    }
}

/// A linear thermometer encoder.
#[derive(Debug)]
pub struct LinearThermometer {
    /// The resolution (output size), in bits.
    resolution: u8,
}

impl LinearThermometer {
    /// Creates a new [`LinearThermometer`](./struct.LinearThermometer.html)
    /// instance with a resolution (output size) of `resolution` bits.
    pub fn with_resolution(resolution: u8) -> Self {
        Self { resolution }
    }
}

impl<L, S, O> SampleEncoder<L, S, O> for LinearThermometer
where
    L: Label,
    O: BitOrder,
    S: BitStore,
    BitSlice<S, O>: BitField,
{
    fn encode_inplace(&self, sample: &mut Sample<L, S, O>) {
        let max_bits = std::mem::size_of::<usize>() << 3;

        if sample.size() > max_bits {
            panic!(
                "LinearThermometer can only encode values up to {} bits",
                max_bits,
            );
        }

        let resolution = self.resolution as usize;
        let out_size = (sample.len() / sample.size()) * resolution;
        let mut bits = BitVec::<S, O>::with_capacity(out_size);

        for value in sample.iter_values() {
            let mut bit_value = 0usize;
            bit_value.view_bits_mut::<O>()[..value.len()]
                .clone_from_bitslice(value);
            let quant_value = ((resolution + 1) * bit_value
                + (value.len() >> 1))
                >> value.len();
            let therm_value = (1usize << quant_value) - 1;
            let therm_value = &therm_value.view_bits::<O>()[..resolution];
            bits.extend_from_bitslice(therm_value);
        }

        sample.set_raw_bits(bits);
        sample.set_size(resolution);
    }
}

#[cfg(test)]
mod tests {
    use bitvec::{bitvec, order::Lsb0};

    use super::*;
    use crate::encode::Permute;

    #[test]
    fn shuffle_fixed_seed() {
        let sample_1 =
            Sample::from_raw_parts(bitvec![0, 0, 0, 0, 1, 1, 1, 1], 1, 0usize);
        let sample_2 =
            Sample::from_raw_parts(bitvec![0, 1, 0, 1, 0, 1, 0, 1], 1, 0usize);
        let sample_1_shuf =
            Sample::from_raw_parts(bitvec![1, 0, 0, 1, 0, 0, 1, 1], 1, 0usize);
        let sample_2_shuf =
            Sample::from_raw_parts(bitvec![0, 1, 0, 1, 1, 0, 0, 1], 1, 0usize);
        let shuffle = <Permute>::with_seed(7);
        assert_eq!(shuffle.encode(sample_1), sample_1_shuf);
        assert_eq!(shuffle.encode(sample_2), sample_2_shuf);
    }

    #[test]
    fn log_therm_in2_out1() {
        let mut sample = Sample::from_raw_parts(
            bitvec![
                0, 0, //
                1, 0, //
                0, 1, //
                1, 1, //
            ],
            2,
            0usize,
        );
        let sample_therm = Sample::from_raw_parts(
            bitvec![
                0, //
                0, //
                1, //
                1, //
            ],
            1,
            0usize,
        );
        LogThermometer::with_resolution(1).encode_inplace(&mut sample);
        assert_eq!(sample, sample_therm);
    }

    #[test]
    fn log_therm_in2_out2() {
        let mut sample = Sample::from_raw_parts(
            bitvec![
                0, 0, //
                1, 0, //
                0, 1, //
                1, 1, //
            ],
            2,
            0usize,
        );
        let sample_therm = Sample::from_raw_parts(
            bitvec![
                0, 0, //
                1, 0, //
                1, 1, //
                1, 1, //
            ],
            2,
            0usize,
        );
        LogThermometer::with_resolution(2).encode_inplace(&mut sample);
        assert_eq!(sample, sample_therm);
    }

    #[test]
    fn log_therm_in2_out4() {
        let mut sample = Sample::from_raw_parts(
            bitvec![
                0, 0, //
                1, 0, //
                0, 1, //
                1, 1, //
            ],
            2,
            0usize,
        );
        let sample_therm = Sample::from_raw_parts(
            bitvec![
                0, 0, 0, 0, //
                1, 1, 0, 0, //
                1, 1, 1, 1, //
                1, 1, 1, 1, //
            ],
            4,
            0usize,
        );
        LogThermometer::with_resolution(4).encode_inplace(&mut sample);
        assert_eq!(sample, sample_therm);
    }

    #[test]
    fn linear_therm_in2_out1() {
        let mut sample = Sample::from_raw_parts(
            bitvec![
                0, 0, //
                1, 0, //
                0, 1, //
                1, 1, //
            ],
            2,
            0usize,
        );
        let sample_therm = Sample::from_raw_parts(
            bitvec![
                0, //
                0, //
                1, //
                1, //
            ],
            1,
            0usize,
        );
        LinearThermometer::with_resolution(1).encode_inplace(&mut sample);
        assert_eq!(sample, sample_therm);
    }

    #[test]
    fn linear_therm_in2_out2() {
        let mut sample = Sample::from_raw_parts(
            bitvec![
                0, 0, //
                1, 0, //
                0, 1, //
                1, 1, //
            ],
            2,
            0usize,
        );
        let sample_therm = Sample::from_raw_parts(
            bitvec![
                0, 0, //
                1, 0, //
                1, 0, //
                1, 1, //
            ],
            2,
            0usize,
        );
        LinearThermometer::with_resolution(2).encode_inplace(&mut sample);
        assert_eq!(sample, sample_therm);
    }

    #[test]
    fn linear_therm_in2_out3() {
        let mut sample = Sample::from_raw_parts(
            bitvec![
                0, 0, //
                1, 0, //
                0, 1, //
                1, 1, //
            ],
            2,
            0usize,
        );
        let sample_therm = Sample::from_raw_parts(
            bitvec![
                0, 0, 0, //
                1, 0, 0, //
                1, 1, 0, //
                1, 1, 1, //
            ],
            3,
            0usize,
        );
        LinearThermometer::with_resolution(3).encode_inplace(&mut sample);
        assert_eq!(sample, sample_therm);
    }

    #[test]
    fn linear_therm_in2_out4() {
        let mut sample = Sample::from_raw_parts(
            bitvec![
                0, 0, //
                1, 0, //
                0, 1, //
                1, 1, //
            ],
            2,
            0usize,
        );
        let sample_therm = Sample::from_raw_parts(
            bitvec![
                0, 0, 0, 0, //
                1, 0, 0, 0, //
                1, 1, 0, 0, //
                1, 1, 1, 1, //
            ],
            4,
            0usize,
        );
        LinearThermometer::with_resolution(4).encode_inplace(&mut sample);
        assert_eq!(sample, sample_therm);
    }
}
