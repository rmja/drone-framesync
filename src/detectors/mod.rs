pub mod cortexm4;
mod single16;
mod single32;
mod double16;
mod double32;

pub trait Detector<T> {
    const SYNCWORD: T;

    /// Search a haystack for syncword specified by the test algorithm.
    /// There may be requirements to the length of the haystack for each detector implementation.
    fn position(&self, haystack: &[u8]) -> Option<usize>;
}

pub use self::{
    single16::Single16Detector,
    single32::Single32Detector,
    double16::Double16Detector,
    double32::Double32Detector,
};
