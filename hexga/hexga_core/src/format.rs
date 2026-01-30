use super::*;

pub use std::fmt::{Debug, Display, Formatter, format};
pub type FmtResult = std::fmt::Result;

pub trait ToDebug
{
    fn to_debug(&self) -> String;
}
impl<T> ToDebug for T
where
    T: std::fmt::Debug,
{
    #[inline(always)]
    fn to_debug(&self) -> String { format!("{:?}", self) }
}

pub trait DebugExtension
{
    fn field_if<T: Debug>(
        &mut self,
        name: &str,
        value: &T,
        pred: impl FnOnce(&T) -> bool,
    ) -> &mut Self;
    fn field_if_not_default<T: Debug + Default + PartialEq>(
        &mut self,
        name: &str,
        value: &T,
    ) -> &mut Self
    {
        self.field_if(name, value, |v| v.is_not_default())
    }

    fn field<T: Debug>(&mut self, name: &str, value: &T) -> &mut Self;

    fn field_if_false(&mut self, name: &str, value: bool) -> &mut Self
    {
        if !value
        {
            self.field(name, &value);
        }
        self
    }
    fn field_if_true(&mut self, name: &str, value: bool) -> &mut Self
    {
        if value
        {
            self.field(name, &value);
        }
        self
    }

    fn field_bool(&mut self, value: bool, name_true: &str, name_false: &str) -> &mut Self;
}

impl<'a, 'b: 'a> DebugExtension for std::fmt::DebugStruct<'a, 'b>
{
    fn field_if<T: Debug>(
        &mut self,
        name: &str,
        value: &T,
        pred: impl FnOnce(&T) -> bool,
    ) -> &mut Self
    {
        if pred(value)
        {
            self.field(name, value);
        }
        self
    }

    fn field<T: Debug>(&mut self, name: &str, value: &T) -> &mut Self { self.field(name, value) }

    fn field_bool(&mut self, value: bool, name_true: &str, name_false: &str) -> &mut Self
    {
        self.field(if value { name_true } else { name_false }, &())
    }
}

pub struct FmtOptional<'a, T>
{
    value: Option<&'a T>,
    separator: &'static str,
}
impl<'a, T> FmtOptional<'a, T>
{
    pub fn new(value: Option<&'a T>) -> Self
    {
        Self {
            value,
            separator: " ",
        }
    }
    pub fn with_separator(mut self, separator: &'static str) -> Self
    {
        self.separator = separator;
        self
    }
}

hexga_map_on::map_on_std_fmt!(
    ($trait_name:ident) =>
    {
        impl<'a, T> std::fmt::$trait_name for FmtOptional<'a, T> where T: std::fmt::$trait_name
        {
            fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult
            {
                match self.value
                {
                    Some(v) =>
                    {
                        v.fmt(f)?;
                        f.write_str(self.separator)
                    },
                    None => Ok(()),
                }
            }
        }
    }
);

/*
pub trait FmtOptionalDefault : Sized
{
    /// Formats the value with a space separator after it only if it is not the default.
    fn fmt_if_not_defaut(&self) -> FmtOptional<'_, Self> where Self: Default + PartialEq
    {
        FmtOptional::new(if self.is_default() { None } else { Some(self) })
    }
}
impl<T> FmtOptionalDefault for T where T: Sized {}
*/
