use std::{alloc::{Layout, alloc}, ptr::NonNull};

use super::*;

/*
#[repr(C)]
pub struct DynArray<T> {
    ptr: NonNull<T>,
    len: usize,
}

impl<T> DynArray<T> {
    pub fn new(len: usize) -> Self {
        let layout = Layout::array::<T>(len).unwrap();
        let ptr = unsafe { alloc(layout) as *mut T };
        let ptr = NonNull::new(ptr).expect("allocation failed");
        Self { ptr, len }
    }

    pub fn len(&self) -> usize {
        self.len
    }
}
*/

// Note: it is useless to have view_mut type (for iterating over slot) because this will break the invariant
// Todo: be able to change the generation of a slot

/*
Common trait a features

Debug
PartialEq, Eq, Hash
From<Vector>
From<[T;N]>
FromIterator
Deref/DerefMut/AsRef/AsMut on slice
Index/IndexMut/Get/GetMut/TryGet/TryGetMut/GetManyMut that work for <I>: SliceIndex
IntoIterator for self, &self, &mut self
Collection, CollectionBijective,
Length
Push<T>
TryPush<T>
Length / Capacity (same impl)
WithCapacity

fn new_zeroed(len: usize) -> Self
fn from_fn(len: usize, f: impl Fn(usize) -> T) -> Self

*/
