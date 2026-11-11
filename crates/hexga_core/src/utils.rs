//! Stuff inside this module need to move somewhere else...
use core::marker::PhantomData;

use crate::prelude::*;

// Waiting for the never type `!` to be stabilized
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Never {}

pub trait Toggleable
{
    fn toggle(&mut self);
}
impl Toggleable for bool
{
    #[inline(always)]
    fn toggle(&mut self)
    {
        use core::ops::Not;
        *self = self.not();
    }
}

pub trait DoOnce
{
    //Excute the lambda one time, and modify the caller to indicate t
    fn once<R, F>(&mut self, f: F) -> Option<R>
    where
        F: FnOnce() -> R;
}
impl DoOnce for bool
{
    fn once<R, F>(&mut self, f: F) -> Option<R>
    where
        F: FnOnce() -> R,
    {
        if !*self
        {
            *self = true;
            Some(f())
        }
        else
        {
            None
        }
    }
}

/// Useful to silence/convert to void some Err.
///
/// Some of my lib will probably have proper error type instead of () (Look for `#proper_error` to know which error type are temporary) when I will have time to add them
pub trait OkOr<T, E>
{
    fn ok_or<E2>(self, err: E2) -> Result<T, E2>;
    fn ok_or_void(self) -> Result<T, ()>;
}

impl<T, E> OkOr<T, E> for Result<T, E>
{
    #[inline(always)]
    fn ok_or<E2>(self, err: E2) -> Result<T, E2> { self.map_err(|_| err) }

    #[inline(always)]
    fn ok_or_void(self) -> Result<T, ()> { self.ok_or(()) }
}
impl<T> OkOr<T, ()> for Option<T>
{
    #[inline(always)]
    fn ok_or_void(self) -> Result<T, ()> { self.ok_or(()) }

    fn ok_or<E>(self, err: E) -> Result<T, E> { self.ok_or(err) }
}

pub trait ResultInto<T, E>: OkOr<T, E>
{
    fn ok_into<O>(self) -> Result<O, E>
    where
        T: Into<O>;
    fn err_into<E2>(self) -> Result<T, E2>
    where
        E: Into<E2>;
}

impl<T, E> ResultInto<T, E> for Result<T, E>
{
    fn ok_into<O>(self) -> Result<O, E>
    where
        T: Into<O>,
    {
        self.map(Into::into)
    }

    fn err_into<E2>(self) -> Result<T, E2>
    where
        E: Into<E2>,
    {
        self.map_err(Into::into)
    }
}

/*
// Eq is imply by Ord, but I prefer to make sure this is visible
/// A key that can be used in an HashMap (Hash + Eq), but also in a BTreeMap (Ord + Eq)
pub trait UniversalKey : Hash + Eq + Ord {}
impl<T> UniversalKey for T where T: Hash + Eq + Ord {}
*/

pub trait ResultDebugExtension<T>
{
    fn ok_or_debug(self) -> Result<T, String>;
}
impl<T, E> ResultDebugExtension<T> for Result<T, E>
where
    E: ToDebug,
{
    fn ok_or_debug(self) -> Result<T, String> { self.map_err(|e| e.to_debug()) }
}

#[cfg(feature = "std")]
pub trait ToFmtWriter: std::io::Write + Sized
{
    fn to_fmt_writer(self) -> IoWriteAdapter<Self>;
}
#[cfg(feature = "std")]
impl<T> ToFmtWriter for T
where
    T: std::io::Write,
{
    fn to_fmt_writer(self) -> IoWriteAdapter<Self> { IoWriteAdapter { writer: self } }
}

#[cfg(feature = "std")]
#[doc(hidden)]
pub struct IoWriteAdapter<W: std::io::Write>
{
    pub writer: W,
}
#[cfg(feature = "std")]
impl<W: std::io::Write> std::fmt::Write for IoWriteAdapter<W>
{
    fn write_str(&mut self, s: &crate::primitive::str) -> std::fmt::Result { self.writer.write_all(s.as_bytes()).map_err(|_| std::fmt::Error) }
}

pub mod prelude
{
    pub use super::Never;
    pub use super::traits::*;
}
pub mod traits
{
    pub use super::{DoOnce, OkOr, ResultDebugExtension, ResultInto, Toggleable};
}



#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DoubleDeref<G> { pub guard: G }

impl<G> From<G> for DoubleDeref<G> { fn from(guard: G) -> Self { Self { guard }}}
impl<G> Deref for DoubleDeref<G> where G: Deref, G::Target: Deref
{
    type Target=<G::Target as Deref>::Target;
    fn deref(&self) -> &Self::Target { self.guard.deref().deref() }
}
impl<G> DerefMut for DoubleDeref<G> where G: DerefMut, G::Target: DerefMut
{
    fn deref_mut(&mut self) -> &mut Self::Target { self.guard.deref_mut().deref_mut() }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DoubleDerefOption<G,T> { pub guard: G, phantom: PhantomData<T> }

impl<G,T> From<G> for DoubleDerefOption<G,T> { fn from(guard: G) -> Self { Self { guard, phantom: PhantomData }}}
impl<G,T> Deref for DoubleDerefOption<G,T> where G: Deref<Target = Option<T>>
{
    type Target=T;
    fn deref(&self) -> &Self::Target { self.guard.deref().as_ref().unwrap() }
}
impl<G,T> DerefMut for DoubleDerefOption<G,T> where G: DerefMut<Target=Option<T>>
{
    fn deref_mut(&mut self) -> &mut Self::Target { self.guard.deref_mut().as_mut().unwrap() }
}
