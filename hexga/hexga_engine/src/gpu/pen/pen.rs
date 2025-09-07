use super::*;

pub mod prelude
{
    pub use super::Pen;
}

ctx_singleton!(
    Pen,
    Drawer,
    { Gpu::try_as_ref().map(|gpu| &gpu.draw) },
    { Gpu::try_as_mut().map(|gpu| &mut gpu.draw) }
);
