use super::*;

pub mod prelude
{
    pub use super::{GetProperty,SetProperty};
}

pub trait GetProperty<T>
{
    // I don't want to use the name get, too generic and overlap with the collection get() method
    fn get_property(&self) -> T;
}
pub trait SetProperty<T>
{
    fn set_property(&mut self, value: T) -> &mut Self;
}

impl<T> GetProperty<T> for T where T:Copy { fn get_property(&self) -> T { *self } }
impl<T> SetProperty<T> for T { fn set_property(&mut self, value: T) -> &mut Self { *self = value; self } }

pub trait GetterAndSetter<T> : GetProperty<T> + SetProperty<T> {}
impl<T,S> GetterAndSetter<T> for S where S: GetProperty<T> + SetProperty<T> {}