
// Todo: Make a Constructor trait ?

pub trait Getter<T>
{
    fn get(&self) -> T;
}
impl<T> Getter<T> for T where T:Copy
{
    fn get(&self) -> T {*self }
}

pub trait Setter<T>
{
    fn set(&mut self, val: T) -> &mut Self;
}


pub trait GetterSetter<T> : Getter<T> + Setter<T>{}
impl<S,T> GetterSetter<T> for S where S:Getter<T> +Setter<T>{}

// Based on [GGEZ Has trait](https://docs.rs/ggez/latest/ggez/context/trait.Has.html)
pub trait HasRef<T>
{
    fn retrive(&self) -> &T;
}
impl<T> HasRef<T> for T  { fn retrive(&self) -> &T { self } }

// Based on [GGEZ HasMut trait](https://docs.rs/ggez/latest/ggez/context/trait.HasMut.html)
pub trait HasMut<T>
{
    fn retrive_mut(&mut self) -> &mut T;
}
impl<T> HasMut<T> for T  { fn retrive_mut(&mut self) -> &mut T { self } }



pub trait Has<T> : HasRef<T> + HasMut<T>{}
impl<S,T> Has<T> for S where S:HasRef<T> +HasMut<T>{}