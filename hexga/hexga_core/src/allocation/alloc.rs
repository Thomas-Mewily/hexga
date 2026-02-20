use super::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct AllocError;

pub type AllocResult<T = ()> = Result<T, AllocError>;

impl TryFrom<AllocLayout> for std::alloc::Layout
{
    type Error = AllocError;
    fn try_from(value: AllocLayout) -> Result<Self, Self::Error>
    {
        std::alloc::Layout::from_size_align(value.size, value.align).ok_or(AllocError)
    }
}

pub type AllocOutput = NonNullUnaliased<[u8]>;

// Can also be used for grid/texture atlas for ex.
pub unsafe trait AllocFromLayout<Layout = AllocLayout>
{
    type Output;
    fn allocate_layout(&mut self, layout: Layout) -> AllocResult<Self::Output>;
    fn allocate_layout_or_panic(&mut self, layout: Layout) -> Self::Output
    {
        self.allocate_layout(layout).expect("bad alloc")
    }
}

pub trait AllocFromLayoutRaw: AllocFromLayout<Output = AllocOutput>
{
    fn allocate_type<T>(&mut self) -> AllocResult<AllocOutput>
    {
        self.allocate_layout(AllocLayout::of_type::<T>())
    }
    fn allocate_value<T>(&mut self, value: T) -> AllocResult<AllocOutput>
    {
        let ptr = self.allocate_type::<T>()?;
        let dst = ptr.as_ptr() as *mut T;
        unsafe { std::ptr::write(dst, value) };
        Ok(ptr)
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
    fn allocate(&mut self, value: T) -> AllocResult<Self::Box<T>>;
    fn allocate_or_panic(&mut self, value: T) -> Self::Box<T>
    {
        self.allocate(value).expect("bad alloc")
    }
}
impl<T, S> Alloc<T> for S
where
    S: ManagedBox + AllocFromLayout<Output = AllocOutput>,
{
    fn allocate(&mut self, value: T) -> AllocResult<Self::Box<T>>
    {
        let slice = self.allocate_type::<T>().ok_or(AllocError)?;
        unsafe { ptr::write::<T>(slice.as_ptr() as *mut T, value) };
        Ok(unsafe { Self::Box::from_non_null(NonNull::new_unchecked(slice.as_ptr() as *mut u8)) })
    }
}
// Todo allocate_slice ?

pub unsafe trait DeallocFromLayout<Layout = AllocLayout, Ptr = NonNullUnaliased<u8>>:
    AllocFromLayout<Layout>
{
    fn deallocate_layout(&mut self, ptr: Ptr, layout: Layout);
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
    fn deallocate_type<T>(&mut self, ptr: NonNullUnaliased<u8>)
    {
        self.deallocate_layout(ptr, AllocLayout::of_type::<T>())
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
    fn allocate_layout(&mut self, layout: AllocLayout) -> AllocResult<NonNull<[u8]>>
    {
        let layout = std::alloc::Layout::try_from(layout).ok_or(AllocError)?;
        let ptr = unsafe { std::alloc::alloc(layout) };
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
    fn deallocate_layout(&mut self, ptr: NonNullUnaliased<u8>, layout: AllocLayout)
    {
        let layout = std::alloc::Layout::try_from(layout).expect("invalid layout");
        unsafe { std::alloc::dealloc(ptr.as_ptr() as *mut u8, layout) };
    }

    fn realloc_layout(
        &mut self,
        ptr: NonNullUnaliased<u8>,
        old_layout: AllocLayout,
        new_layout: AllocLayout,
    ) -> AllocResult<AllocOutput>
    {
        let old_layout = std::alloc::Layout::try_from(old_layout).ok_or(AllocError)?;
        let new_layout = std::alloc::Layout::try_from(new_layout).ok_or(AllocError)?;
        if new_layout.align() != old_layout.align()
        {
            return Err(AllocError);
        }

        let new_ptr =
            unsafe { std::alloc::realloc(ptr.as_ptr() as *mut u8, old_layout, new_layout.size()) };
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
