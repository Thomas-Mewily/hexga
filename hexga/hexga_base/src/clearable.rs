use std::{collections::*, ffi::OsString, path::PathBuf};

pub trait Clearable //: Length
{
    fn clear(&mut self);
}

impl<T>         Clearable for Vec<T>            { fn clear(&mut self) { self.clear(); } }
impl<T>         Clearable for VecDeque<T>       { fn clear(&mut self) { self.clear(); } }
impl<T, S>      Clearable for HashSet<T, S>     { fn clear(&mut self) { self.clear(); } }
impl<T>         Clearable for BinaryHeap<T>     { fn clear(&mut self) { self.clear(); } }
impl<T>         Clearable for BTreeSet<T>       { fn clear(&mut self) { self.clear(); } }
impl<T>         Clearable for LinkedList<T>     { fn clear(&mut self) { self.clear(); } }
impl<K, V, S>   Clearable for HashMap<K, V, S>  { fn clear(&mut self) { self.clear(); } }
impl<K, V>      Clearable for BTreeMap<K, V>    { fn clear(&mut self) { self.clear(); } }
impl            Clearable for String            { fn clear(&mut self) { self.clear(); } }
impl            Clearable for OsString          { fn clear(&mut self) { self.clear(); } }
impl            Clearable for PathBuf           { fn clear(&mut self) { self.clear(); } }
impl<T>         Clearable for Option<T>         { fn clear(&mut self) { *self = None; } }

