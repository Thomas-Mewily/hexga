use super::*;

pub trait Capacity: Collection
{
    fn capacity(&self) -> usize;
}

pub trait WithCapacity: Capacity + Reserve
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

pub trait Reserve: Capacity
{
    fn reserve(&mut self, additional: usize);
    fn reserve_exact(&mut self, additional: usize);

    fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError>;
    fn try_reserve_exact(&mut self, additional: usize) -> Result<(), TryReserveError>;

    /// Ensure total capacity is at least `total`.
    /// Does nothing if the current capacity is already >= `total`.
    fn reserve_total(&mut self, total: usize) where Self: Length
    {
        let cap = self.capacity();

        if total <= cap { return; }

        let additional = total - cap;
        self.reserve(additional);
    }
    /// Ensure total capacity is at least `total`.
    /// Does nothing if the current capacity is already >= `total`.
    fn try_reserve_total(&mut self, total: usize) -> Result<(), TryReserveError> where Self: Length
    {
        let cap = self.capacity();

        if total <= cap { return Ok(()); }

        let additional = total - cap;
        self.try_reserve(additional)
    }

    /// Ensure total capacity is at least `total`.
    /// Does nothing if the current capacity is already >= `total`.
    fn reserve_total_exact(&mut self, total: usize) where Self: Length
    {
        let cap = self.capacity();

        if total <= cap { return; }

        let additional = total - cap;
        self.reserve_exact(additional);
    }
    /// Ensure total capacity is at least `total`.
    /// Does nothing if the current capacity is already >= `total`.
    fn try_reserve_total_exact(&mut self, total: usize) -> Result<(), TryReserveError> where Self: Length
    {
        let cap = self.capacity();

        if total <= cap { return Ok(()); }

        let additional = total - cap;
        self.try_reserve_exact(additional)
    }
}

pub trait Shrink: Collection
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
pub trait Truncate: Collection
{
    /// Shortens it, keeping the first `len` elements and dropping
    /// the rest.
    ///
    /// If `len` is greater or equal to the current length, this has
    /// no effect.
    fn truncate(&mut self, len: usize);
}

#[cfg(feature = "std")]
impl<K, V, S> Capacity for std::collections::HashMap<K, V, S>
where
    K: Eq + Hash,
    S: BuildHasher,
{
    #[inline(always)]
    fn capacity(&self) -> usize { self.capacity() }
}
#[cfg(feature = "std")]
impl<K, V, S> WithCapacity for std::collections::HashMap<K, V, S>
where
    K: Eq + Hash,
    S: BuildHasher,
{
    type Param = S;
    #[inline(always)]
    fn with_capacity_and_param(capacity: usize, hasher: Self::Param) -> Self
    {
        Self::with_capacity_and_hasher(capacity, hasher)
    }
}
#[cfg(feature = "std")]
impl<K, V, S> Reserve for std::collections::HashMap<K, V, S>
where
    K: Eq + Hash,
    S: BuildHasher,
{
    #[inline(always)]
    fn reserve(&mut self, additional: usize) { self.reserve(additional); }
    #[inline(always)]
    fn reserve_exact(&mut self, additional: usize) { self.reserve(additional); }

    #[inline(always)]
    fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError>
    {
        self.try_reserve(additional)
    }
    #[inline(always)]
    fn try_reserve_exact(&mut self, additional: usize) -> Result<(), TryReserveError>
    {
        self.try_reserve(additional)
    }
}
#[cfg(feature = "std")]
impl<K, V, S> Shrink for std::collections::HashMap<K, V, S>
where
    K: Eq + Hash,
    S: BuildHasher,
{
    fn shrink_to_fit(&mut self) { self.shrink_to_fit(); }
    fn shrink_to(&mut self, min_capacity: usize) { self.shrink_to(min_capacity); }
}

