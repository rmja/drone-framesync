use super::Comparator;

pub struct TwosComplement16Comparator<const SW: u16, const THR: usize>;
pub struct TwosComplement32Comparator<const SW: u32, const THR: usize>;

impl<const SW: u16, const THR: usize> Comparator<u16> for TwosComplement16Comparator<SW, THR> {
    const SYNCWORD: u16 = SW;
    
    #[inline(always)]
    fn is_match(value: u16) -> bool {
        let mut r = (value ^ Self::SYNCWORD) as i16;

        for _ in 0..THR {
            r = r ^ (r & -r);
        }

        r == 0
    }
}

impl<const SW: u32, const THR: usize> Comparator<u32> for TwosComplement32Comparator<SW, THR> {
    const SYNCWORD: u32 = SW;
    
    #[inline(always)]
    fn is_match(value: u32) -> bool {
        let mut r = (value ^ Self::SYNCWORD) as i32;

        for _ in 0..THR {
            r = r ^ (r & -r);
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
        assert!(TwosComplement16Comparator::<0xFFFF, 1>::is_match(0xFFFF));
        assert!(TwosComplement16Comparator::<0xFFFF, 1>::is_match(0xFFFE));
        assert!(!TwosComplement16Comparator::<0xFFFF, 1>::is_match(0xFFFC));

        assert!(TwosComplement16Comparator::<0xFFFF, 2>::is_match(0xFFFF));
        assert!(TwosComplement16Comparator::<0xFFFF, 2>::is_match(0xFFFE));
        assert!(TwosComplement16Comparator::<0xFFFF, 2>::is_match(0xFFFC));
        assert!(!TwosComplement16Comparator::<0xFFFF, 2>::is_match(0xFFF8));
    }

    #[test]
    fn is_match_32() {
        assert!(TwosComplement32Comparator::<0xFFFFFFFF, 1>::is_match(0xFFFF_FFFF));
        assert!(TwosComplement32Comparator::<0xFFFFFFFF, 1>::is_match(0xFFFE_FFFF));
        assert!(!TwosComplement32Comparator::<0xFFFFFFFF, 1>::is_match(0xFFFC_FFFF));

        assert!(TwosComplement32Comparator::<0xFFFFFFFF, 2>::is_match(0xFFFF_FFFF));
        assert!(TwosComplement32Comparator::<0xFFFFFFFF, 2>::is_match(0xFFFE_FFFF));
        assert!(TwosComplement32Comparator::<0xFFFFFFFF, 2>::is_match(0xFFFC_FFFF));
        assert!(!TwosComplement32Comparator::<0xFFFFFFFF, 2>::is_match(0xFFF8_FFFF));
    }
}
