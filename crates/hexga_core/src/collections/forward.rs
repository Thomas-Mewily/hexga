use super::*;
use crate::utils::DoubleDerefOption;

macro_rules! forward_read {
    ([$($parameters:tt)*] $wrapper:ty => $inner:ty, $read:path) => {
        impl<$($parameters)*, Output: ?Sized> Collection<Output> for $wrapper
        where $inner: Collection<Output>,
        {
            type Ref<'a> = <$inner as Collection<Output>>::Ref<'a>
            where Self: 'a, Output: 'a;
        }

        impl<$($parameters)*, Idx> CollectionIndex<Idx> for $wrapper
        where $inner: CollectionIndex<Idx>,
        {
            type Output = <$inner as CollectionIndex<Idx>>::Output;
            type Ref<'a> = <$inner as CollectionIndex<Idx>>::Ref<'a>
            where Self: 'a, Self::Output: 'a;
        }

        impl<$($parameters)*, Idx> Get<Idx> for $wrapper
        where $inner: Get<Idx>,
        {
            fn get<'a>(&'a self, index: Idx) -> Option<Self::Ref<'a>>
            where Self::Output: 'a,
            {
                Get::get($read(self), index)
            }
        }

        impl<$($parameters)*, Idx> TryGet<Idx> for $wrapper
        where $inner: TryGet<Idx>,
        {
            type Error = <$inner as TryGet<Idx>>::Error;

            fn try_get<'a>(&'a self, index: Idx) -> Result<Self::Ref<'a>, Self::Error>
            where Self::Output: 'a,
            {
                TryGet::try_get($read(self), index)
            }
        }

        forward_peek!([$($parameters)*] $wrapper => $inner, $read, First, first);
        forward_peek!([$($parameters)*] $wrapper => $inner, $read, Last, last);

        forward_inspect!([$($parameters)*] $wrapper => $inner, $read);
    };
}

macro_rules! forward_inspect {
    ([$($parameters:tt)*] $wrapper:ty => $inner:ty, $read:path) => {
        impl<$($parameters)*> Length for $wrapper
        where $inner: Length,
        {
            fn len(&self) -> usize { Length::len(&*$read(self)) }
            fn is_empty(&self) -> bool { Length::is_empty(&*$read(self)) }
        }

        impl<$($parameters)*, T> Capacity<T> for $wrapper
        where $inner: Capacity<T>,
        {
            fn capacity(&self) -> usize { Capacity::capacity(&*$read(self)) }
        }

        impl<$($parameters)*> CollectionBijective for $wrapper where $inner: CollectionBijective {}
    };
}

macro_rules! forward_peek {
    ([$($parameters:tt)*] $wrapper:ty => $inner:ty, $read:path, $trait:ident, $method:ident) => {
        impl<$($parameters)*, Idx> $trait<Idx> for $wrapper
        where $inner: $trait<Idx>,
        {
            fn $method<'a>(&'a self) -> Option<Self::Ref<'a>>
            where Self::Output: 'a,
            {
                $trait::$method($read(self))
            }
        }
    };
}

macro_rules! forward_mut_peek {
    ([$($parameters:tt)*] $wrapper:ty => $inner:ty, ($receiver:ident => $access:expr), $trait:ident, $method:ident) => {
        impl<$($parameters)*, Idx> $trait<Idx> for $wrapper
        where $inner: $trait<Idx>,
        {
            fn $method<'a>(&'a mut self) -> Option<Self::RefMut<'a>>
            where Self::Output: 'a,
            {
                $trait::$method({ let $receiver = self; $access })
            }
        }
    };
}

