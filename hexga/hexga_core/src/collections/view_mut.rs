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
pub trait ViewMut<'s> : View<'s> where Self: 's
{
    type ViewMut<'v>: ViewMut<'v> where Self: 'v;
    fn as_mut_view(&'s mut self) -> Self::ViewMut<'s>;
}


impl<'s,T> ViewMut<'s> for [T] where Self: 's
{
    type ViewMut<'v> = &'v mut [T] where Self: 'v;
    fn as_mut_view(&'s mut self) -> Self::ViewMut<'s> { self }
}
impl<'s,'b,T> ViewMut<'s> for &'b mut [T] where Self: 's
{
    type ViewMut<'v> = &'v mut [T] where Self: 'v;
    fn as_mut_view(&'s mut self) -> Self::ViewMut<'s> { self }
}
impl<'s,T> ViewMut<'s> for Vec<T> where Self: 's
{
    type ViewMut<'v> = &'v mut [T] where Self: 'v;
    fn as_mut_view(&'s mut self) -> Self::ViewMut<'s> { self }
}
impl<'s,T,const N:usize> ViewMut<'s> for [T;N] where Self: 's
{
    type ViewMut<'v> = &'v mut [T] where Self: 'v;
    fn as_mut_view(&'s mut self) -> Self::ViewMut<'s> { self }
}

impl<'s,T> ViewMut<'s> for &mut T where Self: 's, T: ViewMut<'s> + View<'s>
{
    type ViewMut<'v> = T::ViewMut<'v> where Self: 'v;
    fn as_mut_view(&'s mut self) -> Self::ViewMut<'s> { (*self).as_mut_view() }
}