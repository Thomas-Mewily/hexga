use std::fmt::Debug;

pub mod prelude
{
    pub use super::{LazyAccess, LazyFnMutValue, LazyFnOnceValue, LazyFnValue, LazyNew};
}

pub trait LazyAccess<T, F>
{
    fn is_init(&self) -> bool { self.observe().is_some() }
    fn into_value(self) -> T;

    fn as_ref(&mut self) -> &T { self.as_mut() }
    fn as_mut(&mut self) -> &mut T;

    fn observe(&self) -> Option<&T>;
    fn observe_mut(&mut self) -> Option<&mut T>;
}
pub trait LazyNew<T, F>: LazyAccess<T, F>
{
    fn new(init: F) -> Self;
}
pub trait LazyWithValue<T, F>: LazyNew<T, F>
{
    fn with_value(value: T) -> Self;
}

pub struct LazyFnOnceValue<T, F>
where
    F: FnOnce() -> T,
{
    inner: LazyFnOnceValueInner<T, F>,
}
impl<T> Debug for LazyFnOnceValue<T, fn() -> T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "{:?}", &self.inner)
    }
}
impl<T> Default for LazyFnOnceValue<T, fn() -> T>
where
    T: Default,
{
    fn default() -> Self
    {
        Self {
            inner: LazyFnOnceValueInner::default(),
        }
    }
}
impl<T, F> From<T> for LazyFnOnceValue<T, F>
where
    F: FnOnce() -> T,
{
    fn from(value: T) -> Self
    {
        Self {
            inner: LazyFnOnceValueInner::from(value),
        }
    }
}
impl<T, F> LazyNew<T, F> for LazyFnOnceValue<T, F>
where
    F: FnOnce() -> T,
{
    fn new(init: F) -> Self
    {
        Self {
            inner: LazyFnOnceValueInner::new(init),
        }
    }
}
impl<T, F> LazyWithValue<T, F> for LazyFnOnceValue<T, F>
where
    F: FnOnce() -> T,
{
    fn with_value(value: T) -> Self
    {
        Self {
            inner: LazyFnOnceValueInner::with_value(value),
        }
    }
}
impl<T, F> LazyAccess<T, F> for LazyFnOnceValue<T, F>
where
    F: FnOnce() -> T,
{
    fn into_value(self) -> T { self.inner.into_value() }

    fn as_mut(&mut self) -> &mut T { self.inner.as_mut() }

    fn observe(&self) -> Option<&T> { self.inner.observe() }

    fn observe_mut(&mut self) -> Option<&mut T> { self.inner.observe_mut() }
}

#[derive(Debug)]
pub(crate) enum LazyFnOnceValueInner<T, F>
where
    F: FnOnce() -> T,
{
    Waiting(Option<F>),
    Ready(T),
}
impl<T> Default for LazyFnOnceValueInner<T, fn() -> T>
where
    T: Default,
{
    fn default() -> Self { Self::new(T::default) }
}
impl<T, F> From<T> for LazyFnOnceValueInner<T, F>
where
    F: FnOnce() -> T,
{
    fn from(value: T) -> Self { Self::Ready(value) }
}
impl<T, F> LazyNew<T, F> for LazyFnOnceValueInner<T, F>
where
    F: FnOnce() -> T,
{
    fn new(init: F) -> Self { Self::Waiting(Some(init)) }
}
impl<T, F> LazyWithValue<T, F> for LazyFnOnceValueInner<T, F>
where
    F: FnOnce() -> T,
{
    fn with_value(value: T) -> Self { Self::Ready(value) }
}
impl<T, F> LazyAccess<T, F> for LazyFnOnceValueInner<T, F>
where
    F: FnOnce() -> T,
{
    fn as_mut(&mut self) -> &mut T
    {
        match self
        {
            LazyFnOnceValueInner::Ready(v) => v,
            LazyFnOnceValueInner::Waiting(opt) =>
            {
                let f = opt.take().expect("should be defined");
                let v = f();
                *self = LazyFnOnceValueInner::Ready(v);
                match self
                {
                    LazyFnOnceValueInner::Ready(v) => v,
                    _ => unreachable!(),
                }
            }
        }
    }

    fn into_value(mut self) -> T
    {
        self.as_mut();
        match self
        {
            LazyFnOnceValueInner::Waiting(_) => unreachable!(),
            LazyFnOnceValueInner::Ready(val) => val,
        }
    }

    fn observe(&self) -> Option<&T>
    {
        match self
        {
            LazyFnOnceValueInner::Waiting(_) => None,
            LazyFnOnceValueInner::Ready(v) => Some(v),
        }
    }

    fn observe_mut(&mut self) -> Option<&mut T>
    {
        match self
        {
            LazyFnOnceValueInner::Waiting(_) => None,
            LazyFnOnceValueInner::Ready(v) => Some(v),
        }
    }
}

pub struct LazyFnValue<T, F>
where
    F: Fn() -> T,
{
    init: F,
    value: Option<T>,
}
impl<T> Debug for LazyFnValue<T, fn() -> T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "{:?}", &self.value)
    }
}
impl<T> Default for LazyFnValue<T, fn() -> T>
where
    T: Default,
{
    fn default() -> Self
    {
        Self {
            init: Default::default,
            value: None,
        }
    }
}
impl<T, F> LazyNew<T, F> for LazyFnValue<T, F>
where
    F: Fn() -> T,
{
    fn new(init: F) -> Self { Self { init, value: None } }
}
impl<T, F> LazyAccess<T, F> for LazyFnValue<T, F>
where
    F: Fn() -> T,
{
    fn into_value(self) -> T
    {
        match self.value
        {
            Some(v) => v,
            None => (self.init)(),
        }
    }

    fn as_mut(&mut self) -> &mut T
    {
        if self.value.is_none()
        {
            let val = (self.init)();
            self.value = Some(val);
        }

        match self.value.as_mut()
        {
            Some(v) => v,
            None => unreachable!(),
        }
    }

    fn observe(&self) -> Option<&T> { self.value.as_ref() }

    fn observe_mut(&mut self) -> Option<&mut T> { self.value.as_mut() }
}

pub struct LazyFnMutValue<T, F>
where
    F: FnMut() -> T,
{
    init: F,
    value: Option<T>,
}
impl<T> Debug for LazyFnMutValue<T, fn() -> T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "{:?}", &self.value)
    }
}
impl<T> Default for LazyFnMutValue<T, fn() -> T>
where
    T: Default,
{
    fn default() -> Self
    {
        Self {
            init: Default::default,
            value: None,
        }
    }
}
impl<T, F> LazyNew<T, F> for LazyFnMutValue<T, F>
where
    F: FnMut() -> T,
{
    fn new(init: F) -> Self { Self { init, value: None } }
}
impl<T, F> LazyAccess<T, F> for LazyFnMutValue<T, F>
where
    F: FnMut() -> T,
{
    fn into_value(mut self) -> T
    {
        match self.value
        {
            Some(v) => v,
            None => (self.init)(),
        }
    }

    fn as_mut(&mut self) -> &mut T
    {
        if self.value.is_none()
        {
            let val = (self.init)();
            self.value = Some(val);
        }

        match self.value.as_mut()
        {
            Some(v) => v,
            None => unreachable!(),
        }
    }

    fn observe(&self) -> Option<&T> { self.value.as_ref() }

    fn observe_mut(&mut self) -> Option<&mut T> { self.value.as_mut() }
}
