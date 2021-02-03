use core::{marker::PhantomData, mem::size_of};

use crate::comparators::Comparator;

use super::Detector;

#[derive(Clone, Copy)]
union Window {
    u64: u64,
    u32: WindowParts32,
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

pub struct Single32Detector<C: Comparator<u32>> {
    comparator: PhantomData<C>,
}

impl<C: Comparator<u32>> Single32Detector<C> {
    pub const fn new() -> Self {
        Self {
            comparator: PhantomData,
        }
    }
}

impl<C: Comparator<u32>> Detector<u32> for Single32Detector<C> {
    const SYNCWORD: u32 = C::SYNCWORD;

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
            },
        };

        // Iterate for each of the next 32 bit blocks one at a time.
        for (index, block) in blocks {
            let next = u32::from_be(block);

            current.u32.second = next;

            // Search the first 32 bits of the 64 bit window, one at a time.
            for offset in 0..32 {
                if C::is_match(unsafe { current.u32.first }) {
                    return Some(32 * (index - 1) + offset);
                }

                unsafe {
                    current.u64 <<= 1;
                }
            }

            // Set "next" as "current" for the next iteration.
            current.u32.first = next;
        }

        // Test the last block.
        if C::is_match(unsafe { current.u32.first }) {
            Some(haystack.len() * 8 - 32)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::comparators::Exact32Comparator;

    use super::*;
    use bitvec::prelude::*;

    #[test]
    fn position() {
        let detector = Single32Detector::<Exact32Comparator<0xFFFFFFFF>>::new();
        let lengths = [4 * 8, 8 * 8, 12 * 8, 16 * 8, 20 * 8, 24 * 8, 28 * 8, 32 * 8];

        for length in lengths.iter().copied() {
            for position in 0..=length - 32 {
                let mut haystack = bitvec::bitvec![Msb0, u8; 0; length];

                // Insert 32 bit syncword
                for i in 0..32 {
                    haystack.set(position + i, true);
                }

                let found = detector.position(haystack.as_raw_slice());

                println!("Found {:?} in {:?}", found, haystack);

                assert_eq!(Some(position), found);
            }
        }
    }
}
