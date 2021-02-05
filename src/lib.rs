#![feature(const_fn)]
#![feature(const_impl_trait)]
// #![feature(const_generics)]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
#[macro_use]
extern crate std;

extern crate alloc;

pub mod comparators;
pub mod detectors;
mod bitstream;
mod sliceext;

pub use self::bitstream::BitStream;
