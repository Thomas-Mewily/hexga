use super::*;

pub trait Capacity<T> : Collection<T>
{
    fn capacity(&self) -> usize;
}

pub trait WithCapacity
{
    type Param;
    fn with_capacity_and_param(capacity: usize, param: Self::Param) -> Self;
    #[inline(always)]
    fn with_capacity(capacity: usize) -> Self
    where
        Self: Sized,
        Self::Param: Default,
    {
        Self::with_capacity_and_param(capacity, ___())
    }
}

/// Trait for types that can reserve capacity for additional elements.
///
/// This trait extends [`Capacity`] and provides both fallible and infallible
/// methods for reserving space for new elements.
pub trait Reserve<T>: Capacity<T>
{
    type Error: Debug;

    /// Reserves capacity for at least `additional` elements more than the
    /// current length. The allocator may reserve more space to speculatively
    /// avoid frequent allocations. After calling `reserve`,
    /// capacity will be greater than or equal to `self.len() + additional`.
    /// Does nothing if capacity is already sufficient.
    fn reserve(&mut self, additional: usize) { let _ = self.try_reserve(additional); }
    fn try_reserve(&mut self, additional: usize) -> Result<(), Self::Error>;

    /// Reserves the minimum capacity for at least `additional` elements more than
    /// the current length. Unlike [`Self::reserve`], this will not
    /// deliberately over-allocate to speculatively avoid frequent allocations.
    /// After calling `reserve_exact`, capacity will be greater than or equal to
    /// `self.len() + additional`. Does nothing if the capacity is already
    fn reserve_exact(&mut self, additional: usize) { let _ = self.try_reserve_exact(additional); }
    fn try_reserve_exact(&mut self, additional: usize) -> Result<(), Self::Error>;

    /// Ensure total capacity is at least `total`.
    /// Does nothing if the current capacity is already >= `total`.
    fn reserve_total(&mut self, total: usize)
    where
        Self: Length,
    {
        let cap = self.capacity();

        if total <= cap
        {
            return;
        }

        let additional = total - cap;
        self.reserve(additional);
    }
    /// Ensure total capacity is at least `total`.
    /// Does nothing if the current capacity is already >= `total`.
    fn try_reserve_total(&mut self, total: usize) -> Result<(), Self::Error>
    where
        Self: Length,
    {
        let cap = self.capacity();

        if total <= cap
        {
            return Ok(());
        }

        let additional = total - cap;
        self.try_reserve(additional)
    }

    /// Ensure total capacity is at least `total`.
    /// Does nothing if the current capacity is already >= `total`.
    fn reserve_total_exact(&mut self, total: usize)
    where
        Self: Length,
    {
        let cap = self.capacity();

        if total <= cap
        {
            return;
        }

        let additional = total - cap;
        self.reserve_exact(additional);
    }
    /// Ensure total capacity is at least `total`.
    /// Does nothing if the current capacity is already >= `total`.
    fn try_reserve_total_exact(&mut self, total: usize) -> Result<(), Self::Error>
    where
        Self: Length,
    {
        let cap = self.capacity();

        if total <= cap
        {
            return Ok(());
        }

        let additional = total - cap;
        self.try_reserve_exact(additional)
    }
}

pub trait Shrink<T> : Collection<T>
{
    /// Shrinks the capacity as much as possible.
    fn shrink_to_fit(&mut self);
    /// Shrinks the capacity with a lower bound.
    /// The capacity will remain at least as large as both the length and the supplied value.
    /// If the current capacity is less than the lower limit, this is a no-op.
    fn shrink_to(&mut self, min_capacity: usize);
}

/// Shortens a collection, keeping the first `len` elements and dropping
/// the rest.
pub trait Truncate<T> : Collection<T>
{
    /// Shortens it, keeping the first `len` elements and dropping
    /// the rest.
    ///
    /// If `len` is greater or equal to the current length, this has
    /// no effect.
    fn truncate(&mut self, len: usize);
}

