use super::*;

#[allow(unused)]
#[cfg(feature = "std")]
mod alloc_fn
{
    pub use std::alloc::{alloc, alloc_zeroed, dealloc, realloc};
}
#[allow(unused)]
#[cfg(not(feature = "std"))]
mod alloc_fn
{
    pub use alloc::alloc::{alloc, alloc_zeroed, dealloc, realloc};
}
use alloc_fn::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct AllocError;

pub type AllocResult<T = ()> = Result<T, AllocError>;

impl TryFrom<AllocLayout> for core::alloc::Layout
{
    type Error = AllocError;
    fn try_from(value: AllocLayout) -> Result<Self, Self::Error>
    {
        core::alloc::Layout::from_size_align(value.size, value.align).ok_or(AllocError)
    }
}

pub type AllocOutput = NonNullUnaliased<[u8]>;

// Can also be used for grid/texture atlas for ex.
pub unsafe trait AllocFromLayout<Layout = AllocLayout>
{
    type Output;
    fn alloc_layout(&mut self, layout: Layout) -> AllocResult<Self::Output>;
    fn alloc_layout_or_panic(&mut self, layout: Layout) -> Self::Output
    {
        self.alloc_layout(layout).expect("bad alloc")
    }
}

pub trait AllocFromLayoutRaw: AllocFromLayout<Output = AllocOutput>
{
    fn alloc_type<T>(&mut self) -> AllocResult<AllocOutput>
    {
        self.alloc_layout(AllocLayout::of_type::<T>())
    }
    fn alloc_type_zeroed<T>(&mut self) -> AllocResult<AllocOutput>
    {
        let p = self.alloc_type::<T>()?;
        unsafe {
            let raw: *mut [u8] = p.as_ptr();
            let data: *mut u8 = (*raw).as_mut_ptr();
            core::ptr::write_bytes(data, 0, size_of::<T>());
        }
        Ok(p)
    }
    fn alloc_value<T>(&mut self, value: T) -> AllocResult<AllocOutput>
    {
        let ptr = self.alloc_type::<T>()?;
        let dst = ptr.as_ptr() as *mut T;
        unsafe { core::ptr::write(dst, value) };
        Ok(ptr)
    }
    fn alloc_zeroed<T>(&mut self, layout: AllocLayout) -> AllocResult<AllocOutput>
    {
        let p = self.alloc_layout(layout)?;
        unsafe {
            let raw: *mut [u8] = p.as_ptr();
            let data: *mut u8 = (*raw).as_mut_ptr();
            core::ptr::write_bytes(data, 0, layout.size());
        }
        Ok(p)
    }
}
impl<S> AllocFromLayoutRaw for S where S: AllocFromLayout<Output = AllocOutput> {}

pub trait ManagedBox
{
    // Only support thin pointer, not T: ?Sized
    type Box<T>: Pointer;
}

pub trait Alloc<T>: ManagedBox
{
    // Can't add an extra lifetime to indicate the lifetime of the box, because the borrow checker
    // will also think we will borrow the self reference (allocator), which is not the case !!!.
    // So if we add the referece, we can only allocate 1 object, because on the second one the borrow checker will complain about already borrowed lifetime.
    //
    // fn alloc<'a>(&'a mut self, value: T) -> AllocResult<Self::Box<'a, T>>;
    fn alloc(&mut self, value: T) -> AllocResult<Self::Box<T>>;
    fn alloc_or_panic(&mut self, value: T) -> Self::Box<T> { self.alloc(value).expect("bad alloc") }
}
impl<T, S> Alloc<T> for S
where
    S: ManagedBox + AllocFromLayout<Output = AllocOutput>,
{
    fn alloc(&mut self, value: T) -> AllocResult<Self::Box<T>>
    {
        let slice = self.alloc_type::<T>().ok_or(AllocError)?;
        unsafe { ptr::write::<T>(slice.as_ptr() as *mut T, value) };
        Ok(unsafe { Self::Box::from_non_null(NonNull::new_unchecked(slice.as_ptr() as *mut u8)) })
    }
}
// Todo allocate_slice ?

pub unsafe trait DeallocFromLayout<Layout = AllocLayout, Ptr = NonNullUnaliased<u8>>:
    AllocFromLayout<Layout>
{
    fn dealloc_layout(&mut self, ptr: Ptr, layout: Layout);
    fn realloc_layout(
        &mut self,
        ptr: Ptr,
        old_layout: Layout,
        new_layout: Layout,
    ) -> AllocResult<Self::Output>;
    fn realloc_layout_or_panic(
        &mut self,
        ptr: Ptr,
        old_layout: Layout,
        new_layout: Layout,
    ) -> Self::Output
    {
        self.realloc_layout(ptr, old_layout, new_layout)
            .expect("bad realloc")
    }
}

pub unsafe trait DeallocFromLayoutRaw: DeallocFromLayout
{
    fn dealloc_type<T>(&mut self, ptr: NonNullUnaliased<u8>)
    {
        self.dealloc_layout(ptr, AllocLayout::of_type::<T>())
    }
    //fn realloc_type<T>(&mut self, ptr: Box<[T]>, new_layout: Layout) -> AllocResult<Ptr>;
}
unsafe impl<S> DeallocFromLayoutRaw for S where S: DeallocFromLayout {}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct Memory;

impl ManagedBox for Memory
{
    type Box<T> = Box<T>;
}

unsafe impl AllocFromLayout for Memory
{
    type Output = AllocOutput;
    fn alloc_layout(&mut self, layout: AllocLayout) -> AllocResult<NonNull<[u8]>>
    {
        let layout = core::alloc::Layout::try_from(layout).ok_or(AllocError)?;
        let ptr = unsafe { alloc(layout) };
        if ptr.is_null()
        {
            Err(AllocError)
        }
        else
        {
            Ok(NonNullUnaliased::slice_from_raw_parts(
                unsafe { NonNullUnaliased::new_unchecked(ptr) },
                layout.size(),
            ))
        }
    }
}
unsafe impl DeallocFromLayout for Memory
{
    fn dealloc_layout(&mut self, ptr: NonNullUnaliased<u8>, layout: AllocLayout)
    {
        let layout = core::alloc::Layout::try_from(layout).expect("invalid layout");
        unsafe { dealloc(ptr.as_ptr() as *mut u8, layout) };
    }

    fn realloc_layout(
        &mut self,
        ptr: NonNullUnaliased<u8>,
        old_layout: AllocLayout,
        new_layout: AllocLayout,
    ) -> AllocResult<AllocOutput>
    {
        let old_layout = core::alloc::Layout::try_from(old_layout).ok_or(AllocError)?;
        let new_layout = core::alloc::Layout::try_from(new_layout).ok_or(AllocError)?;
        if new_layout.align() != old_layout.align()
        {
            return Err(AllocError);
        }

        let new_ptr = unsafe { realloc(ptr.as_ptr() as *mut u8, old_layout, new_layout.size()) };
        if new_ptr.is_null()
        {
            Err(AllocError)
        }
        else
        {
            Ok(NonNullUnaliased::slice_from_raw_parts(
                unsafe { NonNullUnaliased::new_unchecked(new_ptr) },
                new_layout.size(),
            ))
        }
    }
}
