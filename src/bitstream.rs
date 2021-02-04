use alloc::vec::Vec;

pub struct BitStream {
    buffer: Vec<u8>,
}

impl BitStream {
    pub fn new() -> Self {
        Self {
            buffer: vec![]
        }
    }

    pub fn append(&mut self, bytes: &[u8]) {

    }

    // pub fn positions(&self) -> impl Iterator<Item = usize> {
    //     todo!();
    // }
}

#[cfg(test)]
mod tests {
    #[test]
    fn hello() {


    }
}