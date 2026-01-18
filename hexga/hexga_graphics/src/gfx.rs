/*
use hexga::cell::SingleThread;
use super::*;

const GFX_CTX : SingleThread<Option<GfxContext>> = SingleThread::new(None);

pub struct Gfx;

impl SingletonRead for Gfx
{
    type Target=GfxContext;
    type ReadGuard=&'static GfxContext;

    fn try_read() -> Option<Self::ReadGuard> {
        GFX_CTX.deref().as_ref()
    }
}

impl Deref for Gfx
{
    type Target=GfxContext;
    fn deref(&self) -> &Self::Target
    {
        unsafe {
            let opt: *const Option<GfxContext> = &*GFX_CTX;
            match &*opt {
                Some(ctx) => ctx,
                None => panic!("gfx not initialized"),
            }
        }
    }
}
impl DerefMut for Gfx
{
    fn deref_mut(&mut self) -> &mut Self::Target
    {
        unsafe {
            #[allow(const_item_mutation)]
            let opt: *mut Option<GfxContext> = &mut *GFX_CTX;
            match &mut *opt {
                Some(ctx) => ctx,
                None => panic!("gfx not initialized"),
            }
        }
    }
}

pub struct GfxContext
{
    pub(crate) camera_buffer: wgpu::Buffer,
    pub(crate) camera_bind_group: wgpu::BindGroup,
    pub(crate) texture_bind_group: wgpu::BindGroupLayout,
}

impl Deref for GfxContext
{
    type Target=gpu::Gpu;
    fn deref(&self) -> &Self::Target { &Gpu }
}
*/