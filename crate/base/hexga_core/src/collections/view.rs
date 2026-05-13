
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
pub trait View
{
    type View<'a>: View where Self: 'a;
    fn as_view<'a>(&'a self) -> Self::View<'a>;
}

impl<T> View for [T] {
    type View<'a> = &'a [T] where Self: 'a;
    fn as_view<'a>(&'a self) -> Self::View<'a> { self }
}

impl<T> View for &[T] {
    type View<'a> = &'a [T] where Self: 'a;
    fn as_view<'a>(&'a self) -> Self::View<'a> { self }
}

impl<T> View for &mut [T] {
    type View<'a> = &'a [T] where Self: 'a;
    fn as_view<'a>(&'a self) -> Self::View<'a> { self }
}

impl<T> View for Vec<T> {
    type View<'a> = &'a [T] where Self: 'a;
    fn as_view<'a>(&'a self) -> Self::View<'a> { self }
}

impl<T, const N: usize> View for [T; N] {
    type View<'a> = &'a [T] where Self: 'a;
    fn as_view<'a>(&'a self) -> Self::View<'a> { self }
}

impl<T: View> View for &T {
    type View<'a> = T::View<'a> where Self: 'a;
    fn as_view<'a>(&'a self) -> Self::View<'a> { (*self).as_view() }
}

impl<T: View> View for &mut T {
    type View<'a> = T::View<'a> where Self: 'a;
    fn as_view<'a>(&'a self) -> Self::View<'a> { (**self).as_view() }
}
/*
impl<'s,T> View<'s> for [T] where Self: 's
{
    type View<'v> = &'v [T] where Self: 'v;
    fn as_view(&'s self) -> Self::View<'s> { self }
}
impl<'s,T> View<'s> for &'s[T] where Self: 's
{
    type View<'v> = &'v [T] where Self: 'v;
    fn as_view(&'s self) -> Self::View<'s> { self }
}
impl<'s,T> View<'s> for &'s mut [T] where Self: 's
{
    type View<'v> = &'v [T] where Self: 'v;
    fn as_view(&'s self) -> Self::View<'s> { self }
}
impl<'s,T> View<'s> for Vec<T> where Self: 's
{
    type View<'v> = &'v [T] where Self: 'v;
    fn as_view(&'s self) -> Self::View<'s> { self }
}
impl<'s,T,const N:usize> View<'s> for [T;N] where Self: 's
{
    type View<'v> = &'v [T] where Self: 'v;
    fn as_view(&'s self) -> Self::View<'s> { self }
}



impl<'s,T> View<'s> for &T where Self: 's, T: View<'s>
{
    type View<'v> = T::View<'v> where Self: 'v;
    fn as_view(&'s self) -> Self::View<'s> { (*self).as_view() }
}
impl<'s,T> View<'s> for &mut T where Self: 's, T: View<'s>
{
    type View<'v> = T::View<'v> where Self: 'v;
    fn as_view(&'s self) -> Self::View<'s> { (**self).as_view() }
}
*/