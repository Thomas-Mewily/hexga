use super::*;


pub struct Ctx;
impl Deref for Ctx
{
    type Target=Context;
    fn deref(&self) -> &Self::Target { ctx() }
}
impl DerefMut for Ctx
{
    fn deref_mut(&mut self) -> &mut Self::Target { ctx_mut() }
}

#[derive(Debug)]
pub struct Graphics
{
    pub(crate) instance : wgpu::Instance,
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

#[derive(Debug, Default)]
pub struct Context
{
    pub input     : InputManager,
    pub clipboard : ClipboardManager,
    pub window    : WindowManager,
    pub graphics  : Graphics,
}
impl Context
{
    pub fn new() -> Self { ___() }
    /*
    pub fn input_ref(&self) -> &CtxInput { &self.input }
    pub fn input(&mut self) -> &mut CtxInput { &mut self.input }
    */
}

macro_rules! declare_context {
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
pub(crate) use declare_context;


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