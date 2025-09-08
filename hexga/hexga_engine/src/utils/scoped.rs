use super::*;

pub struct Draw;
pub struct Update;

pub trait ScopedDraw : Scoped<Draw>
{
    fn begin_draw(&mut self) { self.begin(); }
    fn end_draw(&mut self) { self.end(); }
    fn scoped_draw<F>(&mut self, f: F) where F: FnOnce() { self.scope(f) }
}
impl<T> ScopedDraw for T where T: Scoped<Draw>{}
pub trait ScopedUpdate : Scoped<Update>
{
    fn begin_update(&mut self) { self.begin(); }
    fn end_update(&mut self) { self.end(); }
    fn scoped_update<F>(&mut self, f: F) where F: FnOnce() { self.scope(f) }
}
impl<T> ScopedUpdate for T where T: Scoped<Update>{}

pub mod prelude
{
    pub use super::{Update,ScopedUpdate,Draw,ScopedDraw};
}