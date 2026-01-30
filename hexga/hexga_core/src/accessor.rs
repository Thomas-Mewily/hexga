// Todo: Make a Constructor trait ?

/*
pub trait Has<'a,T>
{
    fn retrieve(&'a self) -> T;
}
impl<'a,T> Has<'a,T> for T where T:Copy
{
    fn retrieve(&'a self) -> T { *self }
}
*/

pub trait Has<T>
{
    fn retrieve(&self) -> T;
}
impl<T> Has<T> for T
where
    T: Copy,
{
    fn retrieve(&self) -> T { *self }
}

/*
pub trait Setter<T>
{
    fn set(&mut self, val: T) -> &mut Self;
}
impl<T> Setter<T> for T
{
    fn set(&mut self, val: T) -> &mut Self {
        *self = val;
        self
    }
}
*/

/*
pub trait With<T>
{
    fn with(self, value: T);
}
*/

//pub trait GetterSetter<T> : Has<T> + Setter<T>{}
//impl<S,T> GetterSetter<T> for S where S:Has<T> +Setter<T>{}

// Based on [GGEZ Has trait](https://docs.rs/ggez/latest/ggez/context/trait.Has.html)
pub trait HasRef<T>
{
    fn retrive_ref(&self) -> &T;
}
impl<T> HasRef<T> for T
{
    fn retrive_ref(&self) -> &T { self }
}

// Based on [GGEZ HasMut trait](https://docs.rs/ggez/latest/ggez/context/trait.HasMut.html)
pub trait HasMut<T>
{
    fn retrive_mut(&mut self) -> &mut T;
}
impl<T> HasMut<T> for T
{
    fn retrive_mut(&mut self) -> &mut T { self }
}

//pub trait Has<T> : HasRef<T> + HasMut<T>{}
//impl<S,T> Has<T> for S where S:HasRef<T> +HasMut<T>{}

/*
pub trait HasReadGuard<'a,T>
{
    fn retrive_guard_ref(&'a self) -> T;
}
impl<'a,T> HasReadGuard<'a,&'a T> for &'a T  { fn retrive_guard_ref(&'a self) -> &'a T { self } }
*/
