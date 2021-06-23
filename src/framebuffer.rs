use core::ops::Range;

use alloc::vec::Vec;

pub struct FrameBuffer {
    /// The frame receive buffer. This buffer is not bit aligned.
    pub receive_buffer: Vec<u8>,
    /// The number if bit shifts (0..7) that needs to be applied to the receive_buffer to make it aligned.
    pub shifts: u8,
    pub frame_len: Option<usize>,
}

impl FrameBuffer {
    pub fn is_received(&self) -> bool {
        if let Some(frame_len) = self.frame_len {
            if self.shifts == 0 {
                self.receive_buffer.len() >= frame_len
            } else {
                // We need one more byte to receive the last bits for the receive
                self.receive_buffer.len() > frame_len
            }
        } else {
            // Frame length has not yet been derived
            false
        }
    }

    pub fn aligned_len(&self) -> usize {
        let len = self.receive_buffer.len();
        if self.shifts == 0 {
            // No shifts needs to be made in the receive buffer
            len
        }
        else if len > 0 {
            // Shifts needs to be made. The aligned result will be one byte shorter than the receive buffer
            len - 1
        }
        else {
            // The receive buffer is empty
            0
        }
    }

    pub fn get_aligned(&self) -> Vec<u8> {
        self.get_aligned_part(0..self.frame_len.unwrap())
    }

    pub fn get_aligned_part(&self, range: Range<usize>) -> Vec<u8> {
        if self.shifts == 0 {
            // Receive buffer is already aligned
            self.receive_buffer.as_slice()[range].to_vec()
        } else {
            // Take the relevant bytes from the receive buffer, including that last partial byte
            let unaligned = &self.receive_buffer.as_slice()[range.start..range.end + 1];

            let left_shifts = self.shifts;
            let right_shifts = 8 - left_shifts;

            let mut iter = unaligned.iter();
            let byte = *iter.next().unwrap();
            let mut partial = byte << left_shifts;

            let mut aligned = Vec::with_capacity(range.end - range.start);
            while let Some(byte) = iter.next().copied() {
                aligned.push(partial | (byte >> right_shifts));
                partial = byte << left_shifts;
            }

            aligned
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{detectors::cortexm4, SyncWindow};

    #[test]
    fn align_without_shifts() {
        let frame = FrameBuffer {
            receive_buffer: vec![1, 2, 3],
            shifts: 0,
            frame_len: None,
        };

        let aligned = frame.get_aligned_part(0..3);

        assert_eq!(vec![1, 2, 3], aligned);
    }

    #[test]
    fn align_with_shifts() {
        let frame = FrameBuffer {
            receive_buffer: vec![0x70, 0xF0, 0x00],
            shifts: 1,
            frame_len: None,
        };

        let aligned = frame.get_aligned_part(0..2);

        assert_eq!(vec![0xE1, 0xE0], aligned);
    }

    #[test]
    fn receptions() {
        let mut bs = SyncWindow::new(cortexm4::sync32_tol0::<0xFFFFFFFF>());
        let rx = &[0x00, 0xff, 0xff, 0xff, 0xff, 0x01, 0x00, 0x00];

        let mut ongoing_receptions: Vec<FrameBuffer> = vec![];

        // Add the received bytes into all ongoing, concurrent receiptions.
        for rec in ongoing_receptions.iter_mut() {
            rec.receive_buffer.extend_from_slice(rx);
        }

        bs.extend(rx);
        while let Some((shifts, remainder)) = bs.detect().next() {
            ongoing_receptions.push(FrameBuffer {
                receive_buffer: remainder,
                shifts,
                frame_len: None, // Not yet determined
            });
        }

        for handle in ongoing_receptions.iter_mut() {
            if handle.frame_len.is_none() && handle.receive_buffer.len() > 4 + 1 {
                // We have at least the syncword and the length
                // Derive the length
                let length_field = handle.get_aligned_part(4..5)[0];
                handle.frame_len = Some(4 + 1 + usize::from(length_field));
            }

            if handle.is_received() {
                let aligned = handle.get_aligned();
                assert_eq!(4 + 1 + 1, aligned.len());
            }
        }
    }
}