macro_rules! forward_write {
    ([$($parameters:tt)*] $wrapper:ty => $inner:ty, ($receiver:ident => $access:expr)) => {
        impl<$($parameters)*, Output: ?Sized> CollectionMut<Output> for $wrapper
        where $inner: CollectionMut<Output>,
        {
            type RefMut<'a> = <$inner as CollectionMut<Output>>::RefMut<'a>
            where Self: 'a, Output: 'a;
        }

        impl<$($parameters)*, Idx> CollectionIndexMut<Idx> for $wrapper
        where $inner: CollectionIndexMut<Idx>,
        {
            type RefMut<'a> = <$inner as CollectionIndexMut<Idx>>::RefMut<'a>
            where Self: 'a, Self::Output: 'a;
        }

        impl<$($parameters)*, Idx> GetMut<Idx> for $wrapper
        where $inner: GetMut<Idx>,
        {
            fn get_mut<'a>(&'a mut self, index: Idx) -> Option<Self::RefMut<'a>>
            where Self::Output: 'a,
            {
                GetMut::get_mut({ let $receiver = self; $access }, index)
            }
        }

        impl<$($parameters)*, Idx> TryGetMut<Idx> for $wrapper
        where $inner: TryGetMut<Idx>,
        {
            fn try_get_mut<'a>(&'a mut self, index: Idx) -> Result<Self::RefMut<'a>, Self::Error>
            where Self::Output: 'a,
            {
                TryGetMut::try_get_mut({ let $receiver = self; $access }, index)
            }
        }

        impl<$($parameters)*, Idx> GetManyMut<Idx> for $wrapper
        where $inner: GetManyMut<Idx>,
        {
            fn try_get_many_mut<'a, const N: usize>(&'a mut self, indices: [Idx; N]) -> Result<[Self::RefMut<'a>; N], ManyMutError>
            where Self::Output: 'a,
            {
                GetManyMut::try_get_many_mut({ let $receiver = self; $access }, indices)
            }

            fn get_many_mut<'a, const N: usize>(&'a mut self, indices: [Idx; N]) -> Option<[Self::RefMut<'a>; N]>
            where Self::Output: 'a,
            {
                GetManyMut::get_many_mut({ let $receiver = self; $access }, indices)
            }
        }

        forward_mut_peek!([$($parameters)*] $wrapper => $inner, ($receiver => $access), FirstMut, first_mut);
        forward_mut_peek!([$($parameters)*] $wrapper => $inner, ($receiver => $access), LastMut, last_mut);

        forward_edit!([$($parameters)*] $wrapper => $inner, ($receiver => $access));
    };
}

