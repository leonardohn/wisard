# `wisard` - A library for WiSARD nets in Rust

![License](https://img.shields.io/github/license/leonardohn/wisard?style=for-the-badge)

## Summary

WiSARD (Wilkie, Stonham, Aleksander Recognition Device) is an alternative,
weightless type of neural network known for its high-speed pattern recognition
capabilities and simplicity. This project is a Rust implementation of WiSARD
nets, providing a lightweight, efficient, and user-friendly library for
building, training, and evaluating models.

This implementation aims to harness the language's performance, safety, and 
concurrency features, to make it ideal for both research and production-grade
applications. We aim to provide a flexible and extensible foundation for 
developing pattern recognition systems that can be deployed in various domains
such as image recognition and signal processing.

Please note that this is a work in progress and it is not yet ready to be used
in production environments.

## Features

- Fast and efficient implementation of WiSARD nets.
- Relevant sample encoding functions for data preprocessing.
- Customizable build-time and run-time parameters for controlling the model
  behavior.
- Support for various data types, including binary, categorical, and
  continuous inputs.

## Installation

To use WiSARD in your Rust project, add the following line to your `Cargo.toml`
file:

```toml
[dependencies]
wisard = "0.0.1"
```

For additional installation options and guidance, please refer to the 
[crate documentation](https://docs.rs/wisard).

## Usage

Here's a simple example demonstrating how to create and train a basic WiSARD 
model using the `wisard` crate:

```rust
use std::collections::HashSet;

use bitvec::prelude::*;
use wisard::model::BinaryWisard;

fn main() {
    // The size of the input (in bits)
    let input_size = 8;
    
    // The size of the address (in bits)
    let addr_size = 2;
    
    // The set of labels used by the samples
    let labels = HashSet::from_iter(vec!["cold", "hot"].into_iter());

    // Create a new WiSARD model
    let mut model = BinaryWisard::new(input_size, addr_size, labels);

    // Provide some sample data
    let samples = vec![
        (bitvec![1, 1, 1, 0, 0, 0, 0, 0], "cold"),
        (bitvec![1, 1, 1, 1, 0, 0, 0, 0], "cold"),
        (bitvec![0, 0, 0, 0, 1, 1, 1, 1], "hot"),
        (bitvec![0, 0, 0, 0, 0, 1, 1, 1], "hot"),
    ];

    // Instantiate the samples
    let samples = samples
        .into_iter()
        .map(|(v, l)| Sample::from_raw_parts(v, addr_size, l))
        .collect::<Vec<_>>();

    // Train the model using each provided sample
    for sample in encoded_samples.iter() {
        model.fit(sample);
    }

    // Display the model predictions
    for sample in encoded_samples.iter() {
        let input = sample.raw_bits();
        let true = sample.label();
        let pred = model.predict(sample);
        println!("Input: {input:?}, True: {true:?}, Pred: {pred:?}");
    }
}
```

For more detailed examples and usage instructions, please consult the
[API documentation](https://docs.rs/wisard).

## Contribution

Contributions to the `wisard` project are welcome! If you find a bug or have
suggestions for improvements, please open an
[Issue](https://github.com/leonardohn/wisard/issues). 
[Pull requests](https://github.com/leonardohn/wisard/pulls) for new features,
bug fixes, and documentation enhancements are also appreciated.

## License

`wisard` is distributed under the terms of both the MIT license and the
Apache License (Version 2.0). See [LICENSE-APACHE](LICENSE-APACHE) and
[LICENSE-MIT](LICENSE-MIT) for details. Opening a pull request is
assumed to signal agreement with these licensing terms.
