use core::marker::PhantomData;

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

    fn position_in_blocks(&self, haystack: &[u16]) -> Option<usize> {
        let mut blocks = haystack.iter().copied().enumerate();

        // Load the first 16 bit block.
        let (_, block) = blocks.next()?;
        let mut current = Window {
            u16: WindowParts16 {
                first: u16::from_be(block),
                second: 0,
            },
        };

        // Iterate for each of the next 16 bit blocks one at a time.
        for (index, block) in blocks {
            let next = u16::from_be(block);

            current.u16.second = next;

            // Search the first 16 bits of the 32 bit window, one at a time.
            for offset in 0..16 {
                if C::is_match(unsafe { current.u16.first }) {
                    return Some(16 * (index - 1) + offset);
                }

                unsafe {
                    current.u32 <<= 1;
                }
            }

            // Set "next" as "current" for the next iteration.
            current.u16.first = next;
        }

        // Test the last block.
        if C::is_match(unsafe { current.u16.first }) {
            Some(haystack.len() * 8 - 16)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::comparators::Exact16Comparator;

    use super::*;
    use bitvec::prelude::*;

    #[test]
    fn position() {
        let detector = Single16Detector::<Exact16Comparator<0xFFFF>>::new();
        let lengths = [2 * 8, 4 * 8, 6 * 8, 8 * 8, 10 * 8, 12 * 8, 14 * 8, 16 * 8];

        for length in lengths.iter().copied() {
            for position in 0..=length - 16 {
                let mut haystack = bitvec::bitvec![Msb0, u8; 0; length];

                // Insert 16 bit syncword
                for i in 0..16 {
                    haystack.set(position + i, true);
                }

                let (found, consumed) = unsafe { detector.position(haystack.as_raw_slice()) };

                println!("Found {:?} in {:?}", found, haystack);

                assert_eq!(Some(position), found);
                assert_eq!((length - 16)/8, consumed);
            }
        }
    }
}
