use core::{marker::PhantomData, mem::size_of};

use alloc::{collections::VecDeque, vec::Vec};

use crate::{detectors::Detector, sliceext::SliceExt};

pub struct BitStream<D: Detector<T>, T> {
    detector: D,
    syncword_type: PhantomData<T>,
    buf: VecDeque<D::Block>,
}

impl<D: Detector<T>, T> BitStream<D, T> {
    pub fn new(detector: D) -> Self {
        Self {
            detector,
            syncword_type: PhantomData,
            buf: VecDeque::new(),
        }
    }

    pub fn extend(&mut self, blocks: &[u8]) {
        let block_count = blocks.len() / size_of::<D::Block>();
        let mut chunks = blocks.chunks_exact(size_of::<D::Block>());

        self.buf.reserve(block_count);
        
        while let Some(chunk) = chunks.next() {
            let block = D::from_slice(chunk);
            self.buf.push_back(block);
        }

        assert_eq!(0, chunks.remainder().len());
    }

    pub fn detect(&mut self) -> impl Iterator<Item = (u8, Vec<u8>)> {
        // TODO: Figure out a way to do this generators to avoid the vector allocation.
        let mut matches = Vec::new();
        let mut any_match = true;
        while self.buf.len() > 0 && any_match {
            any_match = false;
            
            let mut to_remove = self.buf.len() - 1;
            let (first, second) = self.buf.as_slices();
        
            if let Some((m, blocks_to_remove)) = self.detect_next(first, second) {
                matches.push(m);
                to_remove = blocks_to_remove;
                any_match = true;
            }
            else if !second.is_empty() {
                // Test wrap section
                let wrap = [first[first.len() - 1], second[0]];

                if let Some((m, blocks_to_remove)) = self.detect_next(&wrap, &second[1..]) {
                    matches.push(m);
                    to_remove = blocks_to_remove;
                    any_match = true;
                }
            }

            if to_remove >= self.buf.len() {
                self.buf.clear();
            }
            else {
                drop(self.buf.drain(0..to_remove));
            }
        }

        matches.into_iter()
    }

    fn detect_next(&self, haystack: &[D::Block], sequel: &[D::Block]) -> Option<((u8, Vec<u8>), usize)> {
        if let Some(position) = self.detector.position_in_blocks(haystack.iter().copied()) {
            let byte_index = position / 8;
            let bit_shifts = (position - byte_index * 8) as u8;

            // Copy out the reminder of the buffer into the match.
            let mut remaining = haystack.as_u8_slice()[byte_index..].to_vec();
            remaining.extend_from_slice(sequel.as_u8_slice());

            // Remove as much from buf so that we will never find the same hit again
            let blocks_to_remove = (position + size_of::<D::Block>() - 1) / size_of::<D::Block>();

            Some(((bit_shifts, remaining), blocks_to_remove))
        }
        else {
            None
        }
    }
}


struct FrameReception {
    buffer: Vec<u8>,
    shifts: u8,
    frame_len: Option<usize>,
}


#[cfg(test)]
mod tests {
    use core::mem::size_of;

    use crate::detectors::cortexm4;

    use super::*;
    use bitvec::prelude::*;

    #[test]
    fn detect_0_shifts() {
        let mut bs = BitStream::new(cortexm4::sync32_tol0::<0xFFFFFFFF>());
        let rx = &[0x00, 0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00];
        bs.extend(rx);

        let mut iter = bs.detect();
        assert_eq!(Some((0, vec![0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00])), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn detect_1_shift() {
        let mut bs = BitStream::new(cortexm4::sync32_tol0::<0xFFFFFFFF>());
        let rx = &[0x00, 0x7f, 0xff, 0xff, 0xff, 0x80, 0x00, 0x00];
        bs.extend(rx);

        let mut iter = bs.detect();
        assert_eq!(Some((1, vec![0x7f, 0xff, 0xff, 0xff, 0x80, 0x00, 0x00])), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn detect_7_shift() {
        let mut bs = BitStream::new(cortexm4::sync32_tol0::<0xFFFFFFFF>());
        let rx = &[0x00, 0x01, 0xff, 0xff, 0xff, 0xFE, 0x00, 0x00];
        bs.extend(rx);

        let mut iter = bs.detect();
        assert_eq!(Some((7, vec![0x01, 0xff, 0xff, 0xff, 0xFE, 0x00, 0x00])), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn detect_wrap_0_shifts() {
        let mut bs = BitStream::new(cortexm4::sync32_tol0::<0xFFFFFFFF>());        
        let rx = &[0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff];
        bs.extend(rx);
        bs.buf.shrink_to_fit();

        assert_eq!(None, bs.detect().next());

        bs.extend(&[0x00, 0x00, 0x00, 0x00]);

        assert_ne!(0, bs.buf.as_slices().1.len(), "The buffer should wrap for the test to be significant");

        let mut iter = bs.detect();
        assert_eq!(Some((0, vec![0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00])), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn hello() {
        let mut bs = BitStream::new(cortexm4::sync32_tol0::<0xFFFFFFFF>());
        let rx = &[0x00, 0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00];
        bs.extend(rx);

        let mut receptions: Vec<FrameReception> = vec![];

        // Add the received bytes into all concurrent receiptions.
        for rec in receptions.iter_mut() {
            rec.buffer.extend_from_slice(rx);
        }

        while let Some((shifts, remainder)) = bs.detect().next() {
            receptions.push(FrameReception {
                buffer: remainder,
                shifts,
                frame_len: None,
            });
        }

        // for handle in receptions.iter_mut() {
        //     if let Some(frame_length) = handle.frame_len {
        //         if handle.buffer.len() > frame_length {
        //             let shifted = handle.buffer;
        //             // Receive
        //         }
        //     }
        //     else if handle.buffer.len() > 4 + 1 {
        //         // We have at least the syncword and the length
        //         // Derive the length
        //         handle.frame_length = Some(17);
        //     }
        // }
        

        // let mut asd = [0u64, 1u64];
        // asd.align_to()

        // let mut asd2 = unsafe { core::slice::from_raw_parts_mut(asd.as_mut_ptr() as *mut u8, asd.len() * size_of::<u64>()) };
        // let view = asd2.view_bits_mut::<Msb0>();

        // view.set(0, true);

        // println!("{:?}", view);
        // assert_eq!(0x80, asd2[0]);

    }
}