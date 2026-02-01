use hexga_core::prelude::*;
use std::fmt;
use std::iter::{FromIterator, IntoIterator};
/// A fixed-capacity vector backed by an inline array.
///
/// Thank to the crate [arrayvec by bluss](https://docs.rs/arrayvec/latest/arrayvec/), most of the code is based on it.
use std::mem::{ManuallyDrop, MaybeUninit};
use std::ops::{Bound, Deref, DerefMut, Index, IndexMut, RangeBounds};
use std::slice::SliceIndex;
use std::{ptr, slice};

pub mod prelude
{
    pub use super::ArrayVec;
}

/// A fixed-capacity vector backed by an inline array.
///
/// `ArrayVec<T, CAP>` stores up to `CAP` elements without heap allocation.
/// Unlike `Vec<T>`, it cannot grow beyond `CAP`.
///
/// When the capacity is exceeded, [`push`] will panic; use [`try_push`] if you
/// want to handle the full case.
pub struct ArrayVec<T, const CAP: usize>
{
    values: [MaybeUninit<T>; CAP],
    len: usize,
}

impl<T, const CAP: usize> ArrayVec<T, CAP>
{
    /// Creates a new empty `ArrayVec`.
    ///
    /// # Examples
    ///
    /// ```
    /// use hexga_array_vec::ArrayVec;
    ///
    /// let mut vec = ArrayVec::<i32, 4>::new();
    /// assert!(vec.is_empty());
    /// ```
    pub const fn new() -> Self
    {
        Self {
            // SAFETY: An uninitialized `[MaybeUninit<_>; CAP]` is valid.
            values: unsafe { MaybeUninit::uninit().assume_init() },
            len: 0,
        }
    }

    /// Returns the number of elements the vector can hold.
    ///
    /// # Examples
    ///
    /// ```
    /// use hexga_array_vec::ArrayVec;
    ///
    /// let vec = ArrayVec::<i32, 3>::new();
    /// assert_eq!(vec.capacity(), 3);
    /// ```
    pub const fn capacity(&self) -> usize { CAP }

    /// Returns the capacity left in the `ArrayVec`.
    ///
    /// ```
    /// use hexga_array_vec::ArrayVec;
    ///
    /// let mut array = ArrayVec::from([1, 2, 3]);
    /// array.pop();
    /// assert_eq!(array.remaining_capacity(), 1);
    /// ```
    pub const fn remaining_capacity(&self) -> usize { self.capacity() - self.len() }

    /// Shortens the vector, keeping the first `len` elements and dropping
    /// the rest.
    ///
    /// If `len` is greater than the vector’s current length this has no
    /// effect.
    ///
    /// ```
    /// use hexga_array_vec::ArrayVec;
    ///
    /// let mut array = ArrayVec::from([1, 2, 3, 4, 5]);
    /// array.truncate(3);
    /// assert_eq!(&array[..], &[1, 2, 3]);
    /// array.truncate(4);
    /// assert_eq!(&array[..], &[1, 2, 3]);
    /// ```
    pub fn truncate(&mut self, new_len: usize)
    {
        unsafe {
            let len = self.len();
            if new_len < len
            {
                self.set_len(new_len);
                let tail = slice::from_raw_parts_mut(self.as_mut_ptr().add(new_len), len - new_len);
                ptr::drop_in_place(tail);
            }
        }
    }

    /// Returns the number of elements in the vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use hexga_array_vec::ArrayVec;
    ///
    /// let mut vec = ArrayVec::<i32, 4>::new();
    /// assert_eq!(vec.len(), 0);
    /// vec.push(1);
    /// assert_eq!(vec.len(), 1);
    /// ```
    pub const fn len(&self) -> usize { self.len }

    /// Returns `true` if the vector contains no elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use hexga_array_vec::ArrayVec;
    ///
    /// let mut vec = ArrayVec::<i32, 4>::new();
    /// assert!(vec.is_empty());
    /// vec.push(1);
    /// assert!(!vec.is_empty());
    /// ```
    pub const fn is_empty(&self) -> bool { self.len == 0 }

    /// Returns `true` if the vector is at full capacity.
    ///
    /// # Examples
    ///
    /// ```
    /// use hexga_array_vec::ArrayVec;
    ///
    /// let mut vec = ArrayVec::<i32, 2>::new();
    /// assert!(!vec.is_full());
    /// vec.push(1);
    /// vec.push(2);
    /// assert!(vec.is_full());
    /// ```
    pub const fn is_full(&self) -> bool { self.len == CAP }

    /// Appends an element to the back of the vector.
    ///
    /// # Panics
    ///
    /// Panics if the vector is already at full capacity.
    ///
    /// # Examples
    ///
    /// ```
    /// use hexga_array_vec::ArrayVec;
    ///
    /// let mut vec = ArrayVec::<i32, 3>::new();
    /// vec.push(1);
    /// vec.push(2);
    /// assert_eq!(vec.len(), 2);
    /// ```
    #[track_caller]
    pub fn push(&mut self, value: T)
    {
        if self.len >= CAP
        {
            panic!("ArrayVec is at full capacity");
        }

        // SAFETY: We just checked that there's space
        unsafe {
            self.values
                .as_mut_ptr()
                .add(self.len)
                .write(MaybeUninit::new(value));
        }
        self.len += 1;
    }

