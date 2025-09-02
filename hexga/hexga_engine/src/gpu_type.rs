use hexga::graphics::color::ColorRgbaOf;

use crate::*;

#[allow(non_camel_case_types)]
pub type Gfloat = f32;

pub type GVector<const N:usize> = Vector<Gfloat,N>;
pub type GVec1 = Vector1<Gfloat>;
pub type GVec2 = Vector2<Gfloat>;
pub type GVec3 = Vector3<Gfloat>;
pub type GVec4 = Vector4<Gfloat>;

pub const fn gvec1(x: float) -> GVec1 { GVec1::new(x) }
pub const fn gvec2(x: float, y: float) -> GVec2 { GVec2::new(x, y) }
pub const fn gvec3(x: float, y: float, z: float) -> GVec3 { GVec3::new(x, y, z) }
pub const fn gvec4(x: float, y: float, z: float, w: float) -> GVec4 { GVec4::new(x, y, z, w) }

pub type GRectangle<const N:usize> = Rectangle<Gfloat,N>;
pub type GRect1 = GRectangle<1>;
pub type GRect2 = GRectangle<2>;
pub type GRect3 = GRectangle<3>;
pub type GRect4 = GRectangle<4>;

pub type GColor = GColorRgba;
pub type GColorRgba = ColorRgbaOf<Gfloat>;
pub type GColorHsla = hexga::graphics::color::ColorHslaOf<Gfloat>;

pub mod prelude
{
    pub use super::*;
}