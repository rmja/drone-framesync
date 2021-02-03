use core::{marker::PhantomData, mem::size_of};

use crate::comparators::Comparator;

use super::Detector;

#[derive(Clone, Copy)]
union Window {
    u64: u64,
    u32: WindowParts32,
    u16: WindowParts16,
}

#[cfg(target_endian = "little")]
#[derive(Clone, Copy)]
#[repr(C)]
struct WindowParts32 {
    second: u32,
    first: u32,
}

#[cfg(target_endian = "big")]
#[derive(Clone, Copy)]
#[repr(C)]
struct WindowParts32 {
    first: u32,
    second: u32,
}

#[cfg(target_endian = "little")]
#[derive(Clone, Copy)]
#[repr(C)]
struct WindowParts16 {
    fourth: u16,
    third: u16,
    second: u16,
    first: u16,
}

#[cfg(target_endian = "big")]
#[derive(Clone, Copy)]
#[repr(C)]
struct WindowParts16 {
    first: u32,
    second: u32,
    third: u32,
    fourth: u32,
}

pub struct Double16Detector<C: Comparator<u16>> {
    comparator: PhantomData<C>,
}

impl<C: Comparator<u16>> Double16Detector<C> {
    pub const fn new() -> Self {
        Self {
            comparator: PhantomData
        }
    }
}

impl<C: Comparator<u16>> Detector<u16> for Double16Detector<C> {
    const SYNCWORD: u16 = C::SYNCWORD;

    fn position(&self, haystack: &[u8]) -> Option<usize> {
        assert_eq!(0, haystack.len() % size_of::<u32>());
        let haystack: &[u32] = unsafe { core::mem::transmute(haystack) };
        let mut blocks = haystack.iter().copied().enumerate();

        // Load the first 32 bit block.
        let (_, block) = blocks.next()?;
        let mut current = Window {
            u32: WindowParts32 {
                first: u32::from_be(block),
                second: 0,
            }
        };

        // Iterate for each of the next 32 bit blocks one at a time.
        for (index, block) in blocks {
            let next = u32::from_be(block);

            current.u32.second = next;

            // Search the first 16+16 bits of the 32 bit window, one at a time.
            for offset in 0..16 {
                if C::is_match(unsafe { current.u16.first }) {
                    return Some(32 * (index - 1) + offset);
                }

                if C::is_match(unsafe { current.u16.second }) {
                    return Some(32 * (index - 1) + 16 + offset);
                }

                unsafe {
                    current.u64 <<= 1;
                }
            }

            // Set "next" as "current" for the next iteration.
            current.u32.first = next;
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use crate::comparators::Exact16Comparator;

    use super::*;
    use bitvec::prelude::*;
    
    #[test]
    fn position() {
        let detector = Double16Detector::<Exact16Comparator::<0xFFFF>>::new();
        let lengths = [
            4 * 8,
            8 * 8,
            12 * 8,
            16 * 8,
            20 * 8,
            24 * 8,
            28 * 8,
            32 * 8,
        ];

        for length in lengths.iter().copied() {
            for position in 0..=length - 16 {
                let mut haystack = bitvec::bitvec![Msb0, u8; 0; length];

                // Insert 16 bit syncword
                for i in 0..16 {
                    haystack.set(position + i, true);
                }

                let found = detector.position(haystack.as_raw_slice());

                println!("Found {:?} in {:?}", found, haystack);

                assert_eq!(Some(position), found);
            }
        }
    }
}
