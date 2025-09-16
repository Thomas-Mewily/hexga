
pub trait Getter<T>
{
    fn get(&self) -> T;
}

pub trait Setter<T>
{
    fn set(&mut self, val: T) -> &mut Self;
}

// Based on (GGEZ Has trait)[https://docs.rs/ggez/latest/ggez/context/trait.Has.html] 
pub trait Has<T>
{
    fn retrive(&self) -> &T;
}
impl<T> Has<T> for T  { fn retrive(&self) -> &T { self } }

// Based on (GGEZ HasMut trait)[https://docs.rs/ggez/latest/ggez/context/trait.HasMut.html] 
pub trait HasMut<T>
{
    fn retrive_mut(&mut self) -> &mut T;
}
impl<T> HasMut<T> for T  { fn retrive_mut(&mut self) -> &mut T { self } }
