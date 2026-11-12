use super::*;

/// Mostly used for indexing from some ID, for a single element.
pub trait IndexID<C>: Sized
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

impl<C> IndexID<C> for bool {}
impl<C> IndexID<C> for char {}

map_on_number!(
    ($type_name : ident) =>
    {
        {
            use ::core::num::{Wrapping, Saturating};

            impl<C> IndexID<C> for $type_name {}
            impl<C> IndexID<C> for Wrapping<$type_name> {}
            impl<C> IndexID<C> for Saturating<$type_name> {}
        }
    }
);
