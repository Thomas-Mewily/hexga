use super::*;

// Prevision for https://smallcultfollowing.com/babysteps/blog/2025/10/07/the-handle-trait/
pub trait Handle: Clone
{
    fn clone_handle(&self) -> Self { self.clone() }
}
impl<T> Handle for core::cell::RefCell<T> where T: ?Sized + Clone {}

impl<T> Handle for rc::Rc<T> where T: ?Sized {}
impl<T> Handle for rc::Weak<T> where T: ?Sized {}
impl<T> Handle for sync::Arc<T> where T: ?Sized {}
impl<T> Handle for sync::Weak<T> where T: ?Sized {}