    /// Appends an element to the back of the vector, returning `Err` if full.
    ///
    /// # Examples
    ///
    /// ```
    /// use hexga_array_vec::ArrayVec;
    ///
    /// let mut vec = ArrayVec::<i32, 2>::new();
    /// assert!(vec.try_push(1).is_ok());
    /// assert!(vec.try_push(2).is_ok());
    /// assert!(vec.try_push(3).is_err()); // Full!
    /// ```
    pub fn try_push(&mut self, value: T) -> Result<(), CapacityFullError<T>>
    {
        if self.len >= CAP
        {
            return Err(CapacityFullError::new(value));
        }

        // SAFETY: We just checked that there's space
        unsafe {
            self.values
                .as_mut_ptr()
                .add(self.len)
                .write(MaybeUninit::new(value));
        }
        self.len += 1;
        Ok(())
    }

    /// Appends an element to the back of the vector without checking capacity.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the vector is not at full capacity.
    pub unsafe fn push_unchecked(&mut self, value: T)
    {
        debug_assert!(self.len < CAP, "ArrayVec capacity exceeded");
        unsafe {
            self.values
                .as_mut_ptr()
                .add(self.len)
                .write(MaybeUninit::new(value))
        };
        self.len += 1;
    }

    /// Removes the last element from a vector and returns it, or `None` if it is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use hexga_array_vec::ArrayVec;
    ///
    /// let mut vec = ArrayVec::<i32, 3>::new();
    /// vec.push(1);
    /// vec.push(2);
    /// assert_eq!(vec.pop(), Some(2));
    /// assert_eq!(vec.pop(), Some(1));
    /// assert_eq!(vec.pop(), None);
    /// ```
    pub fn pop(&mut self) -> Option<T>
    {
        if self.len == 0
        {
            return None;
        }

        self.len -= 1;
        // SAFETY: We just decremented len, so this element is initialized
        unsafe { Some(self.values.as_ptr().add(self.len).read().assume_init()) }
    }

    /// Returns a reference to the element at the given index, or `None` if out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use hexga_array_vec::ArrayVec;
    ///
    /// let mut vec = ArrayVec::<i32, 3>::new();
    /// vec.push(1);
    /// vec.push(2);
    /// assert_eq!(vec.get(0), Some(&1));
    /// assert_eq!(vec.get(2), None);
    /// ```
    pub fn get<Idx>(&self, index: Idx) -> Option<&<Self as Get<Idx>>::Output>
    where
        Self: Get<Idx>,
    {
        Get::get(self, index)
    }

    /// Returns a mutable reference to the element at the given index, or `None` if out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use hexga_array_vec::ArrayVec;
    ///
    /// let mut vec = ArrayVec::<i32, 3>::new();
    /// vec.push(1);
    /// if let Some(elem) = vec.get_mut(0) {
    ///     *elem = 42;
    /// }
    /// assert_eq!(vec[0], 42);
    /// ```
    pub fn get_mut<Idx>(&mut self, index: Idx) -> Option<&mut <Self as Get<Idx>>::Output>
    where
        Self: GetMut<Idx>,
    {
        GetMut::get_mut(self, index)
    }

    /// Remove all elements in the vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use hexga_array_vec::ArrayVec;
    ///
    /// let mut vec = ArrayVec::<i32, 3>::new();
    /// vec.push(1);
    /// vec.push(2);
    /// vec.clear();
    /// assert!(vec.is_empty());
    /// ```
    pub fn clear(&mut self) { self.truncate(0); }

