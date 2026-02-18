use super::*;


#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct AllocError;

pub type AllocResult<T=()> = Result<T, AllocError>;

impl TryFrom<AllocLayout> for std::alloc::Layout
{
    type Error=AllocError;
    fn try_from(value: AllocLayout) -> Result<Self, Self::Error> {
        std::alloc::Layout::from_size_align(value.size, value.align).ok_or(AllocError)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct Memory;


pub trait MemoryAlloc //: for<T> Alloc<T,Output = Box<T>>
{
    #[track_caller]
    unsafe fn alloc_layout(&mut self, layout: AllocLayout) -> PtrUnaliased<u8> { unsafe { self.try_alloc_layout(layout).expect("bad alloc") } }
    unsafe fn try_alloc_layout(&mut self, layout: AllocLayout) -> AllocResult<PtrUnaliased<u8>>;
}

unsafe impl<T> Alloc<T> for Memory
{
    type Output<Target: ?Sized>=Box<T>;
    fn alloc(&mut self, value: T) -> Option<Self::Output<T>>
    {
        let layout = AllocLayout::of_type::<T>();
        let ptr_u8 = unsafe { self.try_alloc_layout(layout) }.ok()?;
        let ptr = ptr_u8.cast::<T>();

        unsafe {
            ptr.as_ptr().write(value);
        }

        Some(unsafe { Box::from_raw(ptr.as_ptr()) })
    }
}



impl MemoryAlloc for Memory
{
    unsafe fn try_alloc_layout(&mut self, layout: AllocLayout) -> AllocResult<PtrUnaliased<u8>> {
        let layout = std::alloc::Layout::try_from(layout).ok_or(AllocError)?;
        PtrUnaliased::new(unsafe { std::alloc::alloc(layout) }).ok_or(AllocError)
    }
}


pub trait MemoryDealloc
{
    unsafe fn dealloc_layout(&mut self, layout: AllocLayout, ptr: *mut u8);
}


unsafe impl<T> Dealloc<Ptr<T>> for Memory
{
    fn dealloc(&mut self, value: Ptr<T>) {
        unsafe { self.dealloc_layout(AllocLayout::of_type::<T>(), value.as_ptr() as *mut u8) };
    }
}

impl MemoryDealloc for Memory
{
    unsafe fn dealloc_layout(&mut self, layout: AllocLayout, ptr: *mut u8)
    {
        unsafe { std::alloc::dealloc(ptr, layout.try_into().expect("bad layout")) }
    }
}


unsafe impl<T> Realloc<AllocLayout,usize,PtrUnaliased<T>> for Memory {
    /// If this method returns None, then ownership of the value has not been transferred to this allocator, and the contents of the value are unaltered.
    fn realloc(self, ptr: PtrUnaliased<T>, layout: AllocLayout, new_size: usize) -> Option<PtrUnaliased<T>> {
        let raw = unsafe { MemoryRealloc::realloc_layout(self, ptr.as_ptr() as *mut u8, layout, new_size) };
        PtrUnaliased::new(raw as *mut T)
    }
}

pub trait MemoryRealloc
{
    unsafe fn realloc_layout(self, ptr: *mut u8, layout: AllocLayout, new_size: usize) -> *mut u8;
}
impl MemoryRealloc for Memory
{
    unsafe fn realloc_layout(self, ptr: *mut u8, layout: AllocLayout, new_size: usize) -> *mut u8 {
        unsafe { std::alloc::realloc(ptr, layout.try_into().expect("bad layout"), new_size) }
    }
}