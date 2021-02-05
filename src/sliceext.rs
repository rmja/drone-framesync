use core::{mem::size_of, slice};

pub trait SliceExt {
    fn as_u8_slice(&self) -> &[u8];

    fn as_mut_u8_slice(&mut self) -> &mut [u8];
}

impl<T> SliceExt for [T] {
    #[inline]
    fn as_u8_slice(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(self.as_ptr() as *const u8, self.len() * size_of::<T>())
        }
    }

    #[inline]
    fn as_mut_u8_slice(&mut self) -> &mut [u8] {
        unsafe {
            slice::from_raw_parts_mut(self.as_ptr() as *mut u8, self.len() * size_of::<T>())
        }
    }
}
