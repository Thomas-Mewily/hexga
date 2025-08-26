use crate::*;

pub mod prelude
{
    
}

pub(crate) struct Ctx;
impl Deref for Ctx
{
    type Target=Context;
    fn deref(&self) -> &Self::Target { ctx() }
}
impl DerefMut for Ctx
{
    fn deref_mut(&mut self) -> &mut Self::Target { ctx_mut() }
}

static mut CONTEXT : Option<Context> = None;
#[allow(static_mut_refs)]
pub(crate) fn ctx_mut() -> &'static mut Context { unsafe { CONTEXT.as_mut().expect("Ctx not initialized") } }
#[allow(static_mut_refs)]
pub(crate) fn ctx() -> &'static Context { unsafe { CONTEXT.as_ref().expect("Ctx not initialized") } }

#[allow(static_mut_refs)]
pub(crate) fn ctx_mut_or_init() -> &'static mut Context { init_ctx_if_needed(); ctx_mut() }
#[allow(static_mut_refs)]
pub(crate) fn ctx_or_init() -> &'static Context { init_ctx_if_needed(); ctx() }

#[allow(static_mut_refs)]
pub(crate) fn init_ctx_if_needed()
{
    unsafe
    {
        if CONTEXT.is_none()
        {
            CONTEXT = Some(Context::new());
        }
        std::panic::set_hook(Box::new(|info| {
            CONTEXT = None;
            eprintln!("Panic occurred: {info}");
        }));
    }
}

pub(crate) fn reset_ctx()
{
    unsafe { CONTEXT = None };
}

#[derive(Debug, Default)]
pub(crate) struct Context
{
    //pub(crate) input: ContextInput,
    //pub(crate) pen: ContextPen,
    pub(crate) camera: CameraManager,
    pub(crate) windows: WindowManager,
    pub(crate) gfx: Graphics,
}

#[derive(Debug)]
pub(crate) struct Graphics
{
    pub(crate) instance : WgpuInstance,
}

impl Default for Graphics
{
    fn default() -> Self {
        Self
        {
            instance: wgpu::Instance::new(&wgpu::InstanceDescriptor
            {
                ..___()
            })
        }
    }
}



impl Context
{
    pub(crate) fn new() -> Self { ___() }

    pub(crate) fn resumed(&mut self)
    {

    }

    pub(crate) fn paused(&mut self)
    {
        
    }

    pub(crate) fn begin_update<UserEvent>(&mut self, gfx : &Graphics, event_loop: &WinitActiveEventLoop, proxy : &WinitEventLoopProxy<AppInternalEvent<UserEvent>>) where UserEvent: IUserEvent
    {
        self.windows.update(gfx, event_loop, proxy);
        self.camera.update(event_loop);

    }

    pub(crate) fn end_update<UserEvent>(&mut self, gfx : &Graphics, event_loop: &WinitActiveEventLoop, proxy : &WinitEventLoopProxy<AppInternalEvent<UserEvent>>) where UserEvent: IUserEvent
    {
        let _  = (gfx, event_loop, proxy);
    }
}


macro_rules! declare_context_singleton {
    ($struct_name:ident, $target_type:ty, $field:ident) =>
    {
        pub struct $struct_name;

        impl std::ops::Deref for $struct_name {
            type Target = $target_type;
            fn deref(&self) -> &Self::Target {
                &ctx().$field
            }
        }

        impl std::ops::DerefMut for $struct_name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut ctx_mut().$field
            }
        }
    };
}
pub(crate) use declare_context_singleton;


pub fn spawn_task<F, T>(future: F)
where
    F: Future<Output = T> + Send + 'static,
    T: Send + 'static
{
    #[cfg(target_arch = "wasm32")]
    wasm_bindgen_futures::spawn_local(future);

    #[cfg(not(target_arch = "wasm32"))]
    async_std::task::spawn(future);
}