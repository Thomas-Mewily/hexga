// Prevision for https://smallcultfollowing.com/babysteps/blog/2025/10/07/the-handle-trait/
pub trait Handle: Clone
{
    fn clone_handle(&self) -> Self { self.clone() }
}
impl<T> Handle for core::cell::RefCell<T> where T: ?Sized + Clone {}

#[cfg(feature = "std")]
impl<T> Handle for std::rc::Rc<T> where T: ?Sized {}
#[cfg(feature = "std")]
impl<T> Handle for std::rc::Weak<T> where T: ?Sized {}
#[cfg(feature = "std")]
impl<T> Handle for std::sync::Arc<T> where T: ?Sized {}
#[cfg(feature = "std")]
impl<T> Handle for std::sync::Weak<T> where T: ?Sized {}

#[cfg(not(feature = "std"))]
impl<T> Handle for alloc::rc::Rc<T> where T: ?Sized {}
#[cfg(not(feature = "std"))]
impl<T> Handle for alloc::rc::Weak<T> where T: ?Sized {}
#[cfg(not(feature = "std"))]
impl<T> Handle for alloc::sync::Arc<T> where T: ?Sized {}
#[cfg(not(feature = "std"))]
impl<T> Handle for alloc::sync::Weak<T> where T: ?Sized {}
