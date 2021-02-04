use core::marker::PhantomData;

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

pub struct Double32Detector<C: Comparator<u32>> {
    comparator: PhantomData<C>,
}

impl<C: Comparator<u32>> Double32Detector<C> {
    pub const fn new() -> Self {
        Self {
            comparator: PhantomData,
        }
    }
}

impl<C: Comparator<u32>> Detector<u32> for Double32Detector<C> {
    type Block = u64;
    const SYNCWORD: u32 = C::SYNCWORD;

    fn position_in_blocks(&self, haystack: &[u64]) -> Option<usize> {
        let mut blocks = haystack.iter().copied().enumerate();

        // Load the first 64 bit block.
        let (_, block) = blocks.next()?;
        let mut current = Window {
            u64: u64::from_be(block),
        };

        // Iterate for each of the next 32 bit blocks one at a time.
        for (index, block) in blocks {
            let next = Window {
                u64: u64::from_be(block),
            };

            let mut window = Window {
                u32: WindowParts32 {
                    first: unsafe { current.u32.second },
                    second: unsafe { next.u32.first },
                },
            };

            // Search the first 32+32 bits of the 64 bit window, one at a time.
            for offset in 0..32 {
                if C::is_match(unsafe { current.u32.first }) {
                    return Some(64 * (index - 1) + offset);
                }

                if C::is_match(unsafe { window.u32.first }) {
                    return Some(64 * (index - 1) + 32 + offset);
                }

                unsafe {
                    current.u64 <<= 1;
                    window.u64 <<= 1;
                }
            }

            // Set "next" as "current" for the next iteration.
            current.u64 = unsafe { next.u64 };
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use crate::comparators::Exact32Comparator;

    use super::*;
    use bitvec::prelude::*;

    #[test]
    fn position() {
        let detector = Double32Detector::<Exact32Comparator<0xFFFFFFFF>>::new();
        let lengths = [8 * 8, 16 * 8, 24 * 8, 32 * 8];

        for length in lengths.iter().copied() {
            for position in 0..=length - 32 {
                let mut haystack = bitvec::bitvec![Msb0, u8; 0; length];

                // Insert 32 bit syncword
                for i in 0..32 {
                    haystack.set(position + i, true);
                }

                let (found, consumed) = unsafe { detector.position(haystack.as_raw_slice()) };

                println!("Found {:?} in {:?}", found, haystack);

                assert_eq!(Some(position), found);
                assert_eq!((length - 32)/8, consumed);
            }
        }
    }
}
