#![allow(unused_imports)]
#![allow(unused_macros)]

pub(crate) use std::ops::*;
pub(crate) use std::fmt::{Debug, Display, Formatter, Result as DResult};
pub(crate) use hexga_number::*;
pub(crate) use std::marker::PhantomData;

#[cfg(feature = "serde")]
pub(crate) use serde::{Serialize, Serializer, Deserialize, Deserializer, de::Visitor, ser::SerializeStruct};

mod non_empty_stack;
pub use non_empty_stack::*;

/// While waiting for the std:never type to stabilize
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Never{}

pub trait Toggleable
{
    fn toggle(&mut self);
}
impl Toggleable for bool
{
    fn toggle(&mut self) 
    {
        *self = self.not();
    }
}

pub trait ToDebug
{
    fn to_debug(&self) -> String;
}
impl<T> ToDebug for T where T : Debug
{
    fn to_debug(&self) -> String {
        format!("{:?}", self)
    }
}