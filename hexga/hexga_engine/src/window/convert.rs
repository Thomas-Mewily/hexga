use std::collections::btree_map::Keys;

//use copypasta::{ClipboardContext, ClipboardProvider};
use serde::de;

use super::*;

pub(crate) trait WinitConvert<Output>
{
    fn convert(self) -> Output;
}

impl<T> WinitConvert<Vec2> for winit::dpi::LogicalSize<T> where T : ToFloat<Output = float>
{
    fn convert(self) -> Vec2 { vec2(self.width.to_float(), self.height.to_float()) }
}
impl<T> WinitConvert<Vec2i> for winit::dpi::LogicalSize<T> where T : ToInt<Output = int>
{
    fn convert(self) -> Vec2i { vec2i(self.width.to_int(), self.height.to_int()) }
}

impl<T> WinitConvert<Vec2> for winit::dpi::PhysicalSize<T> where T : ToFloat<Output = float>
{
    fn convert(self) -> Vec2 { vec2(self.width.to_float(), self.height.to_float()) }
}
impl<T> WinitConvert<Vec2i> for winit::dpi::PhysicalSize<T> where T : ToInt<Output = int>
{
    fn convert(self) -> Vec2i { vec2i(self.width.to_int(), self.height.to_int()) }
}

impl<T> WinitConvert<Vec2> for winit::dpi::PhysicalPosition<T> where T : ToFloat<Output = float>
{
    fn convert(self) -> Vec2 { vec2(self.x.to_float(), self.y.to_float()) }
}
impl<T> WinitConvert<Vec2i> for winit::dpi::PhysicalPosition<T> where T : ToInt<Output = int>
{
    fn convert(self) -> Vec2i { vec2i(self.x.to_int(), self.y.to_int()) }
}
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
