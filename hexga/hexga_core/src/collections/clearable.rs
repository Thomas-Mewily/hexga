use std::{collections::*, ffi::OsString, path::PathBuf};

pub mod prelude
{
    pub use super::Clearable;
}

/// Clear the collection
pub trait Clearable //: Length
{
    fn clear(&mut self);
    //fn remove_all(&mut self)
}

impl<T>         Clearable for Vec<T>            { #[inline(always)] fn clear(&mut self) { self.clear(); } }
impl<T>         Clearable for VecDeque<T>       { #[inline(always)] fn clear(&mut self) { self.clear(); } }
impl<T, S>      Clearable for HashSet<T, S>     { #[inline(always)] fn clear(&mut self) { self.clear(); } }
impl<T>         Clearable for BinaryHeap<T>     { #[inline(always)] fn clear(&mut self) { self.clear(); } }
impl<T>         Clearable for BTreeSet<T>       { #[inline(always)] fn clear(&mut self) { self.clear(); } }
impl<T>         Clearable for LinkedList<T>     { #[inline(always)] fn clear(&mut self) { self.clear(); } }
impl<K, V, S>   Clearable for HashMap<K, V, S>  { #[inline(always)] fn clear(&mut self) { self.clear(); } }
impl<K, V>      Clearable for BTreeMap<K, V>    { #[inline(always)] fn clear(&mut self) { self.clear(); } }
impl            Clearable for String            { #[inline(always)] fn clear(&mut self) { self.clear(); } }
impl            Clearable for OsString          { #[inline(always)] fn clear(&mut self) { self.clear(); } }
impl            Clearable for PathBuf           { #[inline(always)] fn clear(&mut self) { self.clear(); } }
impl<T>         Clearable for Option<T>         { #[inline(always)] fn clear(&mut self) { *self = None; } }

