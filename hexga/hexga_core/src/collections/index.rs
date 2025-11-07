use super::*;


pub trait IndexExtension : Sized
{
    #[inline(always)]
    fn get<C>(self, collection: &C) -> Option<&C::Output> where C: Get<Self>, C::Output : Sized { collection.get(self) }
    #[inline(always)]
    #[track_caller]
    unsafe fn get_unchecked<C>(self, collection: &C) -> &C::Output where C: Get<Self>, C::Output : Sized { unsafe { collection.get_unchecked(self) } }
    #[inline(always)]
    fn get_mut<C>(self, collection: &mut C) -> Option<&C::Output> where C: GetMut<Self>, C::Output : Sized { collection.get(self) }
    #[inline(always)]
    #[track_caller]
    unsafe fn get_unchecked_mut<C>(self, collection: &mut C) -> &mut C::Output where C: GetMut<Self>, C::Output : Sized { unsafe { collection.get_unchecked_mut(self) } }

    #[inline(always)]
    fn exist<C>(self, collection: &C) -> bool where C: Get<Self>, C::Output : Sized { collection.is_index_valid(self) }
    #[inline(always)]
    fn remove<C>(self, collection: &mut C) -> Option<C::Output> where C: Remove<Self> { collection.remove(self) }
}

impl IndexExtension for usize {}
impl IndexExtension for isize {}