    /// Removes the specified range from the vector in bulk, returning all
    /// removed elements as an iterator. If the iterator is dropped before
    /// being fully consumed, it drops the remaining removed elements.
    ///
    /// # Panics
    ///
    /// Panics if the starting point is greater than the end point or if
    /// the end point is greater than the length of the vector.
    pub fn drain<'s, R>(&'s mut self, range: R) -> Drain<'s, T, CAP>
    where
        R: RangeBounds<usize>,
    {
        let len = self.len();

        let start = match range.start_bound()
        {
            Bound::Unbounded => 0,
            Bound::Included(&i) => i,
            Bound::Excluded(&i) => i.saturating_add(1),
        };
        let end = match range.end_bound()
        {
            Bound::Excluded(&j) => j,
            Bound::Included(&j) => j.saturating_add(1),
            Bound::Unbounded => len,
        };

        // Bounds checking is done by the slice itself
        self.drain_range(start, end)
    }

    fn drain_range<'s>(&'s mut self, start: usize, end: usize) -> Drain<'s, T, CAP>
    {
        let len = self.len();

        let range_ptr: *const [T] = &self[start..end];

        self.len = start;

        unsafe {
            Drain {
                tail_start: end,
                tail_len: len - end,
                iter: (*range_ptr).iter(),
                vec: self as *mut _,
            }
        }
    }

    /// Removes and returns the element at position `index`.
    ///
    /// # Panics
    ///
    /// Panics if `index` is out of bounds.
    pub fn remove(&mut self, index: usize) -> T
    {
        self.try_remove(index).unwrap_or_else(|| {
            panic!(
                "remove index (is {}) should be < len (is {})",
                index,
                self.len()
            )
        })
    }

    /// Removes and returns the element at position `index`, returning `None` if out of bounds.
    pub fn try_remove(&mut self, index: usize) -> Option<T>
    {
        if index >= self.len()
        {
            None
        }
        else
        {
            self.drain(index..=index).next()
        }
    }

    /// Insert `element` at position `index`.
    ///
    /// Shift up all elements after `index`.
    ///
    /// It is an error if the index is greater than the length or if the
    /// ArrayVec is full.
    ///
    /// ***Panics*** if the array is full or the `index` is out of bounds. See
    /// `try_insert` for fallible version.
    ///
    /// ```
    /// use hexga_array_vec::ArrayVec;
    ///
    /// let mut array = ArrayVec::<_, 2>::new();
    ///
    /// array.insert(0, "x");
    /// array.insert(0, "y");
    /// assert_eq!(&array[..], &["y", "x"]);
    ///
    /// ```
    #[track_caller]
    pub fn insert(&mut self, index: usize, element: T) { self.try_insert(index, element).unwrap() }

    /// Inserts an element at position `index`, shifting all elements after it to the right.
    ///
    /// # Panics
    ///
    /// Panics if `index` is out of bounds or if the vector is at full capacity.
    ///
    /// # Examples
    ///
    /// ```
    /// use hexga_array_vec::ArrayVec;
    ///
    /// let mut vec = ArrayVec::<i32, 4>::new();
    /// vec.push(1);
    /// vec.push(3);
    /// vec.insert(1, 2);
    /// assert_eq!(vec[0], 1);
    /// assert_eq!(vec[1], 2);
    /// assert_eq!(vec[2], 3);
    /// ```
    pub fn try_insert(&mut self, index: usize, element: T) -> Result<(), CapacityFullError<T>>
    {
        if index > self.len()
        {
            panic!("invalid index {} len is {}", index, self.len())
        }
        if self.len() == self.capacity()
        {
            return Err(CapacityFullError::new(element));
        }
        let len = self.len();

        unsafe {
            {
                let p: *mut _ = self.get_unchecked_ptr(index);
                // Shift everything over to make space. (Duplicating the
                // `index`th element into two consecutive places.)
                ptr::copy(p, p.offset(1), len - index);
                // Write it in, overwriting the first copy of the `index`th
                // element.
                ptr::write(p, element);
            }
            self.set_len(len + 1);
        }
        Ok(())
    }

    unsafe fn get_unchecked_ptr(&mut self, index: usize) -> *mut T
    {
        unsafe { self.as_mut_ptr().add(index) }
    }

    /// Returns a slice containing all elements in the vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use hexga_array_vec::ArrayVec;
    ///
    /// let mut vec = ArrayVec::<i32, 3>::new();
    /// vec.push(1);
    /// vec.push(2);
    /// assert_eq!(vec.as_slice(), &[1, 2]);
    /// ```
    pub fn as_slice(&self) -> &[T]
    {
        // SAFETY: The first `len` elements are initialized
        unsafe { std::slice::from_raw_parts(self.values.as_ptr() as *const T, self.len) }
    }

    /// Returns a mutable slice containing all elements in the vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use hexga_array_vec::ArrayVec;
    ///
    /// let mut vec = ArrayVec::<i32, 3>::new();
    /// vec.push(1);
    /// vec.push(2);
    /// vec.as_mut_slice()[0] = 42;
    /// assert_eq!(vec[0], 42);
    /// ```
    pub fn as_mut_slice(&mut self) -> &mut [T]
    {
        // SAFETY: The first `len` elements are initialized
        unsafe { std::slice::from_raw_parts_mut(self.values.as_mut_ptr() as *mut T, self.len) }
    }

    /// Resizes the vector to the new length.
    ///
    /// If `new_len` is greater than the current length, the vector is extended with clones of `value`.
    /// If `new_len` is less than the current length, the vector is truncated.
    ///
    /// # Panics
    ///
    /// Panics if `new_len` exceeds the capacity.
    ///
    /// # Examples
    ///
    /// ```
    /// use hexga_array_vec::ArrayVec;
    ///
    /// let mut vec = ArrayVec::<i32, 5>::new();
    /// vec.push(1);
    /// vec.resize(3, 0);
    /// assert_eq!(vec, [1, 0, 0]);
    /// ```
    pub fn resize(&mut self, new_len: usize, value: T)
    where
        T: Clone,
    {
        if new_len > CAP
        {
            panic!("new_len exceeds capacity");
        }

        if new_len > self.len
        {
            while self.len < new_len
            {
                self.push(value.clone());
            }
        }
        else
        {
            while self.len > new_len
            {
                self.pop();
            }
        }
    }

    /// Returns the remaining spare capacity of the vector as a slice of
    /// `MaybeUninit<T>`.
    ///
    /// The returned slice can be used to fill the vector with data (e.g. by
    /// reading from a file) before marking the data as initialized using the
    /// [`set_len`] method.
    ///
    /// [`set_len`]: ArrayVec::set_len
    ///
    /// # Examples
    ///
    /// ```
    /// use hexga_utils::array_vec::ArrayVec;
    ///
    /// // Allocate vector big enough for 10 elements.
    /// let mut v: ArrayVec<i32, 10> = ArrayVec::new();
    ///
    /// // Fill in the first 3 elements.
    /// let uninit = v.spare_capacity_mut();
    /// uninit[0].write(0);
    /// uninit[1].write(1);
    /// uninit[2].write(2);
    ///
    /// // Mark the first 3 elements of the vector as being initialized.
    /// unsafe {
    ///     v.set_len(3);
    /// }
    ///
    /// assert_eq!(&v[..], &[0, 1, 2]);
    /// ```
    pub fn spare_capacity_mut(&mut self) -> &mut [MaybeUninit<T>]
    {
        let len = self.len();
        &mut self.values[len..]
    }

    /// Set the vector’s length without dropping or moving out elements
    ///
    /// This method is `unsafe` because it changes the notion of the
    /// number of “valid” elements in the vector. Use with care.
    ///
    /// This method uses *debug assertions* to check that `length` is
    /// not greater than the capacity.
    pub unsafe fn set_len(&mut self, length: usize) { self.len = length; }

    /// Remove the element at `index` and swap the last element into its place.
    ///
    /// This is a checked version of `.swap_remove`.
    /// This operation is O(1).
    ///
    /// Return `Some(` *element* `)` if the index is in bounds, else `None`.
    ///
    /// ```
    /// use hexga_array_vec::ArrayVec;
    ///
    /// let mut array = ArrayVec::from([1, 2, 3]);
    ///
    /// assert_eq!(array.swap_pop(0), Some(1));
    /// assert_eq!(&array[..], &[3, 2]);
    ///
    /// assert_eq!(array.swap_pop(10), None);
    /// ```
    pub fn swap_pop(&mut self, index: usize) -> Option<T>
    {
        let len = self.len();
        if index >= len
        {
            return None;
        }
        self.swap(index, len - 1);
        self.pop()
    }

    /// Remove the element at `index` and swap the last element into its place.
    ///
    /// This operation is O(1).
    ///
    /// Return the *element* if the index is in bounds, else panic.
    ///
    /// ***Panics*** if the `index` is out of bounds.
    ///
    /// ```
    /// use hexga_array_vec::ArrayVec;
    ///
    /// let mut array = ArrayVec::from([1, 2, 3]);
    ///
    /// assert_eq!(array.swap_remove(0), 1);
    /// assert_eq!(&array[..], &[3, 2]);
    ///
    /// assert_eq!(array.swap_remove(1), 2);
    /// assert_eq!(&array[..], &[3]);
    /// ```
    pub fn swap_remove(&mut self, index: usize) -> T
    {
        self.swap_pop(index)
            .unwrap_or_else(|| panic!("Can't swap remove at idx {index}"))
    }

    /// Retains only the elements specified by the predicate.
    ///
    /// In other words, remove all elements `e` such that `f(&mut e)` returns false.
    /// This method operates in place and preserves the order of the retained
    /// elements.
    ///
    /// ```
    /// use hexga_array_vec::ArrayVec;
    ///
    /// let mut array = ArrayVec::from([1, 2, 3, 4]);
    /// array.retain(|x| *x & 1 != 0 );
    /// assert_eq!(&array[..], &[1, 3]);
    /// ```
    pub fn retain<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut T) -> bool,
    {
        // Check the implementation of
        // https://doc.rust-lang.org/std/vec/struct.Vec.html#method.retain
        // for safety arguments (especially regarding panics in f and when
        // dropping elements). Implementation closely mirrored here.

        let original_len = self.len();
        unsafe { self.set_len(0) };

        struct BackshiftOnDrop<'a, T, const CAP: usize>
        {
            v: &'a mut ArrayVec<T, CAP>,
            processed_len: usize,
            deleted_cnt: usize,
            original_len: usize,
        }

        impl<T, const CAP: usize> Drop for BackshiftOnDrop<'_, T, CAP>
        {
            fn drop(&mut self)
            {
                if self.deleted_cnt > 0
                {
                    unsafe {
                        ptr::copy(
                            self.v.as_ptr().add(self.processed_len),
                            self.v
                                .as_mut_ptr()
                                .add(self.processed_len - self.deleted_cnt),
                            self.original_len - self.processed_len,
                        );
                    }
                }
                unsafe {
                    self.v.set_len(self.original_len - self.deleted_cnt);
                }
            }
        }

        let mut g = BackshiftOnDrop {
            v: self,
            processed_len: 0,
            deleted_cnt: 0,
            original_len,
        };

        #[inline(always)]
        fn process_one<F: FnMut(&mut T) -> bool, T, const CAP: usize, const DELETED: bool>(
            f: &mut F,
            g: &mut BackshiftOnDrop<'_, T, CAP>,
        ) -> bool
        {
            let cur = unsafe { g.v.as_mut_ptr().add(g.processed_len) };
            if !f(unsafe { &mut *cur })
            {
                g.processed_len += 1;
                g.deleted_cnt += 1;
                unsafe { ptr::drop_in_place(cur) };
                return false;
            }
            if DELETED
            {
                unsafe {
                    let hole_slot = cur.sub(g.deleted_cnt);
                    ptr::copy_nonoverlapping(cur, hole_slot, 1);
                }
            }
            g.processed_len += 1;
            true
        }

        // Stage 1: Nothing was deleted.
        while g.processed_len != original_len
        {
            if !process_one::<F, T, CAP, false>(&mut f, &mut g)
            {
                break;
            }
        }

        // Stage 2: Some elements were deleted.
        while g.processed_len != original_len
        {
            process_one::<F, T, CAP, true>(&mut f, &mut g);
        }

        drop(g);
    }

    /// Copy all elements from the slice and append to the `ArrayVec`.
    ///
    /// ```
    /// use hexga_array_vec::ArrayVec;
    ///
    /// let mut vec: ArrayVec<usize, 10> = ArrayVec::new();
    /// vec.push(1);
    /// vec.try_extend_from_slice(&[2, 3]).unwrap();
    /// assert_eq!(&vec[..], &[1, 2, 3]);
    /// ```
    ///
    /// # Errors
    ///
    /// This method will return an error if the capacity left (see
    /// [`remaining_capacity`]) is smaller then the length of the provided
    /// slice.
    ///
    /// [`remaining_capacity`]: #method.remaining_capacity
    pub fn try_extend_from_slice(&mut self, other: &[T]) -> Result<(), CapacityFullError>
    where
        T: Copy,
    {
        if self.remaining_capacity() < other.len()
        {
            return Err(CapacityFullError::new(()));
        }

        let self_len = self.len();
        let other_len = other.len();

        unsafe {
            let dst = self.get_unchecked_ptr(self_len);
            ptr::copy_nonoverlapping(other.as_ptr(), dst, other_len);
            self.set_len(self_len + other_len);
        }
        Ok(())
    }

    /// Attempts to fill the `ArrayVec` from an iterator, returning `None` if too many elements.
    ///
    /// If the iterator yields more than `CAP` elements, returns `None` and leaves
    /// the `ArrayVec` in an unspecified state (some elements may have been added).
    ///
    /// Returns `Some(self)` on success.
    pub fn try_from_iter<I>(mut self, it: I) -> Option<Self>
    where
        I: IntoIterator<Item = T>,
    {
        for item in it.into_iter()
        {
            if self.try_push(item).is_err()
            {
                return None;
            }
        }
        Some(self)
    }

    /// Converts into an array if length equals `N`.
    pub fn into_array<const N: usize>(mut self) -> Result<[T; N], Self>
    {
        if self.len != N
        {
            return Err(self);
        }

        self.len = 0;

        unsafe {
            let mut result: [MaybeUninit<T>; N] = MaybeUninit::uninit().assume_init();

            for i in 0..N
            {
                let value = self.values.as_mut_ptr().add(i).read().assume_init();
                result.as_mut_ptr().add(i).write(MaybeUninit::new(value));
            }

            Ok(result.map(|x| x.assume_init()))
        }
    }

    /// Returns array reference if length equals `N`.
    pub fn as_array<const N: usize>(&self) -> Option<&[T; N]> { self.as_slice().as_array() }
    /// Returns mutable array reference if length equals `N`.
    pub fn as_mut_array<const N: usize>(&mut self) -> Option<&mut [T; N]>
    {
        self.as_mut_slice().as_mut_array()
    }

    /// Return the inner fixed size array.
    ///
    /// Safety:
    /// This operation is safe if and only if length equals capacity.
    pub unsafe fn into_inner_unchecked(self) -> [T; CAP]
    {
        debug_assert_eq!(self.len(), self.capacity());
        let self_ = ManuallyDrop::new(self);
        let array = unsafe { ptr::read(self_.as_ptr() as *const [T; CAP]) };
        array
    }

    /// Return a raw pointer to the vector's buffer.
    pub fn as_ptr(&self) -> *const T { self.values.as_ptr() as _ }

    /// Return a raw mutable pointer to the vector's buffer.
    pub fn as_mut_ptr(&mut self) -> *mut T { self.values.as_mut_ptr() as _ }
}

