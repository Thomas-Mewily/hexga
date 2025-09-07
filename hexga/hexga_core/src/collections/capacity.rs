use std::collections::TryReserveError;
use super::*;


pub mod prelude
{
    pub use super::Capacity;
}


pub trait Capacity
{
    type Param;
    fn capacity(&self) -> usize;

    fn with_capacity_and_param(capacity: usize, param : Self::Param) -> Self;
    #[inline(always)]
    fn with_capacity(capacity: usize) -> Self where Self : Sized, Self::Param : Default { Self::with_capacity_and_param(capacity, ___()) }

    fn reserve(&mut self, additional: usize);
    fn reserve_exact(&mut self, additional: usize); // { self.reserve(additional); }

    fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError>;
    fn try_reserve_exact(&mut self, additional: usize) -> Result<(), TryReserveError>; // { self.try_reserve(additional) }
}

impl<K, V> Capacity for HashMap<K, V, std::hash::RandomState>
where
    K: Eq + Hash,
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

impl<T> Capacity for std::collections::HashSet<T, std::hash::RandomState>
where
    T: Eq + Hash,
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

// Discutable
/* 
impl<'a, T> Capacity for &'a [T]
{
    type Param = &'a Vec<T>;
    #[inline(always)]
    fn capacity(&self) -> usize { self.len() }

    #[inline(always)]
    fn with_capacity_and_param(_: usize, inside : Self::Param) -> Self { inside }

    #[inline(always)]
    fn reserve(&mut self, _: usize) {}
    #[inline(always)]
    fn reserve_exact(&mut self, _: usize) {}

    #[inline(always)]
    fn try_reserve(&mut self, _: usize) -> Result<(), TryReserveError> { Ok(()) }
    #[inline(always)]
    fn try_reserve_exact(&mut self, _: usize) -> Result<(), TryReserveError> { Ok(()) }
}
impl<'a, T> Capacity for &'a mut [T]
impl<'a, T> Capacity for &'a str
impl<'a, T> Capacity for &'a mut str
...
*/

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