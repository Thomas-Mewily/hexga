
#[allow(unused_imports)]
#[cfg(feature = "serde")]
pub(crate) use serde::{Serialize, Serializer, Deserialize, Deserializer, de::Visitor, ser::SerializeStruct};

pub use rayon::prelude::*;
pub use default_is_triple_underscore::*;

/// While waiting for the std:never type to stabilize
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
        use std::ops::Not;
        *self = self.not();
    }
}

pub trait ToDebug
{
    fn to_debug(&self) -> String;
}
impl<T> ToDebug for T where T : std::fmt::Debug
{
    #[inline(always)]
    fn to_debug(&self) -> String {
        format!("{:?}", self)
    }
}

/// Useful to silence/convert to void some Err.
/// 
/// Some of my lib will probably have proper error type instead of () when I will have time to add them
pub trait ResultExtension<T>
{
    fn ok_or_void(self) -> Result<T,()>;
}
impl<T,E> ResultExtension<T> for Result<T,E>
{
    #[inline(always)]
    fn ok_or_void(self) -> Result<T,()> {
        self.map_err(|_| ())
    }
}
impl<T> ResultExtension<T> for Option<T>
{
    #[inline(always)]
    fn ok_or_void(self) -> Result<T,()> {
        self.ok_or(())
    }
}


/*
// Eq is imply by Ord, but I prefer to make sure this is visible
/// A key that can be used in an HashMap (Hash + Eq), but also in a BTreeMap (Ord + Eq)
pub trait UniversalKey : Hash + Eq + Ord {}
impl<T> UniversalKey for T where T: Hash + Eq + Ord {}
*/