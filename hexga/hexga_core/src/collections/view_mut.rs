use super::*;

/// A trait for borrowing a stable, mutable view of a container.
///
/// The view allows access to elements **without taking ownership** and
/// **without permitting insertion or deletion**, but **mutation of existing elements is allowed**.
///
/// # Examples
///
/// ```rust
/// use hexga_core::prelude::*;
///
/// let mut vec = vec![1, 2, 3];
/// let mut view = vec.view_mut(); // returns &mut [i32]
/// view[0] += 10;
/// assert_eq!(vec[0], 11);
/// ```
pub trait ViewMut : View
{
    type ViewMut<'v>: ViewMut where Self: 'v;
    fn view_mut<'s>(&'s mut self) -> Self::ViewMut<'s>;
}


impl<T> ViewMut for [T]
{
    type ViewMut<'v> = &'v mut [T] where Self: 'v;
    fn view_mut<'s>(&'s mut self) -> Self::ViewMut<'s> { self }
}
impl<'b,T> ViewMut for &'b mut [T]
{
    type ViewMut<'v> = &'v mut [T] where Self: 'v;
    fn view_mut<'s>(&'s mut self) -> Self::ViewMut<'s> { self }
}
impl<T> ViewMut for Vec<T>
{
    type ViewMut<'v> = &'v mut [T] where Self: 'v;
    fn view_mut<'s>(&'s mut self) -> Self::ViewMut<'s> { self }
}
impl<T,const N:usize> ViewMut for [T;N]
{
    type ViewMut<'v> = &'v mut [T] where Self: 'v;
    fn view_mut<'s>(&'s mut self) -> Self::ViewMut<'s> { self }
}
