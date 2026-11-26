#![no_std]
#![doc = include_str!("../README.md")]

/// Runs an operation in a scope.
pub trait Scoped
{
    fn begin(&mut self);
    fn scope<F, R>(&mut self, f: F) -> R
    where
        F: FnOnce() -> R,
    {
        self.begin();
        let r = f();
        self.end();
        r
    }
    fn end(&mut self);
}
impl<S> ScopedWith<()> for S
where
    S: Scoped,
{
    fn begin(&mut self, _value: &()) { Scoped::begin(self); }

    fn end(&mut self) { Scoped::end(self); }
}

/// Runs an operation in a scope associated with a value.
pub trait ScopedWith<T = ()>
{
    fn begin(&mut self, value: &T);
    fn scope<F, R>(&mut self, value: T, f: F) -> R
    where
        F: FnOnce(T) -> R,
    {
        self.begin(&value);
        let r = f(value);
        self.end();
        r
    }
    fn end(&mut self);
}

pub trait Enable
{
    fn is_enable(&self) -> bool;
}

pub trait SetEnable
{
    fn set_enable(&mut self, enable: bool);
}

pub trait EnableExtension: Enable
{
    fn is_disable(&self) -> bool { !self.is_enable() }
}

impl<T: Enable + ?Sized> EnableExtension for T {}

pub trait SetEnableExtension: SetEnable
{
    fn enable(&mut self) { self.set_enable(true); }

    fn disable(&mut self) { self.set_enable(false); }

    fn set_disable(&mut self, disable: bool) { self.set_enable(!disable); }
}

impl<T: SetEnable + ?Sized> SetEnableExtension for T {}
