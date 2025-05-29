pub use rayon::prelude::*;
pub use default_is_triple_underscore::*;

use std::fmt::Debug;

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
/// Some of my lib will probably have proper error type instead of () (Look for `#proper_error` to know which error type are temporary) when I will have time to add them
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

pub trait DefaultExtension : Default + PartialEq 
{
    fn is_default(&self) -> bool { self == &___() }
    fn is_not_default(&self) -> bool { !self.is_default() }
}
impl<T> DefaultExtension for T where T : Default + PartialEq {}

pub trait DebugExtension
{
    fn field_if<T : Debug>(&mut self, name: &str, value: &T, pred : impl FnOnce(&T) -> bool) -> &mut Self;
    fn field_if_not_default<T : Debug + Default + PartialEq>(&mut self, name: &str, value: &T) -> &mut Self { self.field_if(name, value, |v| v.is_not_default()) }

    fn field<T : Debug>(&mut self, name: &str, value: &T) -> &mut Self;

    fn field_if_false(&mut self, name: &str, value: bool) -> &mut Self { if !value { self.field(name, &value); } self }
    fn field_if_true(&mut self, name: &str, value: bool) -> &mut Self { if value { self.field(name, &value); } self }

    fn field_bool(&mut self, value : bool, name_true : &str, name_false : &str) -> &mut Self;
}
 
impl<'a, 'b: 'a> DebugExtension for std::fmt::DebugStruct<'a, 'b>
{
    fn field_if<T : Debug>(&mut self, name: &str, value: &T, pred : impl FnOnce(&T) -> bool) -> &mut Self 
    {
        if pred(value)
        {
            self.field(name, value);
        }
        self
    }
    
    fn field<T : Debug>(&mut self, name: &str, value: &T) -> &mut Self {
        self.field(name, value)
    }
    
    fn field_bool(&mut self, value : bool, name_true : &str, name_false : &str) -> &mut Self {
        self.field(if value { name_true } else { name_false }, &())
    }
}

/*
pub trait IteratorExtension<T> where T : PartialEq
{
    fn contains(self, value : &T) -> bool;
}
impl<I,T> IteratorExtension<T> for I where I : Iterator<Item = T>, T : PartialEq
{
    fn contains(mut self, value : &T) -> bool { self.position(|v| &v == value).is_some() }
} 
*/