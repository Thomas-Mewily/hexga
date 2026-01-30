// Prevision for https://smallcultfollowing.com/babysteps/blog/2025/10/07/the-handle-trait/
pub trait Handle: Clone
{
    fn clone_handle(&self) -> Self { self.clone() }
}
impl<T> Handle for std::cell::RefCell<T> where T: ?Sized + Clone {}
impl<T> Handle for std::rc::Rc<T> where T: ?Sized {}
impl<T> Handle for std::rc::Weak<T> where T: ?Sized {}
impl<T> Handle for std::sync::Arc<T> where T: ?Sized {}
impl<T> Handle for std::sync::Weak<T> where T: ?Sized {}
