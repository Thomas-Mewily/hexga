use std::fmt::Debug;


pub mod prelude
{
    pub use super::{LazyValue,LazyAccess,LazyNew};
}

pub trait LazyAccess<T,F> : From<T>
    where F: FnOnce() -> T
{
    fn is_init(&self) -> bool { self.observe().is_some() }
    fn into_value(self) -> T;

    fn as_ref(&mut self) -> &T { self.as_mut() }
    fn as_mut(&mut self) -> &mut T;

    fn observe(&self) -> Option<&T>;
    fn observe_mut(&mut self) -> Option<&mut T>;
}

pub trait LazyNew<T, F> : LazyAccess<T,F>
    where F: FnOnce() -> T,
{
    fn new(init: F) -> Self;
    fn with_value(value: T) -> Self;
}




pub struct LazyValue<T,F>
    where F: FnOnce() -> T
{
    inner: LazyValueInner<T,F>
}
impl<T> Debug for LazyValue<T, fn() -> T>
    where T: Debug
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", &self.inner)
    }
}
impl<T> Default for LazyValue<T, fn() -> T>
    where T: Default
{
    fn default() -> Self {
        Self { inner: LazyValueInner::default() }
    }
}
impl<T,F> From<T> for LazyValue<T,F> where F: FnOnce() -> T
{
    fn from(value: T) -> Self {
        Self { inner: LazyValueInner::from(value) }
    }
}
impl<T,F> LazyNew<T,F> for LazyValue<T,F>
    where F: FnOnce() -> T
{
    fn new(init: F) -> Self {
        Self { inner: LazyValueInner::new(init) }
    }

    fn with_value(value: T) -> Self {
        Self { inner: LazyValueInner::with_value(value) }
    }
}
impl<T,F> LazyAccess<T,F> for LazyValue<T,F>
    where F: FnOnce() -> T
{
    fn into_value(self) -> T {
        self.inner.into_value()
    }

    fn as_mut(&mut self) -> &mut T {
        self.inner.as_mut()
    }

    fn observe(&self) -> Option<&T> {
        self.inner.observe()
    }

    fn observe_mut(&mut self) -> Option<&mut T> {
        self.inner.observe_mut()
    }
}



#[derive(Debug)]
pub(crate) enum LazyValueInner<T,F>
    where F: FnOnce() -> T
{
    Waiting(Option<F>),
    Ready(T),
}
impl<T> Default for LazyValueInner<T,fn() -> T>
    where T: Default
{
    fn default() -> Self {
        Self::new(T::default)
    }
}
impl<T,F> From<T> for LazyValueInner<T,F> where F: FnOnce() -> T
{
    fn from(value: T) -> Self {
        Self::Ready(value)
    }
}
impl<T,F> LazyNew<T,F> for LazyValueInner<T,F>
    where F: FnOnce() -> T
{
    fn new(init: F) -> Self { Self::Waiting(Some(init)) }
    fn with_value(value: T) -> Self{ Self::Ready(value) }
}
impl<T,F> LazyAccess<T,F> for LazyValueInner<T,F>
    where F: FnOnce() -> T
{
    fn as_mut(&mut self) -> &mut T
    {
        match self {
            LazyValueInner::Ready(v) => v,
            LazyValueInner::Waiting(opt) => {
                let f = opt.take().expect("should be defined");
                let v = f();
                *self = LazyValueInner::Ready(v);
                match self {
                    LazyValueInner::Ready(v) => v,
                    _ => unreachable!(),
                }
            }
        }
    }

    fn into_value(mut self) -> T {
        self.as_mut();
        match self
        {
            LazyValueInner::Waiting(_) => unreachable!(),
            LazyValueInner::Ready(val) => val,
        }
    }

    fn observe(&self) -> Option<&T> {
        match self
        {
            LazyValueInner::Waiting(_) => None,
            LazyValueInner::Ready(v) => Some(v),
        }
    }

    fn observe_mut(&mut self) -> Option<&mut T> {
        match self
        {
            LazyValueInner::Waiting(_) => None,
            LazyValueInner::Ready(v) => Some(v),
        }
    }
}

