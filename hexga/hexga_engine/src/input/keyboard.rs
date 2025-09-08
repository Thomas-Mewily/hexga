use super::*;

pub mod prelude
{
    
}

pub struct Keyboard<T=Time> where T:Copy+Default
{
    keys : HashMap<KeyCode, InputValue<T>>,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct InputValue<T>
{
    value: T,
    consumed: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct InputDelta<I,T> where I:Copy, T:Copy+Default
{
    cur : I,
    old : I,
    time : T,
}


