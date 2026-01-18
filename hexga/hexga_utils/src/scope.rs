use super::*;

pub mod prelude
{
    pub use super::{Scoped,ScopedWith};
}

pub trait Scoped
{
    fn begin(&mut self);
    fn scope<F,R>(&mut self, f : F) -> R where F : FnOnce() -> R
    {
        self.begin();
        let r = f();
        self.end();
        r
    }
    fn end(&mut self);
}
impl<S> ScopedWith<()> for S where S: Scoped
{
    fn begin(&mut self, _value : &()) {
        Scoped::begin(self);
    }

    fn end(&mut self) {
        Scoped::end(self);
    }
}

pub trait ScopedWith<T=()>
{
    fn begin(&mut self, value : &T);
    fn scope<F,R>(&mut self, value : T, f : F) -> R where F : FnOnce(T) -> R
    {
        self.begin(&value);
        let r = f(value);
        self.end();
        r
    }
    fn end(&mut self);
}