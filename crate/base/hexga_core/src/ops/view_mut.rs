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
// The new trait
pub trait ViewMut: View {
    type ViewMut<'a>: ViewMut where Self: 'a;
    fn as_mut_view<'a, 'b>(&'a mut self) -> Self::ViewMut<'b> where 'a: 'b;
}

// Implementations
impl<T> ViewMut for [T] {
    type ViewMut<'a> = &'a mut [T] where Self: 'a;
    fn as_mut_view<'a, 'b>(&'a mut self) -> Self::ViewMut<'b> where 'a: 'b { self }
}

impl<T> ViewMut for &mut [T] {
    type ViewMut<'a> = &'a mut [T] where Self: 'a;
    fn as_mut_view<'a, 'b>(&'a mut self) -> Self::ViewMut<'b> where 'a: 'b { self }
}

impl<T> ViewMut for Vec<T> {
    type ViewMut<'a> = &'a mut [T] where Self: 'a;
    fn as_mut_view<'a, 'b>(&'a mut self) -> Self::ViewMut<'b> where 'a: 'b { self }
}

impl<T, const N: usize> ViewMut for [T; N] {
    type ViewMut<'a> = &'a mut [T] where Self: 'a;
    fn as_mut_view<'a, 'b>(&'a mut self) -> Self::ViewMut<'b> where 'a: 'b { self }
}

impl<T: ViewMut> ViewMut for &mut T {
    type ViewMut<'a> = T::ViewMut<'a> where Self: 'a;
    fn as_mut_view<'a, 'b>(&'a mut self) -> Self::ViewMut<'b> where 'a: 'b { (*self).as_mut_view() }
}


/*
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
impl<'s,T> ViewMut<'s> for &'s mut [T] where Self: 's
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
    */