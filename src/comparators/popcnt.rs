use super::Comparator;

pub struct PopCount16Comparator<const SW: u16, const THR: u32>;
pub struct PopCount32Comparator<const SW: u32, const THR: u32>;

impl<const SW: u16, const THR: u32> Comparator<u16> for PopCount16Comparator<SW, THR> {
    const SYNCWORD: u16 = SW;
    
    #[inline(always)]
    fn is_match(value: u16) -> bool {
        let r = value ^ Self::SYNCWORD;

        r.count_ones() <= THR
    }
}

impl<const SW: u32, const THR: u32> Comparator<u32> for PopCount32Comparator<SW, THR> {
    const SYNCWORD: u32 = SW;
    
    #[inline(always)]
    fn is_match(value: u32) -> bool {
        let r = value ^ Self::SYNCWORD;

        r.count_ones() <= THR
    }
}

#[cfg(test)]
pub mod tests {
    use crate::comparators::Comparator;

    use super::*;

    #[test]
    fn is_match_16() {
        assert!(PopCount16Comparator::<0xFFFF, 1>::is_match(0xFFFF));
        assert!(PopCount16Comparator::<0xFFFF, 1>::is_match(0xFFFE));
        assert!(!PopCount16Comparator::<0xFFFF, 1>::is_match(0xFFFC));

        assert!(PopCount16Comparator::<0xFFFF, 2>::is_match(0xFFFF));
        assert!(PopCount16Comparator::<0xFFFF, 2>::is_match(0xFFFE));
        assert!(PopCount16Comparator::<0xFFFF, 2>::is_match(0xFFFC));
        assert!(!PopCount16Comparator::<0xFFFF, 2>::is_match(0xFFF8));
    }

    #[test]
    fn is_match_32() {
        assert!(PopCount32Comparator::<0xFFFFFFFF, 1>::is_match(0xFFFF_FFFF));
        assert!(PopCount32Comparator::<0xFFFFFFFF, 1>::is_match(0xFFFE_FFFF));
        assert!(!PopCount32Comparator::<0xFFFFFFFF, 1>::is_match(0xFFFC_FFFF));

        assert!(PopCount32Comparator::<0xFFFFFFFF, 2>::is_match(0xFFFF_FFFF));
        assert!(PopCount32Comparator::<0xFFFFFFFF, 2>::is_match(0xFFFE_FFFF));
        assert!(PopCount32Comparator::<0xFFFFFFFF, 2>::is_match(0xFFFC_FFFF));
        assert!(!PopCount32Comparator::<0xFFFFFFFF, 2>::is_match(0xFFF8_FFFF));
    }
}
