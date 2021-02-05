pub mod cortexm4;
mod double16;
mod double32;
mod single16;
mod single32;

pub trait Detector<T> {
    type Block: Copy + Sized;

    const SYNCWORD: T;

    fn from_slice(slice: &[u8]) -> Self::Block;

    /// Search a haystack for syncword specified by the test algorithm.
    /// Returns the bit position of the beginning of the syncword.
    /// There may be requirements to the length of the haystack for each detector implementation.
    fn position_in_blocks<I: Iterator<Item = Self::Block>>(&self, haystack: I) -> Option<usize>;
}

pub use self::{
    double16::Double16Detector, double32::Double32Detector, single16::Single16Detector,
    single32::Single32Detector,
};
