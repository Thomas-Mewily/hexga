use super::*;



pub trait Capacity
{
    type Param;
    fn capacity(&self) -> usize;

    fn with_capacity_and_param(capacity: usize, param : Self::Param) -> Self;
    #[inline(always)]
    fn with_capacity(capacity: usize) -> Self where Self : Sized, Self::Param : Default { Self::with_capacity_and_param(capacity, ___()) }

    fn reserve(&mut self, additional: usize);
    fn reserve_exact(&mut self, additional: usize);

    fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError>;
    fn try_reserve_exact(&mut self, additional: usize) -> Result<(), TryReserveError>;
}

impl<K, V, S> Capacity for HashMap<K, V, S>
where
    K: Eq + Hash,
    S: BuildHasher,
{
    type Param = S;
    #[inline(always)]
    fn capacity(&self) -> usize { self.capacity() }
    #[inline(always)]
    fn with_capacity_and_param(capacity: usize, hasher : Self::Param) -> Self { Self::with_capacity_and_hasher(capacity, hasher) }

    #[inline(always)]
    fn reserve(&mut self, additional: usize) { self.reserve(additional); }
    #[inline(always)]
    fn reserve_exact(&mut self, additional: usize) { self.reserve(additional); }

    #[inline(always)]
    fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> { self.try_reserve(additional) }
    #[inline(always)]
    fn try_reserve_exact(&mut self, additional: usize) -> Result<(), TryReserveError> { self.try_reserve(additional) }
}

impl<T> Capacity for std::collections::BinaryHeap<T>
where
    T: Ord,
{
    type Param = ();
    #[inline(always)]
    fn capacity(&self) -> usize { self.capacity() }

    #[inline(always)]
    fn with_capacity_and_param(capacity: usize, _ : Self::Param) -> Self { Self::with_capacity(capacity) }

    #[inline(always)]
    fn reserve(&mut self, additional: usize) { self.reserve(additional); }
    #[inline(always)]
    fn reserve_exact(&mut self, additional: usize) { self.reserve_exact(additional); }

    #[inline(always)]
    fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> { self.try_reserve(additional) }
    #[inline(always)]
    fn try_reserve_exact(&mut self, additional: usize) -> Result<(), TryReserveError> { self.try_reserve_exact(additional) }
}

impl<T,S> Capacity for std::collections::HashSet<T, S>
where
    T: Eq + Hash,
    S: BuildHasher
{
    type Param = ();
    #[inline(always)]
    fn capacity(&self) -> usize { self.capacity() }

    #[inline(always)]
    fn with_capacity_and_param(capacity: usize, _ : Self::Param) -> Self { Self::with_capacity(capacity) }

    #[inline(always)]
    fn reserve(&mut self, additional: usize) { self.reserve(additional); }
    #[inline(always)]
    fn reserve_exact(&mut self, additional: usize) { self.reserve(additional); }

    #[inline(always)]
    fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> { self.try_reserve(additional) }
    #[inline(always)]
    fn try_reserve_exact(&mut self, additional: usize) -> Result<(), TryReserveError> { self.try_reserve(additional) }
}

impl<T> Capacity for std::collections::VecDeque<T>
{
    type Param = ();
    #[inline(always)]
    fn capacity(&self) -> usize { self.capacity() }

    #[inline(always)]
    fn with_capacity_and_param(capacity: usize, _ : Self::Param) -> Self { Self::with_capacity(capacity) }

    #[inline(always)]
    fn reserve(&mut self, additional: usize) { self.reserve(additional); }
    #[inline(always)]
    fn reserve_exact(&mut self, additional: usize) { self.reserve_exact(additional); }

    #[inline(always)]
    fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> { self.try_reserve(additional) }
    #[inline(always)]
    fn try_reserve_exact(&mut self, additional: usize) -> Result<(), TryReserveError> { self.try_reserve_exact(additional) }
}

impl Capacity for std::ffi::OsString
{
    type Param = ();
    #[inline(always)]
    fn capacity(&self) -> usize { self.capacity() }

    #[inline(always)]
    fn with_capacity_and_param(capacity: usize, _ : Self::Param) -> Self { Self::with_capacity(capacity) }

    #[inline(always)]
    fn reserve(&mut self, additional: usize) { self.reserve(additional); }
    #[inline(always)]
    fn reserve_exact(&mut self, additional: usize) { self.reserve_exact(additional); }

    #[inline(always)]
    fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> { self.try_reserve(additional) }
    #[inline(always)]
    fn try_reserve_exact(&mut self, additional: usize) -> Result<(), TryReserveError> { self.try_reserve_exact(additional) }
}

/*
impl<R: std::io::Read> Capacity for std::io::BufReader<R> {}
impl<W: std::io::Write> Capacity for std::io::BufWriter<W> {}
impl<W: std::io::Write> Capacity for std::io::LineWriter<W> {}
*/

impl Capacity for std::path::PathBuf
{
    type Param = ();
    #[inline(always)]
    fn capacity(&self) -> usize { self.capacity() }

    #[inline(always)]
    fn with_capacity_and_param(capacity: usize, _ : Self::Param) -> Self { Self::with_capacity(capacity) }

    #[inline(always)]
    fn reserve(&mut self, additional: usize) { self.reserve(additional); }
    #[inline(always)]
    fn reserve_exact(&mut self, additional: usize) { self.reserve_exact(additional); }

    #[inline(always)]
    fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> { self.try_reserve(additional) }
    #[inline(always)]
    fn try_reserve_exact(&mut self, additional: usize) -> Result<(), TryReserveError> { self.try_reserve_exact(additional) }
}

impl<T> Capacity for Vec<T>
{
    type Param = ();
    #[inline(always)]
    fn capacity(&self) -> usize { self.capacity() }

    #[inline(always)]
    fn with_capacity_and_param(capacity: usize, _ : Self::Param) -> Self { Self::with_capacity(capacity) }

    #[inline(always)]
    fn reserve(&mut self, additional: usize) { self.reserve(additional); }
    #[inline(always)]
    fn reserve_exact(&mut self, additional: usize) { self.reserve_exact(additional); }

    #[inline(always)]
    fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> { self.try_reserve(additional) }
    #[inline(always)]
    fn try_reserve_exact(&mut self, additional: usize) -> Result<(), TryReserveError> { self.try_reserve_exact(additional) }
}


impl Capacity for String
{
    type Param = ();
    #[inline(always)]
    fn capacity(&self) -> usize { self.capacity() }

    #[inline(always)]
    fn with_capacity_and_param(capacity: usize, _ : Self::Param) -> Self { Self::with_capacity(capacity) }

    #[inline(always)]
    fn reserve(&mut self, additional: usize) { self.reserve(additional); }
    #[inline(always)]
    fn reserve_exact(&mut self, additional: usize) { self.reserve_exact(additional); }

    #[inline(always)]
    fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> { self.try_reserve(additional) }
    #[inline(always)]
    fn try_reserve_exact(&mut self, additional: usize) -> Result<(), TryReserveError> { self.try_reserve_exact(additional) }
}