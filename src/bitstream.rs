use core::{marker::PhantomData, mem::size_of, slice};

use alloc::{collections::VecDeque, vec::Vec};

use crate::detectors::Detector;

pub struct BitStream<D: Detector<T>, T> {
    detector: D,
    syncword_type: PhantomData<T>,
    buf: VecDeque<D::Block>,
}

impl<D: Detector<T>, T> BitStream<D, T> {
    pub fn new(capacity: usize, detector: D) -> Self {
        let buf = VecDeque::with_capacity(capacity / size_of::<D::Block>() + 1);
        Self {
            detector,
            syncword_type: PhantomData,
            buf,
        }
    }

    fn as_u8_slice(buf: &[D::Block]) -> &[u8] {
        unsafe {
            slice::from_raw_parts_mut(buf.as_ptr() as *mut u8, buf.len() * size_of::<D::Block>())
        }
    }

    pub fn extend(&mut self, bytes: &[u8]) {
        let mut chunks = bytes.chunks_exact(size_of::<D::Block>());

        for chunk in chunks {
            let block = D::get_block(chunk);
            self.buf.push_back(block);
        }

        assert_eq!(0, chunks.remainder().len());
    }

    pub fn detect(&mut self) -> impl Iterator<Item = (u8, Vec<u8>)> {
        // TODO: Figure out a way to do this generators to avoid the vector allocation.
        let mut matches = Vec::new();
        loop {
            let (first, second) = self.buf.as_slices();
            if second.is_empty() {
                if let Some(position) = self.detector.position_in_blocks(first.iter().copied()) {
                    let index = position / 8;
                    let shifts = (position - index * 8) as u8;

                    let remaining = Self::as_u8_slice(first)[index..].to_vec();
                    matches.push((shifts, remaining));
                } 
                else {
                    break;
                }
            }
            else {
                todo!();
            }
        }

        // drop(self.buf.drain(..self.buf.len() - ));

        matches.into_iter()
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
    fn hello() {
        let detector = cortexm4::sync32_tol0::<0xFFFFFFFF>();
        let mut bs = BitStream::new(16, detector);

        // let rb = RingBuffer::new(20);
        // let (prod, cons) = rb.split();


        let spi = &[0x00, 0xff, 0xff, 0xff, 0xff, 0x00];
        bs.extend(spi);

        // let mut receptions: Vec<FrameReception> = vec![];

        // // Add the received bytes into all concurrent receiptions.
        // for rec in receptions.iter_mut() {
        //     rec.buffer.extend_from_slice(spi);
        // }

        // while let Some(shifts, remainder) = bs.detect() {
        //     let buffer = Vec::with_capacity(remainder.len());
        //     buffer.extend_from_slice(remainder);
        //     receptions.push(FrameReception {
        //         buffer,
        //         shifts,
        //         frame_len: None,
        //     });
        // }


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