macro_rules! forward_edit {
    ([$($parameters:tt)*] $wrapper:ty => $inner:ty, ($receiver:ident => $access:expr)) => {
        forward_push!([$($parameters)*] $wrapper => $inner, ($receiver => $access), Push, push, TryPush, try_push);
        forward_push!([$($parameters)*] $wrapper => $inner, ($receiver => $access), PushFront, push_front, TryPushFront, try_push_front);
        forward_pop!([$($parameters)*] $wrapper => $inner, ($receiver => $access), Pop, pop);
        forward_pop!([$($parameters)*] $wrapper => $inner, ($receiver => $access), PopFront, pop_front);

        impl<$($parameters)*> Clear for $wrapper where $inner: Clear {
            fn clear(&mut self) { Clear::clear(&mut *{ let $receiver = self; $access }) }
        }

        impl<$($parameters)*, T> Reserve<T> for $wrapper where $inner: Reserve<T> {
            type Error = <$inner as Reserve<T>>::Error;
            fn reserve(&mut self, additional: usize) { Reserve::reserve(&mut *{ let $receiver = self; $access }, additional) }
            fn reserve_exact(&mut self, additional: usize) { Reserve::reserve_exact(&mut *{ let $receiver = self; $access }, additional) }
            fn try_reserve(&mut self, additional: usize) -> Result<(), Self::Error> {
                Reserve::try_reserve(&mut *{ let $receiver = self; $access }, additional)
            }
            fn try_reserve_exact(&mut self, additional: usize) -> Result<(), Self::Error> {
                Reserve::try_reserve_exact(&mut *{ let $receiver = self; $access }, additional)
            }
        }

        impl<$($parameters)*, T> Shrink<T> for $wrapper where $inner: Shrink<T> {
            fn shrink_to_fit(&mut self) { Shrink::shrink_to_fit(&mut *{ let $receiver = self; $access }) }
            fn shrink_to(&mut self, minimum: usize) { Shrink::shrink_to(&mut *{ let $receiver = self; $access }, minimum) }
        }

        impl<$($parameters)*, T> Truncate<T> for $wrapper where $inner: Truncate<T> {
            fn truncate(&mut self, len: usize) { Truncate::truncate(&mut *{ let $receiver = self; $access }, len) }
        }

        impl<$($parameters)*, T> RetainMut<T> for $wrapper where $inner: RetainMut<T> {
            fn retain_mut<Predicate: FnMut(&mut T) -> bool>(&mut self, f: Predicate) {
                RetainMut::retain_mut(&mut *{ let $receiver = self; $access }, f)
            }
        }

        impl<$($parameters)*, K, V> Insert<K, V> for $wrapper where $inner: Insert<K, V> {
            fn insert(&mut self, key: K, value: V) -> Option<V> {
                Insert::insert(&mut *{ let $receiver = self; $access }, key, value)
            }
        }

        impl<$($parameters)*, Idx> Remove<Idx> for $wrapper where $inner: Remove<Idx> {
            type Output = <$inner as Remove<Idx>>::Output;
            fn remove(&mut self, index: Idx) -> Option<Self::Output> {
                Remove::remove(&mut *{ let $receiver = self; $access }, index)
            }
        }

        impl<$($parameters)*> CollectionPushStableKey for $wrapper where $inner: CollectionPushStableKey {}
        impl<$($parameters)*> CollectionPopStableKey for $wrapper where $inner: CollectionPopStableKey {}
        impl<$($parameters)*> CollectionInsertStableKey for $wrapper where $inner: CollectionInsertStableKey {}
        impl<$($parameters)*> CollectionRemoveStableKey for $wrapper where $inner: CollectionRemoveStableKey {}
        impl<$($parameters)*> CollectionGetMutStableKey for $wrapper where $inner: CollectionGetMutStableKey {}

    };
}

macro_rules! forward_push {
    ([$($parameters:tt)*] $wrapper:ty => $inner:ty, ($receiver:ident => $access:expr), $trait:ident, $method:ident, $try_trait:ident, $try_method:ident) => {
        impl<$($parameters)*, T> $trait<T> for $wrapper where $inner: $trait<T> {
            type Output = <$inner as $trait<T>>::Output;
            fn $method(&mut self, value: T) -> Self::Output {
                $trait::$method(&mut *{ let $receiver = self; $access }, value)
            }
        }

        impl<$($parameters)*, T> $try_trait<T> for $wrapper where $inner: $try_trait<T> {
            type Error = <$inner as $try_trait<T>>::Error;
            fn $try_method(&mut self, value: T) -> Result<Self::Output, Self::Error> {
                $try_trait::$try_method(&mut *{ let $receiver = self; $access }, value)
            }
        }
    };
}

macro_rules! forward_pop {
    ([$($parameters:tt)*] $wrapper:ty => $inner:ty, ($receiver:ident => $access:expr), $trait:ident, $method:ident) => {
        impl<$($parameters)*, T> $trait<T> for $wrapper where $inner: $trait<T> {
            fn $method(&mut self) -> Option<T> {
                $trait::$method(&mut *{ let $receiver = self; $access })
            }
        }
    };
}

forward_read!([G: Deref<Target = Option<C>>, C] DoubleDerefOption<G, C> => C, Deref::deref);
forward_write!([G: DerefMut<Target = Option<C>>, C] DoubleDerefOption<G, C> => C, (guard => DerefMut::deref_mut(guard)));

#[cfg(feature = "std")]
mod guarded;
