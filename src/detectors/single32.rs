use core::{convert::TryInto, marker::PhantomData, mem};

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
    type Block = u32;
    const SYNCWORD: u32 = C::SYNCWORD;

    fn from_slice(slice: &[u8]) -> Self::Block {
        let bytes: [u8; mem::size_of::<u32>()] = slice.try_into().unwrap();
        unsafe { mem::transmute(bytes) }
    }

    fn position_in_blocks<I: Iterator<Item = Self::Block>>(&self, haystack: I) -> Option<usize> {
        let mut blocks = haystack;

        // Load the first 32 bit block.
        let block = blocks.next()?;
        let mut current = Window {
            u32: WindowParts32 {
                first: u32::from_be(block),
                second: 0,
            },
        };

        // Iterate for each of the next 32 bit blocks one at a time.
        let mut index = 0;
        for block in blocks {
            let next = u32::from_be(block);

            current.u32.second = next;

            // Search the first 32 bits of the 64 bit window, one at a time.
            for offset in 0..32 {
                if C::is_match(unsafe { current.u32.first }) {
                    return Some(32 * index + offset);
                }

                unsafe {
                    current.u64 <<= 1;
                }
            }

            // Set "next" as "current" for the next iteration.
            current.u32.first = next;
            index += 1;
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use core::mem::size_of;

    use crate::{comparators::Exact32Comparator, sliceext::SliceExt};

    use super::*;
    use bitvec::prelude::*;

    #[test]
    fn position() {
        let detector = Single32Detector::<Exact32Comparator<0xFFFFFFFF>>::new();

        for length in 1..10 {
            let bits = length * size_of::<u32>() * 8;
            for position in 0..bits - 32 {
                let mut haystack = vec![0u32; length];

                {
                    let bits = haystack.as_mut_u8_slice().view_bits_mut::<Msb0>();
                    
                    // Insert 32 bit syncword
                    for i in 0..32 {
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
        let detector = Single32Detector::<Exact32Comparator<0xFFFFFFFF>>::new();
        
        for length in 1..10 {
            let bits = length * size_of::<u32>() * 8;
            let position = bits - 32;
            let mut haystack = vec![0u32; length];

            {
                let bits = haystack.as_mut_u8_slice().view_bits_mut::<Msb0>();
                
                // Insert 32 bit syncword
                for i in 0..32 {
                    bits.set(position + i, true);
                }
            }

            let found = detector.position_in_blocks(haystack.iter().copied());

            assert_eq!(None, found);
        }
    }
}