#[cfg(feature = "std")]
mod std_impl
{
    use super::*;

    impl<K, V, S> Capacity<V> for std::collections::HashMap<K, V, S>
    where
        K: Eq + Hash,
        S: BuildHasher,
    {
        #[inline(always)]
        fn capacity(&self) -> usize { self.capacity() }
    }
    impl<K, V, S> WithCapacity for std::collections::HashMap<K, V, S>
    where
        K: Eq + Hash,
        S: BuildHasher,
    {
        type Param = S;
        #[inline(always)]
        fn with_capacity_and_param(capacity: usize, hasher: Self::Param) -> Self { Self::with_capacity_and_hasher(capacity, hasher) }
    }
    impl<K, V, S> Reserve<V> for std::collections::HashMap<K, V, S>
    where
        K: Eq + Hash,
        S: BuildHasher,
    {
        type Error = TryReserveError;
        #[inline(always)]
        fn reserve(&mut self, additional: usize) { self.reserve(additional); }
        #[inline(always)]
        fn reserve_exact(&mut self, additional: usize) { self.reserve(additional); }

        #[inline(always)]
        fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> { self.try_reserve(additional) }
        #[inline(always)]
        fn try_reserve_exact(&mut self, additional: usize) -> Result<(), TryReserveError> { self.try_reserve(additional) }
    }
    impl<K, V, S> Shrink<V> for std::collections::HashMap<K, V, S>
    where
        K: Eq + Hash,
        S: BuildHasher,
    {
        fn shrink_to_fit(&mut self) { self.shrink_to_fit(); }
        fn shrink_to(&mut self, min_capacity: usize) { self.shrink_to(min_capacity); }
    }
}

impl<T> Capacity<T> for BinaryHeap<T>
where
    T: Ord,
{
    #[inline(always)]
    fn capacity(&self) -> usize { self.capacity() }
}
impl<T> WithCapacity for BinaryHeap<T>
where
    T: Ord,
{
    type Param = ();
    #[inline(always)]
    fn with_capacity_and_param(capacity: usize, _: Self::Param) -> Self { Self::with_capacity(capacity) }
}
impl<T> Reserve<T> for BinaryHeap<T>
where
    T: Ord,
{
    type Error = TryReserveError;
    #[inline(always)]
    fn reserve(&mut self, additional: usize) { self.reserve(additional); }
    #[inline(always)]
    fn reserve_exact(&mut self, additional: usize) { self.reserve_exact(additional); }

    #[inline(always)]
    fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> { self.try_reserve(additional) }
    #[inline(always)]
    fn try_reserve_exact(&mut self, additional: usize) -> Result<(), TryReserveError> { self.try_reserve_exact(additional) }
}
impl<T> Shrink<T> for BinaryHeap<T>
where
    T: Ord,
{
    fn shrink_to_fit(&mut self) { self.shrink_to_fit(); }
    fn shrink_to(&mut self, min_capacity: usize) { self.shrink_to(min_capacity); }
}

#[cfg(feature = "std")]
mod std_impl_2
{
    use super::*;