impl<T> Capacity for BinaryHeap<T>
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
    fn with_capacity_and_param(capacity: usize, _: Self::Param) -> Self
    {
        Self::with_capacity(capacity)
    }
}
impl<T> Reserve for BinaryHeap<T>
where
    T: Ord,
{
    #[inline(always)]
    fn reserve(&mut self, additional: usize) { self.reserve(additional); }
    #[inline(always)]
    fn reserve_exact(&mut self, additional: usize) { self.reserve_exact(additional); }

    #[inline(always)]
    fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError>
    {
        self.try_reserve(additional)
    }
    #[inline(always)]
    fn try_reserve_exact(&mut self, additional: usize) -> Result<(), TryReserveError>
    {
        self.try_reserve_exact(additional)
    }
}
impl<T> Shrink for BinaryHeap<T>
where
    T: Ord,
{
    fn shrink_to_fit(&mut self) { self.shrink_to_fit(); }
    fn shrink_to(&mut self, min_capacity: usize) { self.shrink_to(min_capacity); }
}

#[cfg(feature = "std")]
impl<T, S> Capacity for std::collections::HashSet<T, S>
where
    T: Eq + Hash,
    S: BuildHasher,
{
    #[inline(always)]
    fn capacity(&self) -> usize { self.capacity() }
}
#[cfg(feature = "std")]
impl<T, S> WithCapacity for std::collections::HashSet<T, S>
where
    T: Eq + Hash,
    S: BuildHasher,
{
    type Param = ();
    #[inline(always)]
    fn with_capacity_and_param(capacity: usize, _: Self::Param) -> Self
    {
        Self::with_capacity(capacity)
    }
}
#[cfg(feature = "std")]
impl<T, S> Reserve for std::collections::HashSet<T, S>
where
    T: Eq + Hash,
    S: BuildHasher,
{
    #[inline(always)]
    fn reserve(&mut self, additional: usize) { self.reserve(additional); }
    #[inline(always)]
    fn reserve_exact(&mut self, additional: usize) { self.reserve(additional); }

    #[inline(always)]
    fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError>
    {
        self.try_reserve(additional)
    }
    #[inline(always)]
    fn try_reserve_exact(&mut self, additional: usize) -> Result<(), TryReserveError>
    {
        self.try_reserve(additional)
    }
}
#[cfg(feature = "std")]
impl<T, S> Shrink for std::collections::HashSet<T, S>
where
    T: Eq + Hash,
    S: BuildHasher,
{
    fn shrink_to_fit(&mut self) { self.shrink_to_fit(); }
    fn shrink_to(&mut self, min_capacity: usize) { self.shrink_to(min_capacity); }
}

impl<T> Capacity for VecDeque<T>
{
    #[inline(always)]
    fn capacity(&self) -> usize { self.capacity() }
}
impl<T> WithCapacity for VecDeque<T>
{
    type Param = ();
    #[inline(always)]
    fn with_capacity_and_param(capacity: usize, _: Self::Param) -> Self
    {
        Self::with_capacity(capacity)
    }
}
impl<T> Reserve for VecDeque<T>
{
    #[inline(always)]
    fn reserve(&mut self, additional: usize) { self.reserve(additional); }
    #[inline(always)]
    fn reserve_exact(&mut self, additional: usize) { self.reserve_exact(additional); }

    #[inline(always)]
    fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError>
    {
        self.try_reserve(additional)
    }
    #[inline(always)]
    fn try_reserve_exact(&mut self, additional: usize) -> Result<(), TryReserveError>
    {
        self.try_reserve_exact(additional)
    }
}
impl<T> Shrink for VecDeque<T>
{
    fn shrink_to_fit(&mut self) { self.shrink_to_fit(); }
    fn shrink_to(&mut self, min_capacity: usize) { self.shrink_to(min_capacity); }
}
impl<T> Truncate for VecDeque<T>
{
    fn truncate(&mut self, len: usize) { self.truncate(len); }
}

#[cfg(feature = "std")]
impl Capacity for std::ffi::OsString
{
    #[inline(always)]
    fn capacity(&self) -> usize { self.capacity() }
}
#[cfg(feature = "std")]
impl WithCapacity for std::ffi::OsString
{
    type Param = ();
    #[inline(always)]
    fn with_capacity_and_param(capacity: usize, _: Self::Param) -> Self
    {
        Self::with_capacity(capacity)
    }
}
#[cfg(feature = "std")]
impl Reserve for std::ffi::OsString
{
    #[inline(always)]
    fn reserve(&mut self, additional: usize) { self.reserve(additional); }
    #[inline(always)]
    fn reserve_exact(&mut self, additional: usize) { self.reserve_exact(additional); }

