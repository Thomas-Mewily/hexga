use crate::*;


pub(crate) type WgpuInstance = wgpu::Instance;
pub(crate) type WgpuSurface = wgpu::Surface<'static>;


pub(crate) type WinitWindowID = winit::window::WindowId;
pub(crate) type WinitWindow = winit::window::Window;

pub(crate) type WinitEventLoop<T> = winit::event_loop::EventLoop<T>;
pub(crate) type WinitEventLoopBuilder<T> = winit::event_loop::EventLoopBuilder<T>;
pub(crate) type WinitEventLoopProxy<T> = winit::event_loop::EventLoopProxy<T>;
pub(crate) type WinitActiveEventLoop = winit::event_loop::ActiveEventLoop;
pub(crate) use winit::application::ApplicationHandler as WinitApp;
pub(crate) type EventLoopProxy<U> = WinitEventLoopProxy<AppInternalEvent<U>>;

pub(crate) type WinitPhysicalPosition<P> = winit::dpi::PhysicalPosition<P>;
pub(crate) type WinitLogicalPosition<P> = winit::dpi::LogicalPosition<P>;
pub(crate) type WinitPosition = winit::dpi::Position;

pub(crate) type WinitPhysicalSize<P> = winit::dpi::PhysicalSize<P>;
pub(crate) type WinitLogicalSize<P> = winit::dpi::LogicalSize<P>;
pub(crate) type WinitSize = winit::dpi::Size;

#[cfg(target_arch = "wasm32")]
pub type WinitWindowPtrKind<T> = std::rc::Rc<T>;

#[cfg(not(target_arch = "wasm32"))]
pub type WinitWindowPtrKind<T> = std::sync::Arc<T>;

pub trait WinitConvert<Output>
{
    fn convert(self) -> Output;
}

pub trait WinitConvertPoint2 : WinitConvert<Point2> + Sized
{
    fn convert_point2(self) -> Point2 { self.convert() }
}
impl<T> WinitConvertPoint2 for T where T: WinitConvert<Point2> {}

pub trait WinitConvertVec2 : WinitConvert<Vec2> + Sized
{
    fn convert_vec2(self) -> Vec2 { self.convert() }
}
impl<T> WinitConvertVec2 for T where T: WinitConvert<Vec2> {}


impl<T> WinitConvert<Vec2> for WinitLogicalSize<T> where T : ToFloat<Output = float>
{
    fn convert(self) -> Vec2 { vec2(self.width.to_float(), self.height.to_float()) }
}
impl<T> WinitConvert<Point2> for WinitLogicalSize<T> where T : ToInt<Output = int>
{
    fn convert(self) -> Point2 { point2(self.width.to_int(), self.height.to_int()) }
}

impl<T> WinitConvert<Vec2> for WinitPhysicalSize<T> where T : ToFloat<Output = float>
{
    fn convert(self) -> Vec2 { vec2(self.width.to_float(), self.height.to_float()) }
}
impl<T> WinitConvert<Point2> for WinitPhysicalSize<T> where T : ToInt<Output = int>
{
    fn convert(self) -> Point2 { point2(self.width.to_int(), self.height.to_int()) }
}

impl<T> WinitConvert<Vec2> for WinitPhysicalPosition<T> where T : ToFloat<Output = float>
{
    fn convert(self) -> Vec2 { vec2(self.x.to_float(), self.y.to_float()) }
}
impl<T> WinitConvert<Point2> for WinitPhysicalPosition<T> where T : ToInt<Output = int>
{
    fn convert(self) -> Point2 { point2(self.x.to_int(), self.y.to_int()) }
}