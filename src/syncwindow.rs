use core::{marker::PhantomData, mem::size_of};

use alloc::{collections::VecDeque, vec::Vec};

use crate::{detectors::Detector, sliceext::SliceExt};

/// An observation window that holds a series of unaligned bits. Bits can be
/// added to the window by extending it, and consumed from the window by
/// running the detector.
pub struct SyncWindow<D: Detector<T>, T> {
    detector: D,
    syncword_type: PhantomData<T>,
    buf: VecDeque<D::Block>,
}

impl<D: Detector<T>, T> SyncWindow<D, T> {
    /// Create a new `SyncWindow`.
    pub fn new(detector: D) -> Self {
        Self {
            detector,
            syncword_type: PhantomData,
            buf: VecDeque::new(),
        }
    }

    /// Add a series of bytes to be seen by the window.
    pub fn extend(&mut self, bytes: &[u8]) {
        let block_count = bytes.len() / size_of::<D::Block>();
        let mut chunks = bytes.chunks_exact(size_of::<D::Block>());

        self.buf.reserve(block_count);
        
        while let Some(chunk) = chunks.next() {
            let block = D::from_slice(chunk);
            self.buf.push_back(block);
        }

        assert_eq!(0, chunks.remainder().len(), "The number of bytes must be a multiple of the detector block size.");
    }

