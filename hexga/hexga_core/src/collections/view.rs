
/// A trait for borrowing a stable, read-only view of a container.
///
/// The view allows access to elements **without taking ownership** and
/// **without permitting mutation, insertion, or deletion**.
///
/// # Examples
///
/// ```rust
/// use hexga_core::prelude::*;
///
/// let vec = vec![1, 2, 3];
/// let view = vec.view(); // returns &[i32]
/// assert_eq!(view[0], 1);
///
/// let arr = [10, 20, 30];
/// let view = arr.view(); // returns &[i32]
/// assert_eq!(view[2], 30);
/// ```
pub trait View<'s>
{
    type View: View<'s>;
    fn as_view(&'s self) -> Self::View;
}


impl<'s,T> View<'s> for [T] where Self: 's
{
    type View = &'s [T];
    fn as_view(&'s self) -> Self::View { self }
}
impl<'s,T> View<'s> for &'s[T]
{
    type View = &'s [T];
    fn as_view(&'s self) -> Self::View { self }
}
impl<'s,T> View<'s> for &'s mut [T]
{
    type View = &'s [T];
    fn as_view(&'s self) -> Self::View { self }
}
impl<'s,T> View<'s> for Vec<T> where Self: 's
{
    type View = &'s [T];
    fn as_view(&'s self) -> Self::View { self }
}
impl<'s,T,const N:usize> View<'s> for [T;N] where Self: 's
{
    type View = &'s [T];
    fn as_view(&'s self) -> Self::View { self }
}



impl<'s,T> View<'s> for &T where T: View<'s>
{
    type View = T::View;
    fn as_view(&'s self) -> Self::View { (*self).as_view() }
}
impl<'s,T> View<'s> for &mut T where T: View<'s>
{
    type View = T::View;
    fn as_view(&'s self) -> Self::View { (**self).as_view() }
}