impl<T, const CAP: usize> Drop for ArrayVec<T, CAP>
{
    fn drop(&mut self) { self.clear(); }
}

// Deref implementations for slice-like behavior
impl<T, const CAP: usize> Deref for ArrayVec<T, CAP>
{
    type Target = [T];

    fn deref(&self) -> &Self::Target { self.as_slice() }
}

impl<T, const CAP: usize> DerefMut for ArrayVec<T, CAP>
{
    fn deref_mut(&mut self) -> &mut Self::Target { self.as_mut_slice() }
}

pub type Iter<'a, T> = std::slice::Iter<'a, T>;
pub type IterMut<'a, T> = std::slice::IterMut<'a, T>;

impl<T, const CAP: usize> ArrayVec<T, CAP>
{
    /// Returns an iterator over the slice.
    ///
    /// # Examples
    ///
    /// ```
    /// use hexga_array_vec::ArrayVec;
    ///
    /// let mut vec = ArrayVec::<i32, 3>::new();
    /// vec.push(1);
    /// vec.push(2);
    /// let mut iter = vec.iter();
    /// assert_eq!(iter.next(), Some(&1));
    /// assert_eq!(iter.next(), Some(&2));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn iter(&self) -> Iter<'_, T> { self.as_slice().iter() }

    /// Returns an iterator that allows modifying each value.
    ///
    /// # Examples
    ///
    /// ```
    /// use hexga_array_vec::ArrayVec;
    ///
    /// let mut vec = ArrayVec::<i32, 3>::new();
    /// vec.push(1);
    /// vec.push(2);
    /// for elem in vec.iter_mut() {
    ///     *elem *= 2;
    /// }
    /// assert_eq!(vec[0], 2);
    /// assert_eq!(vec[1], 4);
    /// ```
    pub fn iter_mut(&mut self) -> IterMut<'_, T> { self.as_mut_slice().iter_mut() }
}

