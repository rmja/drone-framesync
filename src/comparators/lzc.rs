use super::Comparator;

pub struct LeadingZeroCount16Comparator<const SW: u16, const THR: usize>;
pub struct LeadingZeroCount32Comparator<const SW: u32, const THR: usize>;

impl<const SW: u16, const THR: usize> Comparator<u16> for LeadingZeroCount16Comparator<SW, THR> {
    const SYNCWORD: u16 = SW;
    
    #[inline(always)]
    fn is_match(value: u16) -> bool {
        let mut r = value ^ Self::SYNCWORD;

        for _ in 0..THR {
            r = r.overflowing_shl(r.leading_zeros()).0;
            r &= !0x8000;
        }

        r == 0
    }
}

impl<const SW: u32, const THR: usize> Comparator<u32> for LeadingZeroCount32Comparator<SW, THR> {
    const SYNCWORD: u32 = SW;
    
    #[inline(always)]
    fn is_match(value: u32) -> bool {
        let mut r = value ^ Self::SYNCWORD;

        for _ in 0..THR {
            r = r.overflowing_shl(r.leading_zeros()).0;
            r &= !0x80000000;
        }

        r == 0
    }
}

#[cfg(test)]
pub mod tests {
    use crate::comparators::Comparator;

    use super::*;

    #[test]
    fn is_match_16() {
        assert!(LeadingZeroCount16Comparator::<0xFFFF, 1>::is_match(0xFFFF));
        assert!(LeadingZeroCount16Comparator::<0xFFFF, 1>::is_match(0xFFFE));
        assert!(!LeadingZeroCount16Comparator::<0xFFFF, 1>::is_match(0xFFFC));

        assert!(LeadingZeroCount16Comparator::<0xFFFF, 2>::is_match(0xFFFF));
        assert!(LeadingZeroCount16Comparator::<0xFFFF, 2>::is_match(0xFFFE));
        assert!(LeadingZeroCount16Comparator::<0xFFFF, 2>::is_match(0xFFFC));
        assert!(!LeadingZeroCount16Comparator::<0xFFFF, 2>::is_match(0xFFF8));
    }

    #[test]
    fn is_match_32() {
        assert!(LeadingZeroCount32Comparator::<0xFFFFFFFF, 1>::is_match(0xFFFF_FFFF));
        assert!(LeadingZeroCount32Comparator::<0xFFFFFFFF, 1>::is_match(0xFFFE_FFFF));
        assert!(!LeadingZeroCount32Comparator::<0xFFFFFFFF, 1>::is_match(0xFFFC_FFFF));

        assert!(LeadingZeroCount32Comparator::<0xFFFFFFFF, 2>::is_match(0xFFFF_FFFF));
        assert!(LeadingZeroCount32Comparator::<0xFFFFFFFF, 2>::is_match(0xFFFE_FFFF));
        assert!(LeadingZeroCount32Comparator::<0xFFFFFFFF, 2>::is_match(0xFFFC_FFFF));
        assert!(!LeadingZeroCount32Comparator::<0xFFFFFFFF, 2>::is_match(0xFFF8_FFFF));
    }
}
