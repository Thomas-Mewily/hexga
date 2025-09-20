use super::*;


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