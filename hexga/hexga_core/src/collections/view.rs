
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
    type View<'v>: View where Self: 'v;
    fn view<'s>(&'s self) -> Self::View<'s>;
}


impl<T> View for [T]
{
    type View<'v> = &'v [T] where Self: 'v;
    fn view<'s>(&'s self) -> Self::View<'s> { self }
}
impl<'b,T> View for &'b[T]
{
    type View<'v> = &'v [T] where Self: 'v;
    fn view<'s>(&'s self) -> Self::View<'s> { self }
}
impl<'b,T> View for &'b mut[T]
{
    type View<'v> = &'v [T] where Self: 'v;
    fn view<'s>(&'s self) -> Self::View<'s> { self }
}
impl<T> View for Vec<T>
{
    type View<'v> = &'v [T] where Self: 'v;
    fn view<'s>(&'s self) -> Self::View<'s> { self }
}
impl<T,const N:usize> View for [T;N]
{
    type View<'v> = &'v [T] where Self: 'v;
    fn view<'s>(&'s self) -> Self::View<'s> { self }
}