    #[inline(always)]
    fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError>
    {
        self.try_reserve(additional)
    }
    #[inline(always)]
    fn try_reserve_exact(&mut self, additional: usize) -> Result<(), TryReserveError>
    {
        self.try_reserve_exact(additional)
    }
}
#[cfg(feature = "std")]
impl Shrink for std::ffi::OsString
{
    fn shrink_to_fit(&mut self) { self.shrink_to_fit(); }
    fn shrink_to(&mut self, min_capacity: usize) { self.shrink_to(min_capacity); }
}

#[cfg(feature = "std")]
impl Capacity for std::path::PathBuf
{
    #[inline(always)]
    fn capacity(&self) -> usize { self.capacity() }
}
#[cfg(feature = "std")]
impl WithCapacity for std::path::PathBuf
{
    type Param = ();
    #[inline(always)]
    fn with_capacity_and_param(capacity: usize, _: Self::Param) -> Self
    {
        Self::with_capacity(capacity)
    }
}
#[cfg(feature = "std")]
impl Reserve for std::path::PathBuf
{
    #[inline(always)]
    fn reserve(&mut self, additional: usize) { self.reserve(additional); }
    #[inline(always)]
    fn reserve_exact(&mut self, additional: usize) { self.reserve_exact(additional); }

    #[inline(always)]
    fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError>
    {
        self.try_reserve(additional)
    }
    #[inline(always)]
    fn try_reserve_exact(&mut self, additional: usize) -> Result<(), TryReserveError>
    {
        self.try_reserve_exact(additional)
    }
}
#[cfg(feature = "std")]
impl Shrink for std::path::PathBuf
{
    fn shrink_to_fit(&mut self) { self.shrink_to_fit(); }
    fn shrink_to(&mut self, min_capacity: usize) { self.shrink_to(min_capacity); }
}

impl<T> Capacity for Vec<T>
{
    #[inline(always)]
    fn capacity(&self) -> usize { self.capacity() }
}
impl<T> WithCapacity for Vec<T>
{
    type Param = ();
    #[inline(always)]
    fn with_capacity_and_param(capacity: usize, _: Self::Param) -> Self
    {
        Self::with_capacity(capacity)
    }
}
impl<T> Reserve for Vec<T>
{
    #[inline(always)]
    fn reserve(&mut self, additional: usize) { self.reserve(additional); }
    #[inline(always)]
    fn reserve_exact(&mut self, additional: usize) { self.reserve_exact(additional); }

    #[inline(always)]
    fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError>
    {
        self.try_reserve(additional)
    }
    #[inline(always)]
    fn try_reserve_exact(&mut self, additional: usize) -> Result<(), TryReserveError>
    {
        self.try_reserve_exact(additional)
    }
}
impl<T> Shrink for Vec<T>
{
    fn shrink_to_fit(&mut self) { self.shrink_to_fit(); }
    fn shrink_to(&mut self, min_capacity: usize) { self.shrink_to(min_capacity); }
}
impl<T> Truncate for Vec<T>
{
    fn truncate(&mut self, len: usize) { self.truncate(len); }
}

impl Capacity for String
{
    #[inline(always)]
    fn capacity(&self) -> usize { self.capacity() }
}
impl WithCapacity for String
{
    type Param = ();
    #[inline(always)]
    fn with_capacity_and_param(capacity: usize, _: Self::Param) -> Self
    {
        Self::with_capacity(capacity)
    }
}
impl Reserve for String
{
    #[inline(always)]
    fn reserve(&mut self, additional: usize) { self.reserve(additional); }
    #[inline(always)]
    fn reserve_exact(&mut self, additional: usize) { self.reserve_exact(additional); }

    #[inline(always)]
    fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError>
    {
        self.try_reserve(additional)
    }
    #[inline(always)]
    fn try_reserve_exact(&mut self, additional: usize) -> Result<(), TryReserveError>
    {
        self.try_reserve_exact(additional)
    }
}
impl Shrink for String
{
    fn shrink_to_fit(&mut self) { self.shrink_to_fit(); }
    fn shrink_to(&mut self, min_capacity: usize) { self.shrink_to(min_capacity); }
}
impl Truncate for String
{
    fn truncate(&mut self, len: usize) { self.truncate(len); }
}
