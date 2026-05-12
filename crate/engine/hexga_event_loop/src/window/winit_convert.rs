//use copypasta::{ClipboardContext, ClipboardProvider};
use serde::de;

use super::*;

pub(crate) trait ExternLibConvert<Output>
{
    fn convert(self) -> Output;
}

pub(crate) type WinitPhysicialSize<T=i32> = winit::dpi::PhysicalSize<T>;
pub(crate) type WinitPhysicialPos<T=i32> = winit::dpi::PhysicalPosition<T>;

pub(crate) type WinitLogicalSize<T=i32> = winit::dpi::LogicalSize<T>;
pub(crate) type WinitLogicalPos<T=i32> = winit::dpi::LogicalPosition<T>;


impl<T> ExternLibConvert<Vec2> for WinitLogicalSize<T>
where
    T: ToFloat<Output = float>,
{
    fn convert(self) -> Vec2 { vec2(self.width.to_float(), self.height.to_float()) }
}
impl<T> ExternLibConvert<Point2> for WinitLogicalSize<T>
where
    T: ToInt<Output = int>,
{
    fn convert(self) -> Point2 { point2(self.width.to_int(), self.height.to_int()) }
}

impl<T> ExternLibConvert<Vec2> for WinitPhysicialSize<T>
where
    T: ToFloat<Output = float>,
{
    fn convert(self) -> Vec2 { vec2(self.width.to_float(), self.height.to_float()) }
}
impl<T> ExternLibConvert<Point2> for WinitPhysicialSize<T>
where
    T: ToInt<Output = int>,
{
    fn convert(self) -> Point2 { point2(self.width.to_int(), self.height.to_int()) }
}

impl<T> ExternLibConvert<Vec2> for WinitPhysicialPos<T>
where
    T: ToFloat<Output = float>,
{
    fn convert(self) -> Vec2 { vec2(self.x.to_float(), self.y.to_float()) }
}
impl<T> ExternLibConvert<Point2> for WinitPhysicialPos<T>
where
    T: ToInt<Output = int>,
{
    fn convert(self) -> Point2 { point2(self.x.to_int(), self.y.to_int()) }
}

impl ExternLibConvert<WinitPhysicialPos> for Point2
{
    fn convert(self) -> WinitPhysicialPos { WinitPhysicialPos::new(self.x as i32, self.y as i32) }
}

impl ExternLibConvert<WinitPhysicialSize> for Point2
{
    fn convert(self) -> WinitPhysicialSize { WinitPhysicialSize::new(self.x as i32, self.y as i32) }
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
