pub mod cortexm4;
mod double16;
mod double32;
mod single16;
mod single32;

use core::{mem::{align_of, align_of_val, size_of}, slice};

pub trait Detector<T> {
    type Block;

    const SYNCWORD: T;

    /// Search a haystack for syncword specified by the test algorithm.
    /// Returns the bit position of the beginning of the syncword and the number
    /// of _bytes_ in `haystack` that was fully consumed found not to contain
    /// the syncword.
    ///
    /// # Safety
    ///
    /// The haystack must have alignment similar to `Self::Block`.
    unsafe fn position(&self, haystack: &[u8]) -> (Option<usize>, usize) {
        let block_size = size_of::<Self::Block>();
        let block_count = haystack.len() / block_size;

        debug_assert_eq!(align_of::<Self::Block>(), align_of_val(haystack));

        let haystack = slice::from_raw_parts(haystack.as_ptr() as *const Self::Block, block_count);

        debug_assert_eq!(haystack.len(), block_count);

        match self.position_in_blocks(haystack) {
            Some(position) => (Some(position), position / 8 ),
            None => (None, block_count * block_size - size_of::<T>())
        }
    }

    /// Search a haystack for syncword specified by the test algorithm.
    /// Returns the bit position of the beginning of the syncword.
    /// There may be requirements to the length of the haystack for each detector implementation.
    fn position_in_blocks(&self, haystack: &[Self::Block]) -> Option<usize>;
}

pub use self::{
    double16::Double16Detector, double32::Double32Detector, single16::Single16Detector,
    single32::Single32Detector,
};
