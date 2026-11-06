use super::*;

#[allow(non_camel_case_types)]
/// Gpu Float
pub type Gfloat = f32;

pub type GpuVector<const N: usize> = Vector<Gfloat, N>;
pub type GpuVec1 = Vector1<Gfloat>;
pub type GpuVec2 = Vector2<Gfloat>;
pub type GpuVec3 = Vector3<Gfloat>;
pub type GpuVec4 = Vector4<Gfloat>;

pub const fn gvec1(x: Gfloat) -> GpuVec1 { GpuVec1::new(x) }
pub const fn gvec2(x: Gfloat, y: Gfloat) -> GpuVec2 { GpuVec2::new(x, y) }
pub const fn gvec3(x: Gfloat, y: Gfloat, z: Gfloat) -> GpuVec3 { GpuVec3::new(x, y, z) }
pub const fn gvec4(x: Gfloat, y: Gfloat, z: Gfloat, w: Gfloat) -> GpuVec4 { GpuVec4::new(x, y, z, w) }

pub type GRectangle<const N: usize> = Rectangle<Gfloat, N>;
pub type GRect1 = GRectangle<1>;
pub type GRect2 = GRectangle<2>;
pub type GRect3 = GRectangle<3>;
pub type GRect4 = GRectangle<4>;

pub type GColor = GRgba;
pub type GRgba = RgbaOf<Gfloat>;
pub type GHsla = HslaOf<Gfloat>;

pub type GMatrix<const ROW: usize, const COL: usize> = Matrix<Gfloat, ROW, COL>;
pub type GSquareMatrix<const N: usize> = GMatrix<N, N>;
pub type GMat1 = GSquareMatrix<1>;
pub type GMat2 = GSquareMatrix<2>;
pub type GMat3 = GSquareMatrix<3>;
pub type GMat4 = GSquareMatrix<4>;
