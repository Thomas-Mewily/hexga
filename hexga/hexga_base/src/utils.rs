
#[allow(unused_imports)]
#[cfg(feature = "serde")]
pub(crate) use serde::{Serialize, Serializer, Deserialize, Deserializer, de::Visitor, ser::SerializeStruct};

pub use rayon::prelude::*;

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
    fn to_debug(&self) -> String {
        format!("{:?}", self)
    }
}

