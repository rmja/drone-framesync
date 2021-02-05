use core::{convert::TryInto, marker::PhantomData, mem};

use crate::comparators::Comparator;

use super::Detector;

#[derive(Clone, Copy)]
union Window {
    u32: u32,
    u16: WindowParts16,
}

#[cfg(target_endian = "little")]
#[derive(Clone, Copy)]
#[repr(C)]
struct WindowParts16 {
    second: u16,
    first: u16,
}

#[cfg(target_endian = "big")]
#[derive(Clone, Copy)]
#[repr(C)]
struct WindowParts16 {
    first: u16,
    second: u16,
}

pub struct Single16Detector<C: Comparator<u16>> {
    comparator: PhantomData<C>,
}

impl<C: Comparator<u16>> Single16Detector<C> {
    pub const fn new() -> Self {
        Self {
            comparator: PhantomData,
        }
    }
}

impl<C: Comparator<u16>> Detector<u16> for Single16Detector<C> {
    type Block = u16;
    const SYNCWORD: u16 = C::SYNCWORD;

    fn from_slice(slice: &[u8]) -> Self::Block {
        let bytes: [u8; mem::size_of::<Self::Block>()] = slice.try_into().unwrap();
        unsafe { mem::transmute(bytes) }
    }

    fn position_in_blocks<I: Iterator<Item = Self::Block>>(&self, haystack: I) -> Option<usize> {
        let mut blocks = haystack;

        // Load the first 16 bit block.
        let block = blocks.next()?;
        let mut current = Window {
            u16: WindowParts16 {
                first: u16::from_be(block),
                second: 0,
            },
        };

        // Iterate for each of the next 16 bit blocks one at a time.
        let mut index = 0;
        for block in blocks {
            let next = u16::from_be(block);

            current.u16.second = next;

            // Search the first 16 bits of the 32 bit window, one at a time.
            for offset in 0..16 {
                if C::is_match(unsafe { current.u16.first }) {
                    return Some(16 * index + offset);
                }

                unsafe {
                    current.u32 <<= 1;
                }
            }

            // Set "next" as "current" for the next iteration.
            current.u16.first = next;
            index += 1;
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use core::mem::size_of;

    use crate::{comparators::Exact16Comparator, sliceext::SliceExt};

    use super::*;
    use bitvec::prelude::*;

    #[test]
    fn position() {
        let detector = Single16Detector::<Exact16Comparator<0xFFFF>>::new();

        for length in 1..10 {
            let bits = length * size_of::<u16>() * 8;
            for position in 0..bits - 16 {
                let mut haystack = vec![0u16; length];

                {
                    let bits = haystack.as_mut_u8_slice().view_bits_mut::<Msb0>();
                    
                    // Insert 16 bit syncword
                    for i in 0..16 {
                        bits.set(position + i, true);
                    }
                }

                let found = detector.position_in_blocks(haystack.iter().copied());

                println!("Found {:?} in {:?}", found, haystack);

                assert_eq!(Some(position), found);
            }
        }
    }

    #[test]
    fn no_match_in_last_possible_position() {
        let detector = Single16Detector::<Exact16Comparator<0xFFFF>>::new();

        for length in 1..10 {
            let bits = length * size_of::<u16>() * 8;
            let position = bits - 16;
            let mut haystack = vec![0u16; length];

            {
                let bits = haystack.as_mut_u8_slice().view_bits_mut::<Msb0>();
                
                // Insert 16 bit syncword
                for i in 0..16 {
                    bits.set(position + i, true);
                }
            }

            let found = detector.position_in_blocks(haystack.iter().copied());

            assert_eq!(None, found);
        }
    }
}
