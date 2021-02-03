pub(crate) mod single16;
pub(crate) mod single32;
pub(crate) mod double16;
pub(crate) mod double32;

pub trait Detector<T> {
    const SYNCWORD: T;

    /// Search a haystack for syncword specified by the test algorithm.
    /// The haystack length must be a multiple of size_of::<T>().
    fn position(&self, haystack: &[u8]) -> Option<usize>;
}