    impl<T, S> Capacity<T> for std::collections::HashSet<T, S>
    where
        T: Eq + Hash,
        S: BuildHasher,
    {
        #[inline(always)]
        fn capacity(&self) -> usize { self.capacity() }
    }
    impl<T, S> WithCapacity for std::collections::HashSet<T, S>
    where
        T: Eq + Hash,
        S: BuildHasher,
    {
        type Param = ();
        #[inline(always)]
        fn with_capacity_and_param(capacity: usize, _: Self::Param) -> Self { Self::with_capacity(capacity) }
    }
    impl<T, S> Reserve<T> for std::collections::HashSet<T, S>
    where
        T: Eq + Hash,
        S: BuildHasher,
    {
        type Error = TryReserveError;
        #[inline(always)]
        fn reserve(&mut self, additional: usize) { self.reserve(additional); }
        #[inline(always)]
        fn reserve_exact(&mut self, additional: usize) { self.reserve(additional); }

        #[inline(always)]
        fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> { self.try_reserve(additional) }
        #[inline(always)]
        fn try_reserve_exact(&mut self, additional: usize) -> Result<(), TryReserveError> { self.try_reserve(additional) }
    }
    impl<T, S> Shrink<T> for std::collections::HashSet<T, S>
    where
        T: Eq + Hash,
        S: BuildHasher,
    {
        fn shrink_to_fit(&mut self) { self.shrink_to_fit(); }
        fn shrink_to(&mut self, min_capacity: usize) { self.shrink_to(min_capacity); }
    }
}



impl<T> Capacity<T> for VecDeque<T>
{
    #[inline(always)]
    fn capacity(&self) -> usize { self.capacity() }
}
impl<T> WithCapacity for VecDeque<T>
{
    type Param = ();
    #[inline(always)]
    fn with_capacity_and_param(capacity: usize, _: Self::Param) -> Self { Self::with_capacity(capacity) }
}
impl<T> Reserve<T> for VecDeque<T>
{
    type Error = TryReserveError;
    #[inline(always)]
    fn reserve(&mut self, additional: usize) { self.reserve(additional); }
    #[inline(always)]
    fn reserve_exact(&mut self, additional: usize) { self.reserve_exact(additional); }

    #[inline(always)]
    fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> { self.try_reserve(additional) }
    #[inline(always)]
    fn try_reserve_exact(&mut self, additional: usize) -> Result<(), TryReserveError> { self.try_reserve_exact(additional) }
}
impl<T> Shrink<T> for VecDeque<T>
{
    fn shrink_to_fit(&mut self) { self.shrink_to_fit(); }
    fn shrink_to(&mut self, min_capacity: usize) { self.shrink_to(min_capacity); }
}
impl<T> Truncate<T> for VecDeque<T>
{
    fn truncate(&mut self, len: usize) { self.truncate(len); }
}

#[cfg(feature = "std")]
mod std_impl_3
{
    use super::*;

    impl Capacity<u8> for std::ffi::OsString
    {
        #[inline(always)]
        fn capacity(&self) -> usize { self.capacity() }
    }
    impl WithCapacity for std::ffi::OsString
    {
        type Param = ();
        #[inline(always)]
        fn with_capacity_and_param(capacity: usize, _: Self::Param) -> Self { Self::with_capacity(capacity) }
    }
    impl Reserve<u8> for std::ffi::OsString
    {
        type Error = TryReserveError;
        #[inline(always)]
        fn reserve(&mut self, additional: usize) { self.reserve(additional); }
        #[inline(always)]
        fn reserve_exact(&mut self, additional: usize) { self.reserve_exact(additional); }

        #[inline(always)]
        fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> { self.try_reserve(additional) }
        #[inline(always)]
        fn try_reserve_exact(&mut self, additional: usize) -> Result<(), TryReserveError> { self.try_reserve_exact(additional) }
    }
    impl Shrink<u8> for std::ffi::OsString
    {
        fn shrink_to_fit(&mut self) { self.shrink_to_fit(); }
        fn shrink_to(&mut self, min_capacity: usize) { self.shrink_to(min_capacity); }
    }

    impl Capacity<u8> for std::path::PathBuf
    {
        #[inline(always)]
        fn capacity(&self) -> usize { self.capacity() }
    }
    impl WithCapacity for std::path::PathBuf
    {
        type Param = ();
        #[inline(always)]
        fn with_capacity_and_param(capacity: usize, _: Self::Param) -> Self { Self::with_capacity(capacity) }
    }
    impl Reserve<u8> for std::path::PathBuf
    {
        type Error = TryReserveError;
        #[inline(always)]
        fn reserve(&mut self, additional: usize) { self.reserve(additional); }
        #[inline(always)]
        fn reserve_exact(&mut self, additional: usize) { self.reserve_exact(additional); }

