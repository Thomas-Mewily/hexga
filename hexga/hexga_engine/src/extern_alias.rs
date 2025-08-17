use crate::*;

pub(crate) type WinitWindowID = winit::window::WindowId;
pub(crate) type WinitWindow = winit::window::Window;

pub(crate) type WinitEventLoop<T> = winit::event_loop::EventLoop<T>;
pub(crate) type WinitEventLoopBuilder<T> = winit::event_loop::EventLoopBuilder<T>;
pub(crate) type WinitEventLoopProxy<T> = winit::event_loop::EventLoopProxy<T>;

#[cfg(target_arch = "wasm32")]
pub type WinitWindowPtrKind<T> = std::rc::Rc<T>;

#[cfg(not(target_arch = "wasm32"))]
pub type WinitWindowPtrKind<T> = std::sync::Arc<T>;