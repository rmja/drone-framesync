![maintenance](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)

# drone-framesync

Fast frame synchronization algorithms for finding a position of a syncword within a bitstream with some allowed error tolerance.

The crate contains several algorithms that perform differently on different architectures.
There is a [benchmark project](benchmark) that tests all the algorithms and lists the number of cycles they use.

For Cortex-M4, the fastest algorithms have been determined and are selected based on the allowed threshold.

```rust

use drone_framesync::detectors::{cortexm4, Detector};

// Get the fastest detector for the 16 bit syncword 0xFFFF with an allowed tolerance of 2 bits.
let detector = cortexm4::sync16_tol2::<0xFFFF>();
let haystack = [0u8; 32];
let position = detector.position(&haystack);

// Get the fastest detector for the 32 bit syncword 0xFFFFFFFF with an allowed tolerance of 2 bits.
let detector = cortexm4::sync32_tol2::<0xFFFFFFFF>();
let haystack = [0u8; 32];
let position = detector.position(&haystack);

```

## Usage

Add the crate to your `Cargo.toml` dependencies:

```toml
[dependencies]
drone-framesync = { git = "https://github.com/rmja/drone-framesync" }
```

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
