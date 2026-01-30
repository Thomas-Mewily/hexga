use super::*;

pub trait IndexExtension<C>: Sized
{
    #[inline(always)]
    fn get(self, collection: &C) -> Option<&C::Output>
    where
        C: Get<Self>,
        C::Output: Sized,
    {
        collection.get(self)
    }
    #[inline(always)]
    #[track_caller]
    unsafe fn get_unchecked(self, collection: &C) -> &C::Output
    where
        C: Get<Self>,
        C::Output: Sized,
    {
        unsafe { collection.get_unchecked(self) }
    }
    #[inline(always)]
    fn get_mut(self, collection: &mut C) -> Option<&C::Output>
    where
        C: GetMut<Self>,
        C::Output: Sized,
    {
        collection.get(self)
    }
    #[inline(always)]
    #[track_caller]
    unsafe fn get_unchecked_mut(self, collection: &mut C) -> &mut C::Output
    where
        C: GetMut<Self>,
        C::Output: Sized,
    {
        unsafe { collection.get_unchecked_mut(self) }
    }

    #[inline(always)]
    fn is_valid(self, collection: &C) -> bool
    where
        C: Get<Self>,
        C::Output: Sized,
    {
        collection.is_index_valid(self)
    }
    #[inline(always)]
    fn is_invalid(self, collection: &C) -> bool
    where
        C: Get<Self>,
        C::Output: Sized,
    {
        collection.is_index_invalid(self)
    }
    #[inline(always)]
    fn remove(self, collection: &mut C) -> Option<C::Output>
    where
        C: Remove<Self>,
    {
        collection.remove(self)
    }
}

impl<I, T> IndexExtension<T> for I where I: std::slice::SliceIndex<T> {}