        #[inline(always)]
        fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> { self.try_reserve(additional) }
        #[inline(always)]
        fn try_reserve_exact(&mut self, additional: usize) -> Result<(), TryReserveError> { self.try_reserve_exact(additional) }
    }
    impl Shrink<u8> for std::path::PathBuf
    {
        fn shrink_to_fit(&mut self) { self.shrink_to_fit(); }
        fn shrink_to(&mut self, min_capacity: usize) { self.shrink_to(min_capacity); }
    }
}



impl<T> Capacity<T> for Vec<T>
{
    #[inline(always)]
    fn capacity(&self) -> usize { self.capacity() }
}
impl<T> WithCapacity for Vec<T>
{
    type Param = ();
    #[inline(always)]
    fn with_capacity_and_param(capacity: usize, _: Self::Param) -> Self { Self::with_capacity(capacity) }
}
impl<T> Reserve<T> for Vec<T>
{
    type Error = TryReserveError;
    #[inline(always)]
    fn reserve(&mut self, additional: usize) { self.reserve(additional); }
    #[inline(always)]
    fn reserve_exact(&mut self, additional: usize) { self.reserve_exact(additional); }

    #[inline(always)]
    fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> { self.try_reserve(additional) }
    #[inline(always)]
    fn try_reserve_exact(&mut self, additional: usize) -> Result<(), TryReserveError> { self.try_reserve_exact(additional) }
}
impl<T> Shrink<T> for Vec<T>
{
    fn shrink_to_fit(&mut self) { self.shrink_to_fit(); }
    fn shrink_to(&mut self, min_capacity: usize) { self.shrink_to(min_capacity); }
}
impl<T> Truncate<T> for Vec<T>
{
    fn truncate(&mut self, len: usize) { self.truncate(len); }
}

impl Capacity<u8> for String
{
    #[inline(always)]
    fn capacity(&self) -> usize { self.capacity() }
}
impl WithCapacity for String
{
    type Param = ();
    #[inline(always)]
    fn with_capacity_and_param(capacity: usize, _: Self::Param) -> Self { Self::with_capacity(capacity) }
}
impl Reserve<u8> for String
{
    type Error = TryReserveError;
    #[inline(always)]
    fn reserve(&mut self, additional: usize) { self.reserve(additional); }
    #[inline(always)]
    fn reserve_exact(&mut self, additional: usize) { self.reserve_exact(additional); }

    #[inline(always)]
    fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> { self.try_reserve(additional) }
    #[inline(always)]
    fn try_reserve_exact(&mut self, additional: usize) -> Result<(), TryReserveError> { self.try_reserve_exact(additional) }
}
impl Shrink<u8> for String
{
    fn shrink_to_fit(&mut self) { self.shrink_to_fit(); }
    fn shrink_to(&mut self, min_capacity: usize) { self.shrink_to(min_capacity); }
}
impl Truncate<u8> for String
{
    fn truncate(&mut self, len: usize) { self.truncate(len); }
}


impl<T> Capacity<T> for Option<T> {
    #[inline(always)]
    fn capacity(&self) -> usize { self.is_none() as usize }
}
impl<T> WithCapacity for Option<T> {
    type Param = ();
    #[inline(always)]
    fn with_capacity_and_param(_capacity: usize, _: Self::Param) -> Self { None }
}
impl<T,E> Capacity<T> for Result<T,E> {
    #[inline(always)]
    fn capacity(&self) -> usize { self.is_ok() as usize }
}
impl<T,E> WithCapacity for Result<T,E> 
    where E: Default
{
    type Param = ();
    #[inline(always)]
    fn with_capacity_and_param(_capacity: usize, _: Self::Param) -> Self { Err(___()) }
}