impl<'a, T, const CAP: usize> IntoIterator for &'a ArrayVec<T, CAP>
{
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter { self.iter() }
}

impl<'a, T, const CAP: usize> IntoIterator for &'a mut ArrayVec<T, CAP>
{
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter { self.iter_mut() }
}

impl<T, Idx, const CAP: usize> Index<Idx> for ArrayVec<T, CAP>
where
    Idx: SliceIndex<[T]>,
{
    type Output = Idx::Output;

    fn index(&self, index: Idx) -> &Self::Output { self.as_slice().index(index) }
}

impl<T, Idx, const CAP: usize> IndexMut<Idx> for ArrayVec<T, CAP>
where
    Idx: SliceIndex<[T]>,
{
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output
    {
        self.as_mut_slice().index_mut(index)
    }
}

// Display implementation
impl<T: fmt::Debug, const CAP: usize> fmt::Debug for ArrayVec<T, CAP>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl<T: Clone, const CAP: usize> Clone for ArrayVec<T, CAP>
where
    T: Clone,
{
    fn clone(&self) -> Self
    {
        let mut new_vec = ArrayVec::new();
        for item in self.iter()
        {
            unsafe {
                new_vec.push_unchecked(item.clone());
            }
        }
        new_vec
    }
}

/// Panics if the iterator contains more elements than capacity
impl<T, const CAP: usize> FromIterator<T> for ArrayVec<T, CAP>
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self
    {
        let mut vec = ArrayVec::new();
        for item in iter
        {
            vec.try_push(item).expect("iterator contains more elements than capacity");
        }
        vec
    }
}
impl<T, const CAP: usize> TryFromIterator<T> for ArrayVec<T, CAP>
{
    type Error=CapacityFullError<(Self,T)>;
    fn try_from_iter<It: IntoIterator<Item = T>>(iter: It) -> Result<Self, Self::Error> {
        let mut vec = ArrayVec::new();
        for item in iter
        {
            match vec.try_push(item)
            {
                Ok(_) => {},
                Err(e) => return Err(CapacityFullError::new((vec, e.element))),
            }
        }
        Ok(vec)
    }
}

