// Can also be used for an altlas allocator
// ex: https://docs.rs/guillotiere/latest/guillotiere/struct.AtlasAllocator.html#method.allocate
pub unsafe trait Alloc<T: ?Sized,S=T>
{
    type Output<Target: ?Sized>;
    fn alloc(&mut self, size: S) -> Option<Self::Output<T>>;
    #[track_caller]
    fn alloc_or_panic(&mut self, size: S) -> Self::Output<T> { self.alloc(size).expect("alloc failed") }
}
pub unsafe trait Realloc<Layout,NewLayout,T>
{
    fn realloc(self, value: T, layout: Layout, new_layout: NewLayout) -> Option<T>;
}
pub unsafe trait Dealloc<T: ?Sized>
{
    fn dealloc(&mut self, value: T);
}