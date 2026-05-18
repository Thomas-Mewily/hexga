use super::*;

#[derive(Clone, Copy)]
pub struct Pen;

impl SingletonEmptyStruct for Pen
{
    fn is_init() -> bool { GRAPHICS.try_get_mut().is_ok() }
}
