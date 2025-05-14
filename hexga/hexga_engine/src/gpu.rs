//! All the binding to the current lib 
//! 
//! Contains all the stateless function for rendering, getting the input...

#![allow(dead_code)]
use crate::*;

pub type GpuFloat = f32;

pub type GpuVec2 = Vector2<GpuFloat>;
pub type GpuVec3 = Vector3<GpuFloat>;
pub type GpuVec4 = Vector4<GpuFloat>;

pub trait ToGpuColor
{
    fn to_gpu_color(self) -> GpuColor;
}
impl<T> ToGpuColor for T where T : IColor
{
    fn to_gpu_color(self) -> GpuColor 
    {
        let ColorRGBAByte { r, g, b, a } = self.to_color_byte();
        GpuColor::rgba(r as _, g as _, b as _, a as _)
    }
}

pub type GpuColor = ColorRGBAByte;

pub trait ToGpuFloat
{ 
    type Output;
    fn to_gpu_float(self) -> Self::Output; 
}
impl<T> ToGpuFloat for T where T : ToF32
{
    type Output=T::Output;
    fn to_gpu_float(self) -> Self::Output { self.to_f32() }
}