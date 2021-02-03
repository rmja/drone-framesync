#![feature(const_fn)]
#![feature(const_impl_trait)]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
#[macro_use]
extern crate std;

pub mod comparators;
pub mod detectors;
