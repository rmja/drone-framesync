pub(crate) mod lzc;
pub(crate) mod popcnt;
pub(crate) mod twoscmpl;

pub trait Comparator<T> {
    const SYNCWORD: T;

    /// Determines if `value` is sufficently similar to `SYNCWORD`.
    fn is_match(value: T) -> bool;
}

pub struct Exact16Comparator<const SW: u16>;
pub struct Exact32Comparator<const SW: u32>;

impl<const SW: u16> Comparator<u16> for Exact16Comparator<SW> {
    const SYNCWORD: u16 = SW;

    #[inline(always)]
    fn is_match(value: u16) -> bool {
        value == Self::SYNCWORD
    }
}

impl<const SW: u32> Comparator<u32> for Exact32Comparator<SW> {
    const SYNCWORD: u32 = SW;

    #[inline(always)]
    fn is_match(value: u32) -> bool {
        value == Self::SYNCWORD
    }
}