//use copypasta::{ClipboardContext, ClipboardProvider};
use serde::de;

use super::*;

/// Similar to Into::into(), but we can't use it because both type are declared in different crate
pub(crate) trait ExternLibConvert<Output>
{
    fn convert(self) -> Output;
}

pub(crate) type WinitPhysicialSize<T = i32> = winit::dpi::PhysicalSize<T>;
pub(crate) type WinitPhysicialPos<T = i32> = winit::dpi::PhysicalPosition<T>;

pub(crate) type WinitLogicalSize<T = i32> = winit::dpi::LogicalSize<T>;
pub(crate) type WinitLogicalPos<T = i32> = winit::dpi::LogicalPosition<T>;

impl<T> ExternLibConvert<Vec2> for WinitLogicalSize<T>
where
    T: ToFloat<Output = float>,
{
    fn convert(self) -> Vec2 { vec2(self.width.to_float(), self.height.to_float()) }
}
impl<T> ExternLibConvert<Vec2i> for WinitLogicalSize<T>
where
    T: ToInt<Output = int>,
{
    fn convert(self) -> Vec2i { vec2i(self.width.to_int(), self.height.to_int()) }
}

impl<T> ExternLibConvert<Vec2> for WinitPhysicialSize<T>
where
    T: ToFloat<Output = float>,
{
    fn convert(self) -> Vec2 { vec2(self.width.to_float(), self.height.to_float()) }
}
impl<T> ExternLibConvert<Vec2i> for WinitPhysicialSize<T>
where
    T: ToInt<Output = int>,
{
    fn convert(self) -> Vec2i { vec2i(self.width.to_int(), self.height.to_int()) }
}

impl<T> ExternLibConvert<Vec2> for WinitPhysicialPos<T>
where
    T: ToFloat<Output = float>,
{
    fn convert(self) -> Vec2 { vec2(self.x.to_float(), self.y.to_float()) }
}
impl<T> ExternLibConvert<Vec2i> for WinitPhysicialPos<T>
where
    T: ToInt<Output = int>,
{
    fn convert(self) -> Vec2i { vec2i(self.x.to_int(), self.y.to_int()) }
}

impl ExternLibConvert<WinitPhysicialPos> for Vec2i
{
    fn convert(self) -> WinitPhysicialPos { WinitPhysicialPos::new(self.x as i32, self.y as i32) }
}

impl ExternLibConvert<WinitPhysicialSize> for Vec2i
{
    fn convert(self) -> WinitPhysicialSize { WinitPhysicialSize::new(self.x as i32, self.y as i32) }
}

impl ExternLibConvert<ButtonState> for winit::event::ElementState
{
    fn convert(self) -> ButtonState
    { 
        match self
        {
            WinitEvent::ElementState::Pressed => ButtonState::Down,
            WinitEvent::ElementState::Released => ButtonState::Up,
        } 
    }
}


impl ExternLibConvert<Vec2> for winit::event::MouseScrollDelta
{
    fn convert(self) -> Vec2
    { 
        match self
        {
            WinitEvent::MouseScrollDelta::LineDelta(x, y) => vec2(x as _, y as _),
            WinitEvent::MouseScrollDelta::PixelDelta(p) => p.convert(),
        }
    }
}

impl ExternLibConvert<TouchPhase> for winit::event::TouchPhase
{
    fn convert(self) -> TouchPhase
    { 
        match self
        {
            WinitEvent::TouchPhase::Started => TouchPhase::Begin,
            WinitEvent::TouchPhase::Moved => TouchPhase::Move,
            WinitEvent::TouchPhase::Ended => TouchPhase::End,
            WinitEvent::TouchPhase::Cancelled => TouchPhase::Cancel,
        }
    }
}


impl ExternLibConvert<MouseButton> for winit::event::MouseButton
{
    fn convert(self) -> MouseButton
    { 
        match self
        {
            WinitEvent::MouseButton::Left => MouseButton::Left,
            WinitEvent::MouseButton::Right => MouseButton::Right,
            WinitEvent::MouseButton::Middle => MouseButton::Middle,
            WinitEvent::MouseButton::Back => MouseButton::Back,
            WinitEvent::MouseButton::Forward => MouseButton::Forward,
            WinitEvent::MouseButton::Other(v) => MouseButton::Other(v),
        }
    }
}

impl ExternLibConvert<Touch> for winit::event::Touch
{
    fn convert(self) -> Touch
    { 
        Touch { phase: self.phase.convert(), pos: self.location.convert(), id: self.id }
    }
}

impl ExternLibConvert<AppTheme> for winit::window::Theme
{
    fn convert(self) -> AppTheme
    {
        match self
        {
            winit::window::Theme::Light => AppTheme::Light,
            winit::window::Theme::Dark => AppTheme::Dark,
        } 
    }
}

/*
impl<T> ExternLibConvert<wgpu::Color> for T
where
    T: IColor,
    f64: CastRangeFrom<<T as IColor>::Component>,
{
    fn convert(self) -> wgpu::Color
    {
        let RgbaOf { r, g, b, a } = IColor::to_rgba_of::<f64>(self);
        wgpu::Color { r, g, b, a }
    }
}
*/

/*
impl WinitConvertWithDpi<Vec2> for winit::dpi::PhysicalSize<u32>
{
    type Output = Vec2;
    fn convert_with_dpi(self, dpi : float) -> Self::Output { self.to_logical(dpi as _).convert() }
}
impl WinitConvert for winit::dpi::LogicalPosition<f64>
{
    type Output = Vec2;
    fn convert(self) -> Self::Output { vec2(self.x as _, self.y as _) }
}

impl WinitConvertWithDpi for winit::dpi::PhysicalPosition<i32>
{
    type Output = Vec2;
    fn convert_with_dpi(self, dpi : float) -> Self::Output { self.to_logical(dpi as _).convert() }
}
impl WinitConvertWithDpi for winit::dpi::PhysicalPosition<f64>
{
    type Output = Vec2;
    fn convert_with_dpi(self, dpi : float) -> Self::Output { self.to_logical(dpi as _).convert() }
}
*/
