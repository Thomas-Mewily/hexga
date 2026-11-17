use super::*;

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
/// Depending on the type, the output usually needs a fresh allocation (ex: [`Vec`]).
/// If the mapped type is the same type as the item, consider [`Map::map`] instead because it can often reuse the allocation.
pub trait MapInto: Map {
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
pub trait MapZip: Map
{
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
pub trait MapZipInto: MapZip + MapInto 
{
    /// Elementwise apply `f` to the corresponding items across `M` containers.
    ///
    /// # Panics
    ///
    /// May panic if `M == 0`, or if the containers shapes mismatch (ex two `Grid` of different size).
    /// 
    /// For [`Vec`], a length mismatch is fine because it zips to the shortest length.
    fn map_zip_into<F, R, const M: usize>(this: [Self; M], f: F) -> Self::WithType<R>
    where
        F: FnMut([Self::Item; M]) -> R;

    /// Combine `self` and `other` elementwise with `f`.
    ///
    /// # Panics
    ///
    /// May panic if the containers shapes mismatch (ex two `Grid` of different size).
    /// 
    /// For [`Vec`], a length mismatch is fine because it zips to the shortest length.
    fn map_with_into<F, R>(self, other: Self, mut f: F) -> Self::WithType<R>
    where
        F: FnMut(Self::Item, Self::Item) -> R,
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
        core::array::from_fn(|_| f(core::array::from_fn(|i| unsafe {
            iters[i].next().unwrap_unchecked()
        })))
    }
}
impl<T, const N: usize> MapZipInto for [T; N] {
    fn map_zip_into<F, R, const M: usize>(this: [Self; M], mut f: F) -> Self::WithType<R>
    where
        F: FnMut([T; M]) -> R,
    {
        let mut iters = this.map(|v| v.into_iter());
        core::array::from_fn(|_| f(core::array::from_fn(|i| unsafe {
            iters[i].next().unwrap_unchecked()
        })))
    }
}

impl<T> Map for Vec<T> {
    type Item = T;

    fn map<F>(self, f: F) -> Self
    where
        F: FnMut(T) -> T,
    {
        self.into_iter().map(f).collect()
    }
}
impl<T> MapInto for Vec<T> {
    type WithType<R> = Vec<R>;

    fn map_into<F, R>(self, f: F) -> Self::WithType<R>
    where
        F: FnMut(T) -> R,
    {
        self.into_iter().map(f).collect()
    }
}
impl<T> MapZip for Vec<T> {
    fn map_zip<F, const M: usize>(mut this: [Self; M], mut f: F) -> Self
    where
        F: FnMut([T; M]) -> T,
    {
        if M == 0 {
            return Vec::new();
        }
        let len = this.iter().map(Vec::len).min().unwrap_or(0);
        let mut result = core::mem::take(&mut this[0]);
        result.truncate(len);
        let mut iters = this.map(IntoIterator::into_iter);
        let base = result.as_mut_ptr();
        let mut out_ptr = base;
        let mut k = 0;
        while k < len {
            unsafe { result.set_len(k); }
            let mut args: [core::mem::MaybeUninit<T>; M] =
                [const { core::mem::MaybeUninit::uninit() }; M];
            unsafe { args[0].write(core::ptr::read(out_ptr)); }
            for j in 1..M {
                args[j].write(unsafe { iters[j].next().unwrap_unchecked() });
            }
            let args: [T; M] = unsafe { core::ptr::read(args.as_ptr() as *const [T; M]) };
            let value = f(args);
            unsafe {
                core::ptr::write(out_ptr, value);
                out_ptr = out_ptr.add(1);
            }
            k += 1;
        }
        unsafe { result.set_len(len); }
        result
    }
}
impl<T> MapZipInto for Vec<T> {
    fn map_zip_into<F, R, const M: usize>(this: [Self; M], mut f: F) -> Self::WithType<R>
    where
        F: FnMut([T; M]) -> R,
    {
        if M == 0 {
            return Vec::new();
        }
        let mut len = usize::MAX;
        let mut iters = this.map(|v| {
            len = min(v.len(), len);
            v.into_iter()
        });
        Vec::from_fn(len, |_| f(core::array::from_fn(|i| unsafe {
            iters[i].next().unwrap_unchecked()
        })))
    }
}
