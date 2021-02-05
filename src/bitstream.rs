use alloc::vec::Vec;
use ringbuf::{Consumer, Producer, RingBuffer};

use crate::detectors::Detector;

pub struct BitStream {
    producer: Producer<u8>,
    consumer: Consumer<u8>,
}

impl BitStream {
    pub fn new(capacity: usize) -> Self {
        let ringbuf = RingBuffer::new(capacity);
        let (producer, consumer) = ringbuf.split();
        Self {
            producer,
            consumer,
        }
    }

    pub fn extend(&mut self, bytes: &[u8]) {
        let mut iter = bytes.iter().copied();
        self.producer.push_iter(&mut iter);
    }

    pub fn positions<D: Detector<T>, T>(&self, detector: D) {

        self.consumer.access(|first, second| {
            if second.is_empty() {
                // No wrap

                // detector.position(haystack)
            }


        });

        todo!()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn hello() {


    }
}