    /// Run the detector on the current window and trim it.
    pub fn detect(&mut self) -> impl Iterator<Item = (u8, Vec<u8>)> {
        // TODO: Figure out a way to do this with generators to avoid the vector allocation.
        let mut matches = Vec::new();
        while self.buf.len() > 0 {
            let (first, second) = self.buf.as_slices();
        
            // Test first section.
            let to_remove = if let Some((m, blocks_before)) = self.detect_next(first, second) {
                matches.push(m);

                // Also remove the block in where the syncword was found to ensure that we do not re-detect the same syncword.
                blocks_before + 1
            }
            else if second.is_empty() {
                // No more sections, remove all but the last item - it is not fully tested,
                // as we need to scan from that item into the next arriving.
                first.len() - 1
            }
            else {
                // Test wrap section.
                let wrap = [first[first.len() - 1], second[0]];

                if let Some((m, blocks_before)) = self.detect_next(&wrap, &second[1..]) {
                    matches.push(m);

                    // Remove all blocks until wrap and maybe one more, to ensure that remove
                    // the block with the detected syncword.
                    first.len() + blocks_before
                }
                else {
                    // Remove all items until wrap (there is still at least one more item available)
                    first.len()
                }
            };

            if to_remove == 0 {
                break;
            }
            else if to_remove >= self.buf.len() {
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

            // Find the number of blocks that was fully consumed before the match.
            let blocks_before = position / size_of::<D::Block>();

            Some(((bit_shifts, remaining), blocks_before))
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
    use core::cmp::min;

    use bitvec::prelude::*;

    use crate::detectors::cortexm4;

    use super::*;

    #[test]
    fn detect_0_shifts_pos0() {
        let mut bs = SyncWindow::new(cortexm4::sync32_tol0::<0xFFFFFFFF>());
        let rx = &[0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00];
        bs.extend(rx);

        let mut iter = bs.detect();
        assert_eq!(Some((0, vec![0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00])), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn detect_0_shifts_pos1() {
        let mut bs = SyncWindow::new(cortexm4::sync32_tol0::<0xFFFFFFFF>());
        let rx = &[0x00, 0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00];
        bs.extend(rx);

        let mut iter = bs.detect();
        assert_eq!(Some((0, vec![0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00])), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn detect_1_shift() {
        let mut bs = SyncWindow::new(cortexm4::sync32_tol0::<0xFFFFFFFF>());
        let rx = &[0x00, 0x7f, 0xff, 0xff, 0xff, 0x80, 0x00, 0x00];
        bs.extend(rx);

        let mut iter = bs.detect();
        assert_eq!(Some((1, vec![0x7f, 0xff, 0xff, 0xff, 0x80, 0x00, 0x00])), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn detect_7_shift() {
        let mut bs = SyncWindow::new(cortexm4::sync32_tol0::<0xFFFFFFFF>());
        let rx = &[0x00, 0x01, 0xff, 0xff, 0xff, 0xFE, 0x00, 0x00];
        bs.extend(rx);

        let mut iter = bs.detect();
        assert_eq!(Some((7, vec![0x01, 0xff, 0xff, 0xff, 0xFE, 0x00, 0x00])), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn detect_match_before_wrap_0_shifts() {
        let mut bs = SyncWindow::new(cortexm4::sync32_tol0::<0xFFFFFFFF>());
        assert_eq!(7, bs.buf.capacity()); // It seems as if we can fit 8.
        bs.extend(&[0x00, 0x00, 0x00, 0x00]);
        bs.extend(&[0x00, 0x00, 0x00, 0x00]);
        bs.extend(&[0x00, 0x00, 0x00, 0x00]);
        bs.extend(&[0x00, 0x00, 0x00, 0x00]);
        bs.extend(&[0x00, 0x00, 0x00, 0x00]);
        bs.extend(&[0x00, 0x00, 0x00, 0x00]);
        bs.extend(&[0x00, 0x00, 0x00, 0x00]);
        bs.buf.drain(0..6);
        bs.extend(&[0xff, 0xff, 0xff, 0xff]); // Insert in the last position before wrap
        bs.extend(&[0x00, 0x00, 0x00, 0x00]); // Insert in the first position after wrap

        assert_eq!(1, bs.buf.as_slices().1.len(), "The buffer should wrap for the test to be significant");

        let mut iter = bs.detect();
        assert_eq!(Some((0, vec![0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00])), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn detect_match_in_wrap_4_shifts() {
        let mut bs = SyncWindow::new(cortexm4::sync32_tol0::<0xFFFFFFFF>());
        assert_eq!(7, bs.buf.capacity()); // It seems as if we can fit 8.
        bs.extend(&[0x00, 0x00, 0x00, 0x00]);
        bs.extend(&[0x00, 0x00, 0x00, 0x00]);
        bs.extend(&[0x00, 0x00, 0x00, 0x00]);
        bs.extend(&[0x00, 0x00, 0x00, 0x00]);
        bs.extend(&[0x00, 0x00, 0x00, 0x00]);
        bs.extend(&[0x00, 0x00, 0x00, 0x00]);
        bs.extend(&[0x00, 0x00, 0x00, 0x00]);
        bs.buf.drain(0..6);
        bs.extend(&[0x00, 0x00, 0x0f, 0xff]); // Insert in the last position before wrap
        bs.extend(&[0xff, 0xff, 0xf0, 0x00]); // Insert in the first position after wrap

        assert_eq!(1, bs.buf.as_slices().1.len(), "The buffer should wrap for the test to be significant");

        let mut iter = bs.detect();
        assert_eq!(Some((4, vec![0x0f, 0xff, 0xff, 0xff, 0xf0, 0x00])), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn detect_match_after_wrap_0_shifts() {
        let mut bs = SyncWindow::new(cortexm4::sync32_tol0::<0xFFFFFFFF>());
        assert_eq!(7, bs.buf.capacity()); // It seems as if we can fit 8.
        bs.extend(&[0x00, 0x00, 0x00, 0x00]);
        bs.extend(&[0x00, 0x00, 0x00, 0x00]);
        bs.extend(&[0x00, 0x00, 0x00, 0x00]);
        bs.extend(&[0x00, 0x00, 0x00, 0x00]);
        bs.extend(&[0x00, 0x00, 0x00, 0x00]);
        bs.extend(&[0x00, 0x00, 0x00, 0x00]);
        bs.extend(&[0x00, 0x00, 0x00, 0x00]);
        bs.buf.drain(0..6);
        bs.extend(&[0x00, 0x00, 0x00, 0x00]); // Insert in the last position before wrap
        bs.extend(&[0xff, 0xff, 0xff, 0xff]); // Insert in the first position after wrap
        bs.extend(&[0x00, 0x00, 0x00, 0x00]); // Insert in the first position after wrap

        assert_eq!(2, bs.buf.as_slices().1.len(), "The buffer should wrap for the test to be significant");

        let mut iter = bs.detect();
        assert_eq!(Some((0, vec![0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00])), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn detect_sweep() {
        for extend_before in 0..8 {
            for drain_end in 0..8 {
                for extend_after in 0..8 {
                    for position in 0..=32 {
                        let mut bs = SyncWindow::new(cortexm4::sync32_tol0::<0xFFFFFFFF>());

                        for _ in 0..extend_before {
                            bs.extend(&[0x00, 0x00, 0x00, 0x00]);
                        }

                        bs.buf.drain(0..min(extend_before, drain_end));

                        for _ in 0..extend_after {
                            bs.extend(&[0x00, 0x00, 0x00, 0x00]);
                        }

                        let mut data = [0u8;12];
                        {
                            let bits = data.view_bits_mut::<Msb0>();

                            // Insert 32 bit syncword
                            for i in 0..32 {
                                bits.set(position + i, true);
                            }
                        }
                        bs.extend(&data);

                        println!("{:?}", data);

                        let mut iter = bs.detect();
                        let m = iter.next().unwrap();
                        assert_eq!(position % 8, m.0 as usize);
                        assert!(m.1.len() >= 8);
                        assert_eq!(None, iter.next());
                    }
                }
            }
        }
    }

    #[test]
    fn hello() {
        let mut bs = SyncWindow::new(cortexm4::sync32_tol0::<0xFFFFFFFF>());
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