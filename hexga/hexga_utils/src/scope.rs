use crate::*;

pub mod prelude
{
    pub use super::{Scoped,ScopedWith};
}

pub trait Scoped<T=()>
{
    fn begin(&mut self);
    fn scope<F>(&mut self, f : F) where F : FnOnce()
    {
        self.begin();
        f();
        self.end();
    }
    fn end(&mut self);
}

pub trait ScopedWith<T=()>
{
    fn begin(&mut self, value : T);
    fn scope<F>(&mut self, value : T, f : F) where F : FnOnce()
    {
        self.begin(value);
        f();
        self.end();
    }
    fn end(&mut self);
}