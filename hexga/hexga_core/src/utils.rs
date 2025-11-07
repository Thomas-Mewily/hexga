//! Stuff inside this module need to move somewhere else...
use crate::prelude::*;


// Waiting for the never type `!` to be stabilized
pub enum Never{}

pub trait Toggleable
{
    fn toggle(&mut self);
}
impl Toggleable for bool
{
    #[inline(always)]
    fn toggle(&mut self)
    {
        use std::ops::Not;
        *self = self.not();
    }
}


pub trait Once
{
    //Excute the lambda one time, and modify the caller to indicate t
    fn once<R, F>(&mut self, f : F) -> Option<R> where F : FnOnce() -> R;
}
impl Once for bool
{
    fn once<R, F>(&mut self, f: F) -> Option<R> where F: FnOnce() -> R
    {
        if !*self
        {
            *self = true;
            Some(f())
        }else { None }
    }
}

/// Useful to silence/convert to void some Err.
///
/// Some of my lib will probably have proper error type instead of () (Look for `#proper_error` to know which error type are temporary) when I will have time to add them
pub trait OkOr<T,E>
{
    fn ok_or<E2>(self, err: E2) -> Result<T,E2>;
    fn ok_or_void(self) -> Result<T,()>;
}

impl<T,E> OkOr<T,E> for Result<T,E>
{
    #[inline(always)]
    fn ok_or<E2>(self, err: E2) -> Result<T,E2> { self.map_err(|_| err) }

    #[inline(always)]
    fn ok_or_void(self) -> Result<T,()> {
        self.ok_or(())
    }
}
impl<T> OkOr<T,()> for Option<T>
{
    #[inline(always)]
    fn ok_or_void(self) -> Result<T,()> {
        self.ok_or(())
    }

    fn ok_or<E>(self, err: E) -> Result<T,E> {
        self.ok_or(err)
    }
}


pub trait ResultInto<T,E> : OkOr<T,E>
{
    fn ok_into<O>(self) -> Result<O,E> where T: Into<O>;
    fn err_into<E2>(self) -> Result<T,E2> where E: Into<E2>;
}

impl<T,E> ResultInto<T,E> for Result<T,E>
{
    fn ok_into<O>(self) -> Result<O,E> where T: Into<O> {
        self.map(Into::into)
    }

    fn err_into<E2>(self) -> Result<T,E2> where E: Into<E2> {
        self.map_err(Into::into)
    }
}

/*
// Eq is imply by Ord, but I prefer to make sure this is visible
/// A key that can be used in an HashMap (Hash + Eq), but also in a BTreeMap (Ord + Eq)
pub trait UniversalKey : Hash + Eq + Ord {}
impl<T> UniversalKey for T where T: Hash + Eq + Ord {}
*/




/*
pub trait IteratorExtension<T> where T: PartialEq
{
    fn contains(self, value : &T) -> bool;
}
impl<I,T> IteratorExtension<T> for I where I : Iterator<Item = T>, T : PartialEq
{
    fn contains(mut self, value : &T) -> bool { self.position(|v| &v == value).is_some() }
}
*/


pub trait ResultDebugExtension<T>
{
    fn ok_or_debug(self) -> Result<T,String>;
}
impl<T,E> ResultDebugExtension<T> for Result<T,E> where E:ToDebug
{
    fn ok_or_debug(self) -> Result<T,String> {
        self.map_err(|e| e.to_debug())
    }
}



pub trait ToFmtWriter : std::io::Write + Sized
{
    fn to_fmt_writer(self) -> IoWriteAdapter<Self>;
}
impl<T> ToFmtWriter for T where T: std::io::Write
{
    fn to_fmt_writer(self) -> IoWriteAdapter<Self> {
        IoWriteAdapter{ writer: self }
    }
}


#[doc(hidden)]
pub struct IoWriteAdapter<W: std::io::Write>
{
    pub writer: W
}
impl<W: std::io::Write> std::fmt::Write for IoWriteAdapter<W>
{
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.writer.write_all(s.as_bytes()).map_err(|_| std::fmt::Error)
    }
}