// IntoIterator implementations
impl<T, const CAP: usize> IntoIterator for ArrayVec<T, CAP>
{
    type Item = T;
    type IntoIter = IntoIter<T, CAP>;

    fn into_iter(self) -> Self::IntoIter
    {
        IntoIter {
            index: 0,
            array: self,
        }
    }
}

pub struct IntoIter<T, const CAP: usize>
{
    array: ArrayVec<T, CAP>,
    index: usize,
}

impl<T, const CAP: usize> IntoIter<T, CAP>
{
    /// Returns the remaining items of this iterator as a slice.
    pub fn as_slice(&self) -> &[T] { &self.array[self.index..] }

    /// Returns the remaining items of this iterator as a mutable slice.
    pub fn as_mut_slice(&mut self) -> &mut [T] { &mut self.array[self.index..] }
}

impl<T, const CAP: usize> Iterator for IntoIter<T, CAP>
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item>
    {
        if self.index == self.array.len
        {
            None
        }
        else
        {
            unsafe {
                let ptr = self.array.values.as_ptr().add(self.index) as *const T;
                self.index += 1;
                Some(ptr::read(ptr))
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>)
    {
        let len = self.array.len - self.index;
        (len, Some(len))
    }
}

impl<T, const CAP: usize> DoubleEndedIterator for IntoIter<T, CAP>
{
    fn next_back(&mut self) -> Option<Self::Item>
    {
        if self.index == self.array.len
        {
            None
        }
        else
        {
            unsafe {
                self.array.len -= 1;
                let ptr = self.array.values.as_ptr().add(self.array.len) as *const T;
                Some(ptr::read(ptr))
            }
        }
    }
}

impl<T, const CAP: usize> ExactSizeIterator for IntoIter<T, CAP> {}

impl<T, const CAP: usize> std::iter::FusedIterator for IntoIter<T, CAP> {}

impl<T, const CAP: usize> Drop for IntoIter<T, CAP>
{
    fn drop(&mut self)
    {
        // panic safety: Set length to 0 before dropping elements.
        let index = self.index;
        let len = self.array.len();
        unsafe {
            self.array.set_len(0);
            let elements =
                slice::from_raw_parts_mut(self.array.get_unchecked_ptr(index), len - index);
            ptr::drop_in_place(elements);
        }
    }
}

impl<T: PartialEq, const CAP: usize> PartialEq for ArrayVec<T, CAP>
{
    fn eq(&self, other: &Self) -> bool { self.as_slice() == other.as_slice() }
}

impl<T: PartialEq, const CAP: usize> PartialEq<[T]> for ArrayVec<T, CAP>
{
    fn eq(&self, other: &[T]) -> bool { self.as_slice() == other }
}

impl<T, const CAP: usize> Default for ArrayVec<T, CAP>
{
    fn default() -> Self { Self::new() }
}

/// A draining iterator for `ArrayVec`.
pub struct Drain<'a, T: 'a, const CAP: usize>
{
    /// Index of tail to preserve
    tail_start: usize,
    /// Length of tail
    tail_len: usize,
    /// Current remaining range to remove
    iter: slice::Iter<'a, T>,
    vec: *mut ArrayVec<T, CAP>,
}

unsafe impl<'a, T: Sync, const CAP: usize> Sync for Drain<'a, T, CAP> {}
unsafe impl<'a, T: Send, const CAP: usize> Send for Drain<'a, T, CAP> {}

impl<'a, T: 'a, const CAP: usize> Iterator for Drain<'a, T, CAP>
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item>
    {
        self.iter
            .next()
            .map(|elt| unsafe { ptr::read(elt as *const T) })
    }

    fn size_hint(&self) -> (usize, Option<usize>) { self.iter.size_hint() }
}

