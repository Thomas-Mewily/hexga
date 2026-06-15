use crate::experimental::GPU;

use super::*;

#[derive(Clone, Copy)]
pub struct Gpu;

impl SingletonEmptyStruct for Gpu
{
    fn is_init() -> bool { GPU.try_get().is_ok() }
}

// Maybe to much
impl Deref for Gpu
{
    type Target = GpuContext;
    #[track_caller]
    #[inline]
    fn deref(&self) -> &Self::Target { GPU.get() }
}
