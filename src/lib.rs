#![feature(const_fn)]
#![feature(const_impl_trait)]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
#[macro_use]
extern crate std;

extern crate alloc;

pub mod comparators;
pub mod detectors;
mod syncwindow;
mod sliceext;

pub use self::syncwindow::SyncWindow;