impl<'a, T: 'a, const CAP: usize> DoubleEndedIterator for Drain<'a, T, CAP>
{
    fn next_back(&mut self) -> Option<Self::Item>
    {
        self.iter
            .next_back()
            .map(|elt| unsafe { ptr::read(elt as *const T) })
    }
}

impl<'a, T: 'a, const CAP: usize> ExactSizeIterator for Drain<'a, T, CAP> {}

impl<'a, T: 'a, const CAP: usize> Drop for Drain<'a, T, CAP>
{
    fn drop(&mut self)
    {
        // len is currently 0 so panicking while dropping will not cause a double drop.

        // exhaust self first
        while let Some(_) = self.next()
        {}

        if self.tail_len > 0
        {
            unsafe {
                let source_vec = &mut *self.vec;
                // memmove back untouched tail, update to new length
                let start = source_vec.len();
                let tail = self.tail_start;
                let ptr = source_vec.as_mut_ptr();
                ptr::copy(ptr.add(tail), ptr.add(start), self.tail_len);
                source_vec.set_len(start + self.tail_len);
            }
        }
    }
}


impl<T, const CAP: usize> Collection for ArrayVec<T, CAP> {}
impl<T, const CAP: usize> CollectionBijective for ArrayVec<T, CAP> {}

impl<Idx, T, const CAP: usize> Get<Idx> for ArrayVec<T, CAP>
where
    [T]: Get<Idx>,
{
    type Output = <[T] as Get<Idx>>::Output;

    #[inline(always)]
    fn get(&self, index: Idx) -> Option<&Self::Output> { Get::get(self.as_slice(), index) }

    #[track_caller]
    #[inline(always)]
    unsafe fn get_unchecked(&self, index: Idx) -> &Self::Output
    {
        unsafe { Get::get_unchecked(self.as_slice(), index) }
    }
}
impl<Idx, T, const CAP: usize> TryGet<Idx> for ArrayVec<T, CAP>
where
    [T]: TryGet<Idx>,
{
    type Error = <[T] as TryGet<Idx>>::Error;

    #[inline(always)]
    fn try_get(&self, index: Idx) -> Result<&Self::Output, Self::Error>
    {
        TryGet::try_get(self.as_slice(), index)
    }
}

impl<Idx, T, const CAP: usize> TryGetMut<Idx> for ArrayVec<T, CAP>
where
    [T]: TryGetMut<Idx>,
{
    #[inline(always)]
    fn try_get_mut(&mut self, index: Idx) -> Result<&mut Self::Output, Self::Error>
    {
        TryGetMut::try_get_mut(self.as_mut_slice(), index)
    }
}

impl<Idx, T, const CAP: usize> GetMut<Idx> for ArrayVec<T, CAP>
where
    [T]: GetMut<Idx>,
{
    #[inline(always)]
    fn get_mut(&mut self, index: Idx) -> Option<&mut Self::Output>
    {
        GetMut::get_mut(self.as_mut_slice(), index)
    }

    #[inline(always)]
    unsafe fn get_unchecked_mut(&mut self, index: Idx) -> &mut Self::Output
    {
        unsafe { GetMut::get_unchecked_mut(self.as_mut_slice(), index) }
    }
}

impl<Idx, T, const CAP: usize> GetManyMut<Idx> for ArrayVec<T, CAP>
where
    [T]: GetManyMut<Idx>,
{
    #[track_caller]
    #[inline(always)]
    unsafe fn get_many_unchecked_mut<const N: usize>(
        &mut self,
        indices: [Idx; N],
    ) -> [&mut Self::Output; N]
    {
        unsafe { GetManyMut::get_many_unchecked_mut(self.as_mut_slice(), indices) }
    }

    #[inline(always)]
    fn try_get_many_mut<const N: usize>(
        &mut self,
        indices: [Idx; N],
    ) -> Result<[&mut Self::Output; N], ManyMutError>
    {
        GetManyMut::try_get_many_mut(self.as_mut_slice(), indices)
    }

    #[inline(always)]
    fn get_many_mut<const N: usize>(&mut self, indices: [Idx; N])
    -> Option<[&mut Self::Output; N]>
    {
        GetManyMut::get_many_mut(self.as_mut_slice(), indices)
    }
}

impl<T, const CAP: usize> Length for ArrayVec<T, CAP>
{
    fn len(&self) -> usize { self.len() }
}
impl<T, const CAP: usize> SetLength for ArrayVec<T, CAP>
{
    unsafe fn set_len(&mut self, new_len: usize) {
        unsafe { self.set_len(new_len) };
    }
}
impl<T, const CAP: usize> Capacity for ArrayVec<T, CAP>
{
    fn capacity(&self) -> usize { self.capacity() }
}

impl<T, const CAP: usize> Clear for ArrayVec<T, CAP>
{
    fn clear(&mut self) {
        self.clear();
    }
}

impl<T, const CAP: usize> Truncate for ArrayVec<T, CAP>
{
    fn truncate(&mut self, len: usize) {
        self.truncate(len);
    }
}


// Discutable...
impl<T, const CAP: usize> WithCapacity for ArrayVec<T, CAP>
{
    type Param=();
    fn with_capacity_and_param(_capacity: usize, _param: Self::Param) -> Self {
        Self::new()
    }
}

#[cfg(feature = "serde")]
impl<T, const CAP: usize> serde::Serialize for ArrayVec<T, CAP>
where
    T: serde::Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeSeq;

        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for element in self.iter()
        {
            seq.serialize_element(element)?;
        }
        seq.end()
    }
}

#[cfg(feature = "serde")]
impl<'de, T, const CAP: usize> serde::Deserialize<'de> for ArrayVec<T, CAP>
where
    T: serde::Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ArrayVecVisitor<T, const CAP: usize>(std::marker::PhantomData<T>);

        impl<'de, T, const CAP: usize> serde::de::Visitor<'de> for ArrayVecVisitor<T, CAP>
        where
            T: serde::Deserialize<'de>,
        {
            type Value = ArrayVec<T, CAP>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result
            {
                write!(formatter, "a sequence with at most {} elements", CAP)
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut array_vec = ArrayVec::new();

                while let Some(element) = seq.next_element()?
                {
                    if array_vec.try_push(element).is_err()
                    {
                        return Err(serde::de::Error::invalid_length(
                            CAP + 1,
                            &format!("at most {} elements", CAP).as_str(),
                        ));
                    }
                }

                Ok(array_vec)
            }
        }

        deserializer.deserialize_seq(ArrayVecVisitor(std::marker::PhantomData))
    }
}

impl<T, const CAP: usize> From<[T; CAP]> for ArrayVec<T, CAP>
{
    fn from(array: [T; CAP]) -> Self
    {
        Self {
            values: array.map(MaybeUninit::new),
            len: CAP,
        }
    }
}

impl<T, const CAP: usize> AsRef<[T]> for ArrayVec<T, CAP>
{
    fn as_ref(&self) -> &[T] { self.as_slice() }
}

impl<T, const CAP: usize> AsMut<[T]> for ArrayVec<T, CAP>
{
    fn as_mut(&mut self) -> &mut [T] { self.as_mut_slice() }
}

impl<T, const CAP: usize> Push<T> for ArrayVec<T, CAP>
{
    type Output=usize;
    #[track_caller]
    fn push(&mut self, value: T) -> Self::Output {
        let len = self.len();
        self.push(value);
        len
    }
}
impl<T, const CAP: usize> TryPush<T> for ArrayVec<T, CAP>
{
    type Error=CapacityFullError<T>;
    fn try_push(&mut self, value: T) -> Result<Self::Output, Self::Error> {
        let len = self.len();
        self.try_push(value).map(|_| len)
    }
}
impl<T, const CAP: usize> Pop<T> for ArrayVec<T, CAP>
{
    fn pop(&mut self) -> Option<T> {
        self.pop()
    }

    fn pop_if<F>(&mut self, predicate: F) -> Option<T>
    where
        F: FnOnce(&mut T) -> bool
    {
        let Some(last) = self.last_mut() else { return None; };
        if predicate(last)
        {
            self.pop()
        }else
        {
            None
        }
    }
}

unsafe impl<T, const CAP: usize> BitZero for ArrayVec<T, CAP> where T: BitZero {}
