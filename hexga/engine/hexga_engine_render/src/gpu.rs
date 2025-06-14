//! All the binding to the current lib
//!
//! Contains all the stateless function for rendering, getting the input...

#![allow(dead_code)]
use crate::*;

pub type GpuFloat = f32;

pub type GpuMat1 = Matrix1<GpuFloat>;
pub type GpuMat2 = Matrix2<GpuFloat>;
pub type GpuMat3 = Matrix3<GpuFloat>;
pub type GpuMat4 = Matrix4<GpuFloat>;


pub trait ToGpuFloat
{
    type Output;
    fn to_gpu_float(self) -> Self::Output;
}
impl<T> ToGpuFloat for T where T: ToF32
{
    type Output=T::Output;
    fn to_gpu_float(self) -> Self::Output { self.to_f32() }
}

pub type GpuVec1 = Vector1<GpuFloat>;
pub type GpuVec2 = Vector2<GpuFloat>;
pub type GpuVec3 = Vector3<GpuFloat>;
pub type GpuVec4 = Vector4<GpuFloat>;

pub const fn gpu_vec1(x : GpuFloat) -> GpuVec1 { GpuVec1::new(x) }
pub const fn gpu_vec2(x : GpuFloat, y : GpuFloat) -> GpuVec2 { GpuVec2::new(x, y) }
pub const fn gpu_vec3(x : GpuFloat, y : GpuFloat, z : GpuFloat) -> GpuVec3 { GpuVec3::new(x, y, z) }
pub const fn gpu_vec4(x : GpuFloat, y : GpuFloat, z : GpuFloat, w : GpuFloat) -> GpuVec4 { GpuVec4::new(x, y, z, w) }


pub type GpuRect2 = Rectangle2<GpuFloat>;

pub type GpuInt  = i32;

pub type GpuPoint1 = Vector1<GpuInt>;
pub type GpuPoint2 = Vector2<GpuInt>;
pub type GpuPoint3 = Vector3<GpuInt>;
pub type GpuPoint4 = Vector4<GpuInt>;

pub trait ToGpuInt
{
    type Output;
    fn to_gpu_int(self) -> Self::Output;
}
impl<T> ToGpuInt for T where T: ToI32
{
    type Output=T::Output;
    fn to_gpu_int(self) -> Self::Output { self.to_i32() }
}

pub const fn gpu_point1(x : GpuInt) -> GpuPoint1 { GpuPoint1::new(x) }
pub const fn gpu_point2(x : GpuInt, y : GpuInt) -> GpuPoint2 { GpuPoint2::new(x, y) }
pub const fn gpu_point3(x : GpuInt, y : GpuInt, z : GpuInt) -> GpuPoint3 { GpuPoint3::new(x, y, z) }
pub const fn gpu_point4(x : GpuInt, y : GpuInt, z : GpuInt, w : GpuInt) -> GpuPoint4 { GpuPoint4::new(x, y, z, w) }

pub type GpuUint = u32;

pub trait ToGpuUint
{
    type Output;
    fn to_gpu_uint(self) -> Self::Output;
}
impl<T> ToGpuUint for T where T: ToU32
{
    type Output=T::Output;
    fn to_gpu_uint(self) -> Self::Output { self.to_u32() }
}

pub trait ToGpuColor<T>
{
    fn to_gpu_color(self) -> GpuColor;
}
impl<S,T> ToGpuColor<T> for S where S : IColor<T>, T : Primitive
{
    fn to_gpu_color(self) -> GpuColor
    {
        self.to_rgba_of::<GpuFloat>()
    }
}
pub type GpuColor = ColorRgbaOf<GpuFloat>;






//pub type GpuImage<T> = Image<T, GpuImageParam>;
//pub type GpuImageRGBAByte = GpuImage<GpuColor>;
