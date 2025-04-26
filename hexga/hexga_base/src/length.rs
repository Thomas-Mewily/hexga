use std::collections::*;

pub trait Length 
{
    // Returns the number of elements inside this collection, also referred as a 'length'.
    fn len(&self) -> usize;

    /// True if the container contains 0 elements.
    ///
    /// ```rust
    /// use hexga_base::*;
    ///
    /// assert_eq!([1, 2, 3].is_empty(), false);
    ///
    /// let empty_array : [i32; 0] = [];
    /// assert_eq!(empty_array.is_empty(), true);
    ///
    /// assert_eq!("".is_empty(), true);
    /// assert_eq!("hello".is_empty(), false);
    ///
    /// assert_eq!("".is_empty(), !("".is_not_empty()));
    /// ```
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// True if the container contains at least one element.
    /// Will always be different than `is_empty()`.
    ///
    /// ```rust
    /// use hexga_base::*;
    ///
    /// assert_eq!([1, 2, 3].is_not_empty(), true);
    ///
    /// let empty_array : [i32; 0] = [];
    /// assert_eq!(empty_array.is_not_empty(), false);
    ///
    /// assert_eq!("".is_not_empty(), false);
    /// assert_eq!("hello".is_not_empty(), true);
    ///
    /// assert_eq!("".is_empty(), !("".is_not_empty()));
    /// ```
    fn is_not_empty(&self) -> bool {
        !self.is_empty()
    }
}

use std::ffi::OsStr;

impl<T>                 Length for Vec<T>           { fn len(&self) -> usize { self.len() } }
impl<T>                 Length for VecDeque<T>      { fn len(&self) -> usize { self.len() } }
impl<T,S>               Length for HashSet<T,S>     { fn len(&self) -> usize { self.len() } }
impl<T>                 Length for BinaryHeap<T>    { fn len(&self) -> usize { self.len() } }
impl<T>                 Length for BTreeSet<T>      { fn len(&self) -> usize { self.len() } }
impl<T>                 Length for LinkedList<T>    { fn len(&self) -> usize { self.len() } }
impl<K, V, S>           Length for HashMap<K, V, S> { fn len(&self) -> usize { self.len() } }
impl<K, V>              Length for BTreeMap<K, V>   { fn len(&self) -> usize { self.len() } }
impl<T>                 Length for [T]              { fn len(&self) -> usize { self.len() } }
impl                    Length for String           { fn len(&self) -> usize { self.len() } }
impl                    Length for str              { fn len(&self) -> usize { self.len() } }
impl                    Length for OsStr            { fn len(&self) -> usize { self.len() } }
impl<T, const N: usize> Length for [T; N]           { fn len(&self) -> usize { N } }
