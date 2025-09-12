pub use std::fmt::{format,Formatter,Debug,Display};
pub type FmtResult = std::fmt::Result;

pub trait ToDebug
{
    fn to_debug(&self) -> String;
}
impl<T> ToDebug for T where T: std::fmt::Debug
{
    #[inline(always)]
    fn to_debug(&self) -> String {
        format!("{:?}", self)
    }
}