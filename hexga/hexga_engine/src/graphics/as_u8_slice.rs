use super::*;


pub(crate) unsafe trait AsU8Slice
{
    fn as_u8_slice(&self) -> &[u8];
    fn as_u8_slice_mut(&mut self) -> &mut [u8];
}
unsafe impl<T> AsU8Slice for T where T:Sized + Copy
{
    fn as_u8_slice(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(
                (self as *const Self).cast::<u8>(),
                std::mem::size_of::<Self>(),
            )
        }
    }

    fn as_u8_slice_mut(&mut self) -> &mut [u8] {
        unsafe {
            std::slice::from_raw_parts_mut(
                (self as *mut Self).cast::<u8>(),
                std::mem::size_of::<Self>(),
            )
        }
    }
}

unsafe impl<T> AsU8Slice for [T] where T:Sized + Copy
{
    fn as_u8_slice(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(
                self.as_ptr().cast::<u8>(),
                self.len() * std::mem::size_of::<T>(),
            )
        }
    }

    fn as_u8_slice_mut(&mut self) -> &mut [u8] {
        unsafe {
            std::slice::from_raw_parts_mut(
                self.as_mut_ptr().cast::<u8>(),
                self.len() * std::mem::size_of::<T>(),
            )
        }
    }
}