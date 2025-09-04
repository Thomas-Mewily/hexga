use hexga::graphics::color::ColorRgbaOf;
use super::*;

#[allow(non_camel_case_types)]
pub type Gpufloat = f32;

pub type GpuVector<const N:usize> = Vector<Gpufloat,N>;
pub type GpuVec1 = Vector1<Gpufloat>;
pub type GpuVec2 = Vector2<Gpufloat>;
pub type GpuVec3 = Vector3<Gpufloat>;
pub type GpuVec4 = Vector4<Gpufloat>;

pub const fn gpu_vec1(x: float) -> GpuVec1 { GpuVec1::new(x) }
pub const fn gpu_vec2(x: float, y: float) -> GpuVec2 { GpuVec2::new(x, y) }
pub const fn gpu_vec3(x: float, y: float, z: float) -> GpuVec3 { GpuVec3::new(x, y, z) }
pub const fn gpu_vec4(x: float, y: float, z: float, w: float) -> GpuVec4 { GpuVec4::new(x, y, z, w) }

pub type GpuRectangle<const N:usize> = Rectangle<Gpufloat,N>;
pub type GpuRect1 = GpuRectangle<1>;
pub type GpuRect2 = GpuRectangle<2>;
pub type GpuRect3 = GpuRectangle<3>;
pub type GpuRect4 = GpuRectangle<4>;

pub type GpuColor = GpuColorRgba;
pub type GpuColorRgba = ColorRgbaOf<Gpufloat>;
pub type GpuColorHsla = hexga::graphics::color::ColorHslaOf<Gpufloat>;

pub mod prelude
{
    pub use super::
    {
        Gpufloat,
        GpuVector,GpuVec1,GpuVec2,GpuVec3,GpuVec4,
        gpu_vec1,gpu_vec2,gpu_vec3,gpu_vec4,
        GpuRectangle,GpuRect1,GpuRect2,GpuRect3,GpuRect4,
        GpuColor,GpuColorRgba,GpuColorHsla
    };
}