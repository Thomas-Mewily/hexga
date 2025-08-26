use crate::*;

pub mod prelude
{
    
}

pub trait Scoped<T>
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