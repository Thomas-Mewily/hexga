use super::*;
use ::core::alloc::Allocator;

// Update it and add `try_map` once stabilized ?
// https://doc.rust-lang.org/std/primitive.array.html#method.try_map

/// Map item inside self.
///
/// Similar to [`Iterator::map`], but for container-like types that keep their shape/type.
/// Best effort is made to reuses the existing allocation when possible. That why [`Map::map`] is easier to use/write than [`MapInto::map_into`].
pub trait Map: Sized {
    type Item;

    /// Apply `f` to each item.
    fn map<F>(self, f: F) -> Self
    where
        F: FnMut(Self::Item) -> Self::Item;
}

/// Map item inside self, while allowing its type to change.
///
/// A borrowed container can implement this trait to produce an owned output.
/// Depending on the type, the output usually needs a fresh allocation (ex: [`Vec`]).
/// If the mapped type is the same type as the item, consider [`Map::map`] instead because it can often reuse the allocation.
pub trait MapInto: Sized {
    type Item;
    type WithType<R>: MapInto<Item = R>;

    /// Elementwise apply `f` to each item, while allowing the item type to change.
    fn map_into<F, R>(self, f: F) -> Self::WithType<R>
    where
        F: FnMut(Self::Item) -> R;
}

/// Map item from multiple self.
///
/// Similar to [`Iterator::map`], but for container-like types that keep their shape/type.
/// Best effort is made to reuses the existing allocation when possible. That why [`MapZip::map_zip`] is easier to use/write than [`MapZipInto::map_zip_into`].
pub trait MapZip: Map {
    /// Elementwise apply `f` to the corresponding items across `M` containers.
    ///
    /// # Panics
    ///
    /// May panic if `M == 0`, or if the containers shapes mismatch (ex two `Grid` of different size).
    ///
    /// For [`Vec`], a length mismatch is fine because it zips to the shortest length.
    fn map_zip<F, const M: usize>(this: [Self; M], f: F) -> Self
    where
        F: FnMut([Self::Item; M]) -> Self::Item;

    /// Elementwise apply `f` to `self` and `other`.
    ///
    /// # Panics
    ///
    /// May panic if the containers shapes mismatch (ex two `Grid` of different size).
    ///
    /// For [`Vec`], a length mismatch is fine because it zips to the shortest length.
    fn map_with<F>(self, other: Self, mut f: F) -> Self
    where
        F: FnMut(Self::Item, Self::Item) -> Self::Item,
    {
        Self::map_zip([self, other], |[a, b]| f(a, b))
    }
}

/// Map item from multiple self, while allowing its type to change.
pub trait MapZipInto: MapZip + MapInto<Item = <Self as Map>::Item> {
    /// Elementwise apply `f` to the corresponding items across `M` containers.
    ///
    /// # Panics
    ///
    /// May panic if `M == 0`, or if the containers shapes mismatch (ex two `Grid` of different size).
    ///
    /// For [`Vec`], a length mismatch is fine because it zips to the shortest length.
    fn map_zip_into<F, R, const M: usize>(this: [Self; M], f: F) -> Self::WithType<R>
    where
        F: FnMut([<Self as Map>::Item; M]) -> R;

    /// Combine `self` and `other` elementwise with `f`.
    ///
    /// # Panics
    ///
    /// May panic if the containers shapes mismatch (ex two `Grid` of different size).
    ///
    /// For [`Vec`], a length mismatch is fine because it zips to the shortest length.
    fn map_with_into<F, R>(self, other: Self, mut f: F) -> Self::WithType<R>
    where
        F: FnMut(<Self as Map>::Item, <Self as Map>::Item) -> R,
    {
        Self::map_zip_into([self, other], |[a, b]| f(a, b))
    }
}

impl<T, const N: usize> Map for [T; N] {
    type Item = T;

    fn map<F>(self, mut f: F) -> Self
    where
        F: FnMut(T) -> T,
    {
        self.map(|v| f(v))
    }
}
impl<T, const N: usize> MapInto for [T; N] {
    type Item = T;
    type WithType<R> = [R; N];

    fn map_into<F, R>(self, f: F) -> Self::WithType<R>
    where
        F: FnMut(T) -> R,
    {
        self.map(f)
    }
}
impl<T, const N: usize> MapZip for [T; N] {
    fn map_zip<F, const M: usize>(this: [Self; M], mut f: F) -> Self
    where
        F: FnMut([T; M]) -> T,
    {
        let mut iters = this.map(|v| v.into_iter());
        core::array::from_fn(|_| f(core::array::from_fn(|i| unsafe { iters[i].next().unwrap_unchecked() })))
    }
}
impl<T, const N: usize> MapZipInto for [T; N] {
    fn map_zip_into<F, R, const M: usize>(this: [Self; M], mut f: F) -> Self::WithType<R>
    where
        F: FnMut([T; M]) -> R,
    {
        let mut iters = this.map(|v| v.into_iter());
        core::array::from_fn(|_| f(core::array::from_fn(|i| unsafe { iters[i].next().unwrap_unchecked() })))
    }
}

impl<T, A: Allocator + Clone> Map for Vec<T, A> {
    type Item = T;

    fn map<F>(self, f: F) -> Self
    where
        F: FnMut(T) -> T,
    {
        self.map_into(f)
    }
}
impl<T, A: Allocator + Clone> MapInto for Vec<T, A> {
    type Item = T;
    type WithType<R> = Vec<R, A>;

    fn map_into<F, R>(self, f: F) -> Self::WithType<R>
    where
        F: FnMut(T) -> R,
    {
        let mut result = Vec::with_capacity_in(self.len(), self.allocator().clone());
        result.extend(self.into_iter().map(f));
        result
    }
}
impl<T: Copy, A: Allocator + Clone> MapInto for &Vec<T, A> {
    type Item = T;
    type WithType<R> = Vec<R, A>;

    fn map_into<F, R>(self, f: F) -> Self::WithType<R>
    where
        F: FnMut(T) -> R,
    {
        let mut result = Vec::with_capacity_in(self.len(), self.allocator().clone());
        result.extend(self.iter().copied().map(f));
        result
    }
}
impl<T, A: Allocator + Clone + Default> MapZip for Vec<T, A> {
    fn map_zip<F, const M: usize>(this: [Self; M], f: F) -> Self
    where
        F: FnMut([T; M]) -> T,
    {
        Self::map_zip_into(this, f)
    }
}
impl<T, A: Allocator + Clone + Default> MapZipInto for Vec<T, A> {
    fn map_zip_into<F, R, const M: usize>(this: [Self; M], mut f: F) -> Self::WithType<R>
    where
        F: FnMut([T; M]) -> R,
    {
        let Some(first) = this.first() else {
            return Vec::new_in(A::default());
        };
        let len = this.iter().map(Vec::len).min().unwrap_or(0);
        let mut result = Vec::with_capacity_in(len, first.allocator().clone());
        let mut iters = this.map(IntoIterator::into_iter);
        for _ in 0..len {
            result.push(f(core::array::from_fn(|i| iters[i].next().unwrap())));
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::alloc::System;

    #[test]
    fn maps_and_zips_vectors_with_custom_allocators() {
        let allocator = System;
        let mut values = Vec::new_in(&allocator);
        values.extend([1, 2]);
        let mapped = values.map_into(|value| value.to_string());
        assert_eq!(mapped.as_slice(), ["1", "2"]);
        assert!(core::ptr::eq(*mapped.allocator(), &allocator));

        let mut left = Vec::new_in(System);
        left.extend([1, 2, 3]);
        let mut right = Vec::new_in(System);
        right.extend([4, 5]);
        let zipped = left.map_with(right, |a, b| a + b);
        assert_eq!(zipped.as_slice(), [5, 7]);
        assert!(Vec::<i32, System>::map_zip([], |_| unreachable!()).is_empty());
